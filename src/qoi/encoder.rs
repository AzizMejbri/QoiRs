use crate::qoi::types::{
    DynamicPixel, Pixel, PixelDiff, QoiHeader, QoiOpDiff, QoiOpIndex, QoiOpLuma, QoiOpRGB,
    QoiOpRGBA, QoiOpRun, Range,
};
use crate::qoi::types16::Pixel16;
use std::result::Result;
use std::slice::Iter;
use std::str::from_utf8;
use std::string::String;
use std::vec::Vec;

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
                    from_utf8(&bytestream[i..i + 2]).unwrap().parse().unwrap(),
                    from_utf8(&bytestream[i + 2..i + 4])
                        .unwrap()
                        .parse()
                        .unwrap(),
                    from_utf8(&bytestream[i + 4..i + 6])
                        .unwrap()
                        .parse()
                        .unwrap(),
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
            array_.copy_from_slice(
                &array
                    .iter()
                    .map(|p| p.as_pixel().unwrap())
                    .collect::<Vec<Pixel>>(),
            );
            encode_(&image_[..], &mut array_, width, height)
        }
        DynamicPixel::Pixel16(_) => {
            let image_ = image
                .iter()
                .map(|p| p.as_pixel16().unwrap())
                .collect::<Vec<Pixel16>>();

            let mut array_ = [Pixel16::default(); 64];
            array_.copy_from_slice(
                &array
                    .iter()
                    .map(|p| p.as_pixel16().unwrap())
                    .collect::<Vec<Pixel16>>(),
            );
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
    let output: &mut Vec<u8> = &mut Vec::with_capacity(100);
    output.append(&mut QoiHeader::new(width, height, 3, 0).as_bytes());
    let mut prev: &Pixel = &Pixel::new(0, 0, 0, 255);
    let empty0: Pixel = Pixel::new(1, 0, 0, 0);
    let emptyn: Pixel = Pixel::new(0, 0, 0, 0);
    let mut image_iterator: Iter<Pixel> = image.iter();
    while let Some(pixel) = image_iterator.next() {
        let values: (u8, u8, u8, u8) = pixel.extract();
        let diff: PixelDiff = PixelDiff::new(pixel, prev);
        let diff_diff: Option<PixelDiff> = PixelDiff::new_diff(pixel, prev);
        if pixel == prev {
            let mut run: u8 = 1;
            while let Some(value) = image_iterator.by_ref().next() {
                if value == pixel && run < 62 {
                    run += 1
                } else {
                    break;
                }
            }
            output.append(&mut QoiOpRun::new(run).as_bytes());
        } else if array[pixel.hash() as usize] == *pixel {
            output.append(&mut QoiOpIndex::new(pixel.hash()).as_bytes());
        } else if pixel.hash() == 0 && array[0] == empty0 || array[pixel.hash() as usize] == emptyn
        {
            array[pixel.hash() as usize] = *pixel;
            output.append(&mut QoiOpIndex::new(pixel.hash()).as_bytes());
        } else if diff.belongs(
            Range::new(PixelDiff::new2(-2, -2, -2, 0), PixelDiff::new2(1, 1, 1, 0)).unwrap(),
        ) {
            let rgba: (i8, i8, i8, i8) = diff.extract();
            output.append(&mut QoiOpDiff::new(rgba.0, rgba.1, rgba.2).as_bytes());
        } else if let Some(value) = diff_diff {
            if value.belongs(
                Range::new(
                    PixelDiff::new2(-32, -8, -8, 0),
                    PixelDiff::new2(31, 7, 7, 0),
                )
                .unwrap(),
            ) {
                let extracted = value.extract();
                output
                    .append(&mut QoiOpLuma::new(extracted.1, extracted.0, extracted.2).as_bytes());
            }
        } else if values.3 == prev.extract().3 {
            output.append(&mut QoiOpRGB::new(values.0, values.1, values.2).as_bytes());
        } else {
            output.append(&mut QoiOpRGBA::new(values.0, values.1, values.2, values.3).as_bytes());
        }

        prev = pixel;
    }
    output.append(&mut Pixel::new(0, 0, 0, 255).as_bytes());
    output.append(&mut array.map(|p| p.as_bytes()).concat());
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
