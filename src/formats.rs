use regex::{Match, Regex};

use crate::{color::Canonical, errors::ParseFormatError};

#[derive(Debug)]
pub enum ColorFormats {
    RGBu8,
    RGBf,
    Hex,
}

pub trait ColorFormat {
    fn matches(color_str: &str) -> bool;
    fn parse(color_str: &str) -> Result<Canonical, ParseFormatError>;
}

lazy_static! {
    static ref RGBA_F_REGEX: Regex = Regex::new(
        r"(?x)
    [rR][gG][bB][aA]?
    \(
        \s*(?P<r>[01]\.\d+)\s*,
        \s*(?P<g>[01]\.\d+)\s*,
        \s*(?P<b>[01]\.\d+)\s*
        (,
            \s*(?P<a>[01]\.\d+)
        \s*)?
    \)"
    )
    .unwrap();
    static ref RGBA_U8_REGEX: Regex = Regex::new(
        r"(?x)
    [rR][gG][bB][aA]?
    \(
        \s*(?P<r>[0-9]{1,3})\s*,
        \s*(?P<g>[0-9]{1,3})\s*,
        \s*(?P<b>[0-9]{1,3})\s*
        (,
            \s*(?P<a>[0-9]{1,3})
        \s*)?
    \)"
    )
    .unwrap();
    static ref HEX_REGEX: Regex = Regex::new(
        r"/(?x)
    \#
      (?P<r>[0-9a-fA-F]{2})
      (?P<g>[0-9a-fA-F]{2})
      (?P<b>[0-9a-fA-F]{2})
      (?P<a>[0-9a-fA-F]{2})?"
    )
    .unwrap();
}

pub struct RGBFloatFormat {}
pub struct RGBu8Format {}

impl ColorFormat for RGBFloatFormat {
    fn matches(color_str: &str) -> bool {
        RGBA_F_REGEX.is_match(color_str.trim())
    }

    fn parse(color_str: &str) -> Result<Canonical, ParseFormatError> {
        let caps = RGBA_F_REGEX.captures(color_str);
        let caps = match caps {
            Some(captures) => captures,
            None => return Err(ParseFormatError(ColorFormats::RGBf, color_str.into())),
        };
        let r = extract_float_in_range(caps.name("r"))?;
        let g = extract_float_in_range(caps.name("g"))?;
        let b = extract_float_in_range(caps.name("b"))?;
        let a = match caps.name("a") {
            opt @ Some(_) => extract_float_in_range(opt)?,
            None => 1.0,
        };
        Ok(Canonical::from_f(r, g, b, a))
    }
}

fn extract_float_in_range(match_opt: Option<Match>) -> Result<f32, ParseFormatError> {
    match match_opt {
        Some(mat) => {
            let f = mat.as_str().parse::<f32>()?;
            if 0.0 <= f && f <= 1.0 {
                Ok(f)
            } else {
                Err(ParseFormatError(
                    ColorFormats::RGBf,
                    format!("parsed float {} is not within valid range (0, 1)", f),
                ))
            }
        }
        None => Err(ParseFormatError(
            ColorFormats::RGBf,
            "required float color component is missing".into(),
        )),
    }
}

#[cfg(test)]
mod tests_rgb_float_format {
    use super::*;

    #[test]
    fn test_color_format_matches() {
        let ok_candidates = vec![
            "rgb(0.0, 0.0, 0.0)",         // - rgb variations
            "rgb(0.0, 0.0, 0.0, 0.0)",    // |
            "rgba(0.0, 0.0, 0.0)",        // |
            "rgba(0.0, 0.0, 0.0, 0.0)",   // |
            "RGBA(0.0, 0.0, 0.0)",        // - casing
            "RgBa(0.0, 0.0, 0.0)",        // |
            "rGb(0.0, 0.0, 0.0)",         // |
            " rgb( 0.5  ,  1.0 ,0.25 ) ", // - strange spacings
            "rgb(0.111111111111111111, 0.2, 0.12345, 0.696969)",
            "rgba(0.00, 0.0000, 0.00000, 0.0)",
            // invalid but matches
            "rgb(1.5, 0.91, 1.99999)", // - starts with `1` (but > 1)
        ];

        for cand in ok_candidates {
            assert!(RGBFloatFormat::matches(cand))
        }

        let ko_candidates = vec![
            "rgb(0, 0, 0)",
            "rgba(0, 0, 0)",
            "rgb(-1.0, 1.0, 0.0)",
            "rgba(1.0.0, 1.0, 0.1001)",
        ];

        for cand in ko_candidates {
            assert!(!RGBFloatFormat::matches(cand))
        }
    }

    #[test]
    fn test_color_format_parse() {
        assert_eq!(
            RGBFloatFormat::parse("rgb(0.0, 0.0, 0.0").unwrap(),
            Canonical::from_f(0.0, 0.0, 0.0, 1.0)
        );

        assert_eq!(
            RGBFloatFormat::parse("rgba(0.5, 0.123, 0.1010, 0.90)").unwrap(),
            Canonical::from_f(0.5, 0.123, 0.1010, 0.90)
        );
    }
}
