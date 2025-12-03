use crate::qoi::types::{
    DynamicPixel, Pixel, PixelDiff, QoiHeader, QoiOpDiff, QoiOpIndex, QoiOpLuma, QoiOpRGB,
    QoiOpRGBA, QoiOpRun, Range,
};
use crate::qoi::types16::Pixel16;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

#[inline(always)]
fn read_number(bytestream: &[u8], start: &mut usize) -> u32 {
    while bytestream[*start] == b' ' || bytestream[*start] == b'\n' {
        *start += 1;
    }
    let mut end = *start;
    while bytestream[end] != b' ' && bytestream[end] != b'\n' {
        end += 1;
    }
    let num = std::str::from_utf8(&bytestream[*start..end])
        .unwrap()
        .parse::<u32>()
        .unwrap();
    *start = end;
    num
}
// NOTE: this is specific to the ppm (P6 specification) file format, other formats required
//       different implementations
pub fn bytestream_to_pixelstream(bytestream: &[u8]) -> (Vec<DynamicPixel>, u32, u32, u32) {
    if bytestream.starts_with("P6".as_bytes()) {
        let mut i = 2;
        let width = read_number(bytestream, &mut i);
        let height = read_number(bytestream, &mut i);
        let max_col_val = read_number(bytestream, &mut i);
        i += 1;
        let mut image: Vec<DynamicPixel> = Vec::with_capacity((width * height) as usize);
        if max_col_val <= 256 {
            while i < (width * height * 3) as usize {
                image.push(DynamicPixel::Pixel(Pixel::new(
                    bytestream[i],
                    bytestream[i + 1],
                    bytestream[i + 2],
                    0,
                )));
                i += 3;
            }
            return (image, width, height, max_col_val);
        } else {
            while i < (width * height * 6) as usize {
                image.push(DynamicPixel::Pixel16(Pixel16::new(
                    u16::from_be_bytes([bytestream[i], bytestream[i + 1]]),
                    u16::from_be_bytes([bytestream[i + 2], bytestream[i + 3]]),
                    u16::from_be_bytes([bytestream[i + 4], bytestream[i + 6]]),
                    0,
                )));
                i += 6;
            }
            return (image, width, height, max_col_val);
        }
    }
    //NOTE: Add other file formats here
    (Vec::new(), 0, 0, 0)
}

pub fn encode(
    image: &[DynamicPixel],
    array: &mut [DynamicPixel; 64],
    width: u32,
    height: u32,
    max_col_val: u32,
) -> Result<Vec<u8>, String> {
    if image.is_empty() {
        return Err("The image parsing deduced the image doesn't abide by our logic".to_string());
    }
    match image[0] {
        DynamicPixel::Pixel(_) => {
            let image_ = image
                .iter()
                .map(|p| p.as_pixel().unwrap())
                .collect::<Vec<Pixel>>();

            let mut array_ = [Pixel::default(); 64];
            for (i, dp) in array.iter().enumerate().take(64) {
                array_[i] = dp.as_pixel().expect("expected Pixel variant in array");
            }
            encode_(&image_[..], &mut array_, width, height)
        }
        DynamicPixel::Pixel16(_) => {
            let image_ = image
                .iter()
                .map(|p| p.as_pixel16().unwrap())
                .collect::<Vec<Pixel16>>();

            let mut array_ = [Pixel16::default(); 64];
            for (i, dp) in array.iter().enumerate().take(64) {
                array_[i] = dp.as_pixel16().expect("expected Pixel variant in array");
            }
            encode_16(&image_[..], &array_, width, height, max_col_val)
        }
    }
}

pub fn encode_(
    image: &[Pixel],
    array: &mut [Pixel; 64],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    let output: &mut Vec<u8> = &mut Vec::with_capacity(22usize + (width * height * 5) as usize);
    QoiHeader::new(width, height, 3, 0).append_self(output);
    let mut prev: &Pixel = &Pixel::new(0, 0, 0, 255);
    let mut i: usize = 0;
    let n = image.len();
    while (0..n).contains(&i) {
        let pixel = &image[i];
        if pixel == prev {
            let mut run: u8 = 1;
            while (i + run as usize) < n && (&image[i + run as usize] == pixel) && (run < 62) {
                run += 1;
            }
            i += run as usize;
            QoiOpRun::new(run).append_self(output);

            prev = pixel;
            continue;
        }
        let h = pixel.hash();
        if array[h as usize] == *pixel {
            QoiOpIndex::new(h).append_self(output);
            prev = pixel;
            i += 1;
            continue;
        }
        array[h as usize] = *pixel;
        let diff: PixelDiff = PixelDiff::new(pixel, prev);
        if diff.belongs(
            Range::new(PixelDiff::new2(-2, -2, -2, 0), PixelDiff::new2(1, 1, 1, 0)).unwrap(),
        ) {
            let rgba: (i8, i8, i8, i8) = diff.extract();
            QoiOpDiff::new(rgba.0, rgba.1, rgba.2).append_self(output);
            prev = pixel;
            i += 1;
            continue;
        }
        let diff_diff: PixelDiff = PixelDiff::new_diff(pixel, prev);
        if diff_diff.belongs(
            Range::new(
                PixelDiff::new2(-8, -32, -8, 0),
                PixelDiff::new2(7, 31, 7, 0),
            )
            .unwrap(),
        ) {
            let extracted = diff_diff.extract();
            QoiOpLuma::new(extracted.1, extracted.0, extracted.2).append_self(output);
            prev = pixel;
            i += 1;
            continue;
        }
        let values: (u8, u8, u8, u8) = pixel.extract();
        if diff_diff.is_alpha_zero() {
            QoiOpRGB::new(values.0, values.1, values.2).append_self(output);
            prev = pixel;
            i += 1;
            continue;
        }
        QoiOpRGBA::new(values.0, values.1, values.2, values.3).append_self(output);

        prev = pixel;
        i += 1;
    }
    output.extend_from_slice(&[0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8]);
    Ok(output.to_vec())
}

pub fn encode_16(
    image: &[Pixel16],
    mut array: &[Pixel16; 64],
    width: u32,
    height: u32,
    max_col_val: u32,
) -> Result<Vec<u8>, String> {
    todo!()
}
