use crate::qoi::types16::Pixel16;

pub struct Range<T> {
    lower_limit: T,
    upper_limit: T,
}

impl<T> Range<T>
where
    T: PartialOrd,
{
    pub fn new(lower: T, upper: T) -> Result<Self, ()> {
        if lower < upper {
            Ok(Self {
                lower_limit: lower,
                upper_limit: upper,
            })
        } else {
            Err(())
        }
    }
}

#[derive(PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    #[inline(always)]
    pub fn extract(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
    #[inline(always)]
    pub fn hash(&self) -> u8 {
        ((self.r as u32 * 3 + self.g as u32 * 5 + self.b as u32 * 7 + self.a as u32 * 11) & 63)
            as u8
    }
    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        [self.r, self.g, self.b, self.a].to_vec()
    }
    #[inline(always)]
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.r);
        bytestream.push(self.g);
        bytestream.push(self.b);
        bytestream.push(self.a)
    }
}

#[derive(Clone, Copy)]
pub enum DynamicPixel {
    Pixel(Pixel),
    Pixel16(Pixel16),
}

impl DynamicPixel {
    #[inline(always)]
    pub fn as_pixel(&self) -> Result<Pixel, String> {
        match self {
            DynamicPixel::Pixel(value) => Ok(*value),
            DynamicPixel::Pixel16(_) => {
                Err("Cannot convert a 16-bit pixel to an 8-bit pixel".to_string())
            }
        }
    }
    #[inline(always)]
    pub fn as_pixel16(&self) -> Result<Pixel16, String> {
        match self {
            DynamicPixel::Pixel16(value) => Ok(*value),
            DynamicPixel::Pixel(_) => {
                Err("Cannot convert a 16-bit pixel to an 8-bit pixel (jk! we can but we won't, you have a conflict)".to_string())
            }
        }
    }
}

#[derive(PartialOrd, PartialEq)]
pub struct PixelDiff {
    r: i8,
    g: i8,
    b: i8,
    a: i8,
}

impl PixelDiff {
    #[inline(always)]
    pub fn new(p1: &Pixel, p2: &Pixel) -> Self {
        Self {
            r: (p1.r as i16 - p2.r as i16) as i8,
            g: (p1.g as i16 - p2.g as i16) as i8,
            b: (p1.b as i16 - p2.b as i16) as i8,
            a: (p1.a as i16 - p2.a as i16) as i8,
        }
    }
    #[inline(always)]
    pub fn new2(r: i8, g: i8, b: i8, a: i8) -> Self {
        Self { r, g, b, a }
    }
    #[inline(always)]
    pub fn new_diff(p1: &Pixel, p2: &Pixel) -> Self {
        Self {
            g: (p1.g as i16 - p2.g as i16) as i8,
            r: ((p1.r as i16 - p2.r as i16) - (p1.g as i16 - p2.g as i16)) as i8,
            b: ((p1.b as i16 - p2.b as i16) - (p1.g as i16 - p2.g as i16)) as i8,
            a: (p1.a as i16 - p2.a as i16) as i8,
        }
    }
    #[inline(always)]
    pub fn belongs(&self, range: Range<PixelDiff>) -> bool {
        //self <= &range.upper_limit && self >= &range.lower_limit
        self.r >= range.lower_limit.r
            && self.g >= range.lower_limit.g
            && self.b >= range.lower_limit.b
            && self.a >= range.lower_limit.a
            && self.r <= range.upper_limit.r
            && self.g <= range.upper_limit.g
            && self.b <= range.upper_limit.b
            && self.a <= range.upper_limit.a
    }
    #[inline(always)]
    pub fn extract(&self) -> (i8, i8, i8, i8) {
        (self.r, self.g, self.b, self.a)
    }
    #[inline(always)]
    pub fn is_alpha_zero(&self) -> bool {
        self.a == 0
    }
}

pub struct QoiHeader {
    magic_0: u8,
    magic_1: u8,
    magic_2: u8,
    magic_3: u8,
    width: u32,
    height: u32,
    chanels: u8,
    colorspace: u8,
}

