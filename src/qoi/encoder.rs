use crate::qoi::types::{
    DynamicPixel, Pixel, Pixel16, PixelDiff, QoiOpDiff, QoiOpIndex, QoiOpLuma, QoiOpRGB, QoiOpRGBA,
    QoiOpRun, Range,
};
use std::result::Result;
use std::slice::Iter;
use std::str::from_utf8;
use std::string::String;
use std::vec::Vec;

// NOTE: this is specific to the ppm (P6 specification) file format, other formats required
//       different implementations
pub fn bytestream_to_pixelstream(bytestream: &[u8]) -> Vec<DynamicPixel> {
    if bytestream.starts_with("P6".as_bytes()) {
        assert!(bytestream[2] == 10 || bytestream[2] == 20);
        let mut i = 3;
        let mut j = 3;
        while bytestream[i] != 20 && bytestream[i] != 10 {
            i += 1
        }
        let width: u32 = from_utf8(&bytestream[j..i - 1])
            .expect("Error in the Encoding! The ppm file is not utf-8 encoded")
            .parse()
            .expect("Error in the Encoding! the ppm file is corrupted");
        i += 1;
        j = i;
        while bytestream[j] != 10 && bytestream[j] != 20 {
            j += 1
        }
        let height: u32 = from_utf8(&bytestream[i..j])
            .expect("Error in the Encoding! the ppm file is not utf-8 encoded")
            .parse()
            .expect("Error in the Encoding! the ppm file is corrupted");
        j += 1;
        i = j;
        while bytestream[i] != 10 && bytestream[i] != 20 {
            i += 1
        }
        let max_col_val :u16 = from_utf8(&bytestream[j..i])
            .expect("Error in the Encoding! the ppm file is not utf-8 encoded")
            .parse()
            .expect("Error in the Encoding! the ppm file is corrupted! (the maximum pixel color value is not a 16-bit unsigned integer)");
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
            return image;
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
        }
    }
    Vec::new()
    //NOTE: Add other file formats here
}

pub fn encode(image: &[DynamicPixel], array: &mut [DynamicPixel; 64]) -> Result<Vec<u8>, String> {
    match image[0] {
        DynamicPixel::Pixel(pixel) => encode_(image, array),
        DynamicPixel::Pixel16(pixel) => encode_16(image, array),
    }
}
pub fn encode_(image: &[Pixel], array: &mut [Pixel; 64]) -> Result<Vec<u8>, String> {
    let output: &mut Vec<u8> = &mut Vec::with_capacity(100);
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
                if value == pixel {
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
    Ok(output.to_vec())
}
