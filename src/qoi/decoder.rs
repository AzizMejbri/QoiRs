use crate::qoi::types::Pixel;

// returns the pixel stream, the width, the height, the chanels and the colorspace respectively
pub fn decode(bytestream: &[u8], array: &mut [Pixel; 64]) -> (Vec<Pixel>, u32, u32, u8, u8) {
    assert!(bytestream[0] == 0x71);
    assert!(bytestream[1] == 0x6F);
    assert!(bytestream[2] == 0x69);
    assert!(bytestream[3] == 0x66);
    let width = u32::from_be_bytes([bytestream[4], bytestream[5], bytestream[6], bytestream[7]]);
    let height = u32::from_be_bytes([bytestream[8], bytestream[9], bytestream[10], bytestream[11]]);
    let chanels = bytestream[12];
    let colorspace = bytestream[13];

    //PERFORMANCE: extremely slow loop, O(N) on a massive array of bytes, temporary solution
    let mut i = 14;
    while bytestream[i..i + 8] != [0, 0, 0, 0, 0, 0, 0, 1] {
        i += 1
    }
    let mut pixel_stream: Vec<Pixel> = Vec::with_capacity((i - 14) >> 2); // every pixel is 4 bytes long
    array[0] = Pixel::new(0, 0, 0, 1);
    array[1..64].copy_from_slice(&[Pixel::new(0, 0, 0, 0); 63]);

    let mut j = 14;
    let mut prev = Pixel::new(0, 0, 0, 255);
    while j < i {
        if bytestream[j] >> 6 == 0 {
            // it is a 1-bit chunk and the tag is 00, so its QoiOpIndex
            let h = bytestream[j] & 0b00111111;
            prev = array[h as usize];
            pixel_stream.push(prev);
            j += 1;
            continue;
        }
        if bytestream[j] >> 6 == 1 {
            // it is a 1-bit chunk and the tag is 01, so its QoiOpDiff
            let extracted_prev = prev.extract();
            let dr = (bytestream[j] >> 4) & 0b11;
            let dg = (bytestream[j] >> 2) & 0b11;
            let db = bytestream[j] & 0b11;
            prev = Pixel::new(
                (extracted_prev.0 as i32 + dr as i32 - 2i32) as u8,
                (extracted_prev.1 as i32 + dg as i32 - 2i32) as u8,
                (extracted_prev.2 as i32 + db as i32 - 2i32) as u8,
                extracted_prev.3,
            );
            let h = prev.hash() as usize;
            array[h] = prev;
            pixel_stream.push(prev);
            j += 1;
            continue;
        }
        if bytestream[j] >> 6 == 3 && !(62..=63).contains(&(bytestream[j] & 0b00111111)) {
            for _ in 0..(bytestream[j] & 0b00111111) + 1 {
                pixel_stream.push(prev)
            }
            j += 1;
            continue;
        }
        if bytestream[j] >> 6 == 2 {
            let diff_green = bytestream[j] & 0b00111111;
            let dr_dg = bytestream[j + 1] & 0b11110000;
            let db_dg = bytestream[j + 1] & 0b00001111;
            let extracted_prev = prev.extract();
            prev = Pixel::new(
                (dr_dg as i32 + extracted_prev.0 as i32 + diff_green as i32 - 40i32) as u8,
                (extracted_prev.1 as i32 + diff_green as i32 - 32i32) as u8,
                (db_dg as i32 + extracted_prev.2 as i32 + diff_green as i32 - 40i32) as u8,
                extracted_prev.3,
            );
            let h = prev.hash() as usize;
            array[h] = prev;
            pixel_stream.push(prev);
            j += 2;
            continue;
        }
        if bytestream[j] == 0xFE {
            let extracted_prev = prev.extract();
            prev = Pixel::new(
                bytestream[j + 1],
                bytestream[j + 2],
                bytestream[j + 3],
                extracted_prev.3,
            );
            let h = prev.hash() as usize;
            array[h] = prev;

            pixel_stream.push(prev);
            j += 4;
            continue;
        }
        if bytestream[j] == 0xFF {
            prev = Pixel::new(
                bytestream[j + 1],
                bytestream[j + 2],
                bytestream[j + 3],
                bytestream[j + 4],
            );
            let h = prev.hash() as usize;
            array[h] = prev;

            pixel_stream.push(prev);
            j += 5;
            continue;
        } else {
            eprintln!("Error! Non-aligned chunks");
        }
    }

    (pixel_stream, width, height, chanels, colorspace)
}

pub fn decode_to_p6_8_bit(bytestream: &[u8], array: &mut [Pixel; 64]) -> Vec<u8> {
    let decoded = decode(bytestream, array);
    let mut output: Vec<u8> = Vec::new();
    output.extend_from_slice(format!("P6\n{} {}\n255\n", decoded.1, decoded.2,).as_bytes());
    let mut extracted: (u8, u8, u8, u8);
    for pixel in decoded.0 {
        extracted = pixel.extract();
        output.extend_from_slice(&[extracted.0, extracted.1, extracted.2])
    }
    output
}