impl QoiHeader {
    #[inline(always)]
    pub fn new(width: u32, height: u32, chanels: u8, colorspace: u8) -> Self {
        Self {
            magic_0: 113,
            magic_1: 111,
            magic_2: 105,
            magic_3: 102,
            width,
            height,
            chanels,
            colorspace,
        }
    }
    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(14);
        output.push(self.magic_0);
        output.push(self.magic_1);
        output.push(self.magic_2);
        output.push(self.magic_3);
        output.push((self.width >> 24) as u8);
        output.push((self.width >> 16) as u8);
        output.push((self.width >> 8) as u8);
        output.push((self.width) as u8);
        output.push((self.height >> 24) as u8);
        output.push((self.height >> 16) as u8);
        output.push((self.height >> 8) as u8);
        output.push((self.height) as u8);
        output.push(self.chanels);
        output.push(self.colorspace);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.magic_0);
        bytestream.push(self.magic_1);
        bytestream.push(self.magic_2);
        bytestream.push(self.magic_3);
        bytestream.push((self.width >> 24) as u8);
        bytestream.push((self.width >> 16) as u8);
        bytestream.push((self.width >> 8) as u8);
        bytestream.push((self.width) as u8);
        bytestream.push((self.height >> 24) as u8);
        bytestream.push((self.height >> 16) as u8);
        bytestream.push((self.height >> 8) as u8);
        bytestream.push((self.height) as u8);
        bytestream.push(self.chanels);
        bytestream.push(self.colorspace);
    }
}

pub struct QoiOpRGB {
    tag: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl QoiOpRGB {
    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            tag: 0b11111100,
            r,
            g,
            b,
        }
    }

    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(4);
        output.push(self.tag);
        output.push(self.r);
        output.push(self.g);
        output.push(self.b);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag);
        bytestream.push(self.r);
        bytestream.push(self.g);
        bytestream.push(self.b);
    }
}

pub struct QoiOpRGBA {
    tag: u8,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl QoiOpRGBA {
    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            tag: 0b11111111,
            r,
            g,
            b,
            a,
        }
    }
    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(5);
        output.push(self.tag);
        output.push(self.r);
        output.push(self.g);
        output.push(self.b);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag);
        bytestream.push(self.r);
        bytestream.push(self.g);
        bytestream.push(self.b);
    }
}

pub struct QoiOpIndex {
    tag_index: u8,
}

impl QoiOpIndex {
    #[inline(always)]
    pub fn new(index: u8) -> Self {
        assert!(index <= 64);
        Self {
            tag_index: index & 0b00111111,
        }
    }

    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
        output.push(self.tag_index);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag_index)
    }
}

pub struct QoiOpDiff {
    tag_dr_dg_db: u8,
}

impl QoiOpDiff {
    #[inline(always)]
    pub fn new(dr: i8, dg: i8, db: i8) -> Self {
        assert!((-2..=1).contains(&dr));
        assert!((-2..=1).contains(&db));
        assert!((-2..=1).contains(&dg));

        Self {
            tag_dr_dg_db: 0b01000000
                | ((((dr + 2) as u8) << 4) & 0b00110000)
                | ((((dg + 2) as u8) << 2) & 0b00001100)
                | (((db + 2) as u8) & 0b00000011),
        }
    }

    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
        output.push(self.tag_dr_dg_db);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag_dr_dg_db)
    }
}

pub struct QoiOpLuma {
    tag_diffg: u8,
    dr_dg_db_dg: u8,
}

impl QoiOpLuma {
    pub fn new(diff_green: i8, dr_dg: i8, db_dg: i8) -> Self {
        assert!((-32..=31).contains(&diff_green));
        assert!((-8..=7).contains(&dr_dg));
        assert!((-8..=7).contains(&db_dg));
        Self {
            tag_diffg: 0b10000000 | ((diff_green + 32) as u8 & 0b00111111),
            dr_dg_db_dg: (((((dr_dg + 8) as u8) << 4) & 0b11110000)
                | ((db_dg + 8) as u8 & 0b00001111)),
        }
    }
    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(2);
        output.push(self.tag_diffg);
        output.push(self.dr_dg_db_dg);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag_diffg);
        bytestream.push(self.dr_dg_db_dg)
    }
}

pub struct QoiOpRun {
    tag_run: u8,
}

impl QoiOpRun {
    #[inline(always)]
    pub fn new(run: u8) -> Self {
        assert!((1..=62).contains(&run));
        Self {
            tag_run: 0b11000000 | ((run - 1) & 0b00111111),
        }
    }
    // PERFORMANCE: so many unnecessary memory allocations to vectors for every single QoiOp
    //              Dont use unless absolutely neccessary
    #[deprecated]
    pub fn as_bytes(&self) -> Vec<u8> {
        let output: &mut Vec<u8> = &mut Vec::with_capacity(1);
        output.push(self.tag_run);
        output.to_vec()
    }
    pub fn append_self(&self, bytestream: &mut Vec<u8>) {
        bytestream.push(self.tag_run)
    }
}
