use crate::errors::ColorError;

pub struct Scheme {
    pub name: String,
    pub colors: Vec<Canonical>,
}

const BIT_SHIFT_RED: usize = 4 * 6;
const BIT_SHIFT_GREEN: usize = 4 * 4;
const BIT_SHIFT_BLUE: usize = 4 * 2;

pub type Canonical = RGBA<u8>;
pub type Packed = u32;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: Copy> Copy for RGBA<T> {}
impl<T: Copy> Copy for RGB<T> {}

impl Canonical {
    pub fn parse_from_hex(input: &str) -> Result<Self, ColorError> {
        let hex_str = input.trim_start_matches('#');
        if hex_str.len() == 6 {
            let r = u8::from_str_radix(&hex_str[..2], 16)?;
            let g = u8::from_str_radix(&hex_str[2..4], 16)?;
            let b = u8::from_str_radix(&hex_str[4..6], 16)?;
            Ok(RGBA::new(r, g, b, u8::MAX))
        } else if hex_str.len() == 8 {
            let r = u8::from_str_radix(&hex_str[..2], 16)?;
            let g = u8::from_str_radix(&hex_str[2..4], 16)?;
            let b = u8::from_str_radix(&hex_str[4..6], 16)?;
            let a = u8::from_str_radix(&hex_str[6..8], 16)?;
            Ok(RGBA::new(r, g, b, a))
        } else {
            Err(ColorError::ParseHexError(format!(
                "String argument {} does not have the correct length of 6 or 8",
                input
            )))
        }
    }

    pub fn pack(&self) -> Packed {
        let r = (self.r as u32) << BIT_SHIFT_RED;
        let g = (self.g as u32) << BIT_SHIFT_GREEN;
        let b = (self.b as u32) << BIT_SHIFT_BLUE;
        let a = self.a as u32;
        r | g | b | a
    }

    pub fn unpack(rgba: Packed) -> Canonical {
        // shifts the packed u32 by X bits to the right, so the desired
        // color component is represented by the 8 least significant bits.
        // then by casting it to an u8, it cuts off all but those 8 bits.
        // explicit bitmasking is not necessary
        let r = (rgba >> BIT_SHIFT_RED) as u8;
        let g = (rgba >> BIT_SHIFT_GREEN) as u8;
        let b = (rgba >> BIT_SHIFT_BLUE) as u8;
        let a = rgba as u8;
        RGBA { r, g, b, a }
    }
}

impl<T> RGB<T> {
    pub fn new(red: T, green: T, blue: T) -> RGB<T> {
        RGB {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn into_rgba(self, alpha: T) -> RGBA<T> {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    pub fn map<U, F>(self, f: F) -> RGB<U>
    where
        F: Fn(T) -> U,
    {
        RGB {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
        }
    }
}

impl<T> From<RGBA<T>> for RGB<T> {
    fn from(c: RGBA<T>) -> Self {
        RGB {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

impl<T> RGBA<T> {
    pub fn new(red: T, green: T, blue: T, alpha: T) -> RGBA<T> {
        RGBA {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub fn into_rgba(self, alpha: T) -> RGBA<T> {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    pub fn map<U, F>(self, f: F) -> RGBA<U>
    where
        F: Fn(T) -> U,
    {
        RGBA {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: f(self.a),
        }
    }
}

impl From<RGB<u8>> for RGBA<u8> {
    fn from(rgb: RGB<u8>) -> Self {
        RGBA {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: u8::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_roughly_equal(a: f32, b: f32) {
        let diff = f32::abs(a - b);
        assert!(diff < 0.001, "Diff {} was not smaller than 0.001", diff)
    }

    #[test]
    fn test_rgb_into_rgba() {
        let rgb = RGB::new(255u8, 0, 0);

        assert_eq!(rgb.into_rgba(128), RGBA::new(255u8, 0, 0, 128))
    }

    #[test]
    fn test_rgb_map() {
        let rgb = RGB::new(255u8, 12, 0);

        let mapped_rgb = rgb.map(|val| val as f32 / 255.0 * 100.0);
        assert_roughly_equal(mapped_rgb.r, 100.0);
        assert_roughly_equal(mapped_rgb.g, 12.0 / 255.0 * 100.0);
        assert_roughly_equal(mapped_rgb.b, 0.0);
    }

    #[test]
    fn test_rgba_into_rgb() {
        let rgba = RGBA::new(128u8, 0, 10, 128);

        assert_eq!(
            RGB::from(rgba),
            RGB {
                r: 128u8,
                g: 0,
                b: 10,
            }
        );
    }

    #[test]
    fn test_rgba_map() {
        let rgba = RGBA::new(255u8, 12, 0, 128);

        let mapped_rgba = rgba.map(|val| val as f32 / 255.0 * 100.0);
        assert_roughly_equal(mapped_rgba.r, 100.0);
        assert_roughly_equal(mapped_rgba.g, 12.0 / 255.0 * 100.0);
        assert_roughly_equal(mapped_rgba.b, 0.0);
        assert_roughly_equal(mapped_rgba.a, 128.0 / 255.0 * 100.0);
    }

    #[test]
    fn test_canonical_parse_from_hex() {
        let ok = Canonical::parse_from_hex("#00aa11").unwrap();
        let ok_with_alpha = Canonical::parse_from_hex("#ffffff00").unwrap();
        let too_short = Canonical::parse_from_hex("#12345").unwrap_err();
        let wrong_format = Canonical::parse_from_hex("#xz00??_k").unwrap_err();

        assert_eq!(
            ok,
            RGBA {
                r: 0,
                g: 170,
                b: 17,
                a: 255
            }
        );
        assert_eq!(
            ok_with_alpha,
            RGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 0
            }
        );
        assert!(matches!(too_short, ColorError::ParseHexError(_)));
        assert!(matches!(wrong_format, ColorError::ParseToIntError(_, _)));
    }

    #[test]
    fn test_canonical_pack() {
        // 80 80 00 FF = 2_155_872_511
        let canonical = Canonical::new(128, 128, 0, 255);
        let packed = canonical.pack();

        // let expected = u32::from_str_radix("808000ff", 16).unwrap();

        assert_eq!(2_155_872_511u32, packed);
    }

    #[test]
    fn test_canonical_unpack() {
        // AC AB AC AB = 2_896_932_011
        let packed = 2_896_932_011u32;
        let unpacked = Canonical::unpack(packed);

        assert_eq!(unpacked, Canonical::new(172, 171, 172, 171));
    }

    #[test]
    fn test_canonical_pack_unpack() {
        
        let canonical = Canonical::new(123, 234, 213, 132);
        let packed = canonical.pack();
        let unpacked = Canonical::unpack(packed);
        let packed_again = unpacked.pack();

        assert_eq!(canonical, unpacked);
        assert_eq!(packed, packed_again);
    }
}
