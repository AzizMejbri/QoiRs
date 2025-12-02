use crate::qoi::types::Range;

#[derive(PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Pixel16 {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
}
impl Pixel16 {
    pub fn new(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self { r, g, b, a }
    }
    pub fn extract(&self) -> (u16, u16, u16, u16) {
        (self.r, self.g, self.b, self.a)
    }
    pub fn hash(&self) -> u8 {
        ((self.r * 3 + self.g * 5 + self.b * 6 + self.a * 11) % 64) as u8
    }
}

#[derive(PartialOrd, PartialEq)]
pub struct PixelDiff16 {
    r: i8,
    g: i8,
    b: i8,
    a: i8,
}

impl PixelDiff16 {
    pub fn new(p1: &Pixel16, p2: &Pixel16) -> Self {
        Self {
            r: (p1.r - p2.r) as i8,
            g: (p1.g - p2.g) as i8,
            b: (p1.b - p2.b) as i8,
            a: (p1.a - p2.a) as i8,
        }
    }
    pub fn new2(r: i8, g: i8, b: i8, a: i8) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_diff(p1: &Pixel16, p2: &Pixel16) -> Option<Self> {
        if p1.a == p2.a {
            None
        } else {
            Some(Self {
                g: (p1.g - p2.g).try_into().expect(
                    "Error Converting in gd, PixelDiff::new_diff(&Pixel, &Pixel) -> PixelDiff",
                ),
                r: ((p1.r - p2.r) / (p1.g - p2.g)).try_into().expect(
                    "Error Convering in dr, PixelDiff::new_diff(&Pixel, &Pixel) -> PixelDiff",
                ),
                b: ((p1.b - p2.b) / (p1.g - p2.g)).try_into().expect(
                    "Error Convering in db, PixelDiff::new_diff(&Pixel, &Pixel) -> PixelDiff",
                ),
                a: 0,
            })
        }
    }
    pub fn belongs(&self, range: Range<PixelDiff16>) -> bool {
        // self.r >= range.lower_limit.r
        //     && self.g >= range.lower_limit.g
        //     && self.b >= range.lower_limit.b
        //     && self.a >= range.lower_limit.a
        //     && self.r <= range.upper_limit.r
        //     && self.g <= range.upper_limit.g
        //     && self.b <= range.upper_limit.b
        //     && self.a <= range.upper_limit.a
        todo!()
    }
    pub fn extract(&self) -> (i8, i8, i8, i8) {
        (self.r, self.g, self.b, self.a)
    }
}
//
// pub struct QoiHeader {
//     magic_0: u8,
//     magic_1: u8,
//     magic_2: u8,
//     magic_3: u8,
//     width: u32,
//     height: u32,
//     chanels: u8,
//     colorspace: u8,
// }
//
// impl QoiHeader {
//     fn new(width: u32, height: u32, chanels: u8, colorspace: u8) -> Self {
//         Self {
//             magic_0: 113,
//             magic_1: 111,
//             magic_2: 105,
//             magic_3: 102,
//             width,
//             height,
//             chanels,
//             colorspace,
//         }
//     }
//     fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(112);
//         output.push(self.magic_0);
//         output.push(self.magic_1);
//         output.push(self.magic_2);
//         output.push(self.magic_3);
//         output.push((self.width >> 24) as u8 & 0b11111111);
//         output.push((self.width >> 16) as u8 & 0b11111111);
//         output.push((self.width >> 8) as u8 & 0b11111111);
//         output.push((self.width) as u8 & 0b11111111);
//         output.push((self.height >> 24) as u8 & 0b11111111);
//         output.push((self.height >> 16) as u8 & 0b11111111);
//         output.push((self.height >> 8) as u8 & 0b11111111);
//         output.push((self.height) as u8 & 0b11111111);
//         output.push(self.chanels);
//         output.push(self.colorspace);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpRGB {
//     tag: u8,
//     r: u8,
//     g: u8,
//     b: u8,
// }
//
// impl QoiOpRGB {
//     pub fn new(r: u8, g: u8, b: u8) -> Self {
//         Self {
//             tag: 0b11111100,
//             r,
//             g,
//             b,
//         }
//     }
//
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(4);
//         output.push(self.tag);
//         output.push(self.r);
//         output.push(self.g);
//         output.push(self.b);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpRGBA {
//     tag: u8,
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// }
//
// impl QoiOpRGBA {
//     pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
//         Self {
//             tag: 0b11111111,
//             r,
//             g,
//             b,
//             a,
//         }
//     }
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(5);
//         output.push(self.tag);
//         output.push(self.r);
//         output.push(self.g);
//         output.push(self.b);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpIndex {
//     tag_index: u8,
// }
//
// impl QoiOpIndex {
//     pub fn new(index: u8) -> Self {
//         assert!(0 <= index && index <= 64);
//         Self {
//             tag_index: index & 0b00111111,
//         }
//     }
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
//         output.push(self.tag_index);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpDiff {
//     tad_dr_dg_db: u8,
// }
//
// impl QoiOpDiff {
//     pub fn new(dr: i8, dg: i8, db: i8) -> Self {
//         assert!(-2 <= dr && dr <= 1);
//         assert!(-2 <= db && db <= 1);
//         assert!(-2 <= dg && dg <= 1);
//
//         Self {
//             tad_dr_dg_db: 0b01000000
//                 | ((((dr + 2) as u8) << 4) & 0b00110000)
//                 | ((((dg + 2) as u8) << 2) & 0b00001100)
//                 | (((db + 2) as u8) & 0b00000011),
//         }
//     }
//
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
//         output.push(self.tad_dr_dg_db);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpLuma {
//     tag_diffg: u8,
//     dr_dg_db_dg: u8,
// }
//
// impl QoiOpLuma {
//     pub fn new(diff_green: i8, dr_dg: i8, db_dg: i8) -> Self {
//         assert!(-32 <= diff_green && diff_green <= 31);
//         assert!(-8 <= dr_dg && dr_dg <= 7);
//         assert!(-8 <= db_dg && db_dg <= 7);
//         Self {
//             tag_diffg: 0b10000000 | ((diff_green + 32) as u8 & 0b00111111),
//             dr_dg_db_dg: (((((dr_dg + 8) as u8) << 4) & 0b11110000)
//                 | ((db_dg + 8) as u8 & 0b00001111)),
//         }
//     }
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(2);
//         output.push(self.tag_diffg);
//         output.push(self.dr_dg_db_dg);
//         output.to_vec()
//     }
// }
//
// pub struct QoiOpRun {
//     tag_run: u8,
// }
//
// impl QoiOpRun {
//     pub fn new(run: u8) -> Self {
//         assert!(1 <= run && run <= 62);
//         Self {
//             tag_run: 0b11000000 | ((run - 1) & 0b00111111),
//         }
//     }
//     pub fn as_bytes(&self) -> Vec<u8> {
//         let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
//         output.push(self.tag_run);
//         output.to_vec()
//     }
// }
// TODO!: implement the Qoi for 16-bit valued images
