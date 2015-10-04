//! Represent colors in HSL and convert between HSL and RGB.
//!
//! # Examples
//!
//! ```rust
//! use hsl::HSL;
//!
//! let yellow = [255, 255, 0];
//! let yellow_hsl = HSL::from_rgb(&yellow);
//!
//! assert_eq!(yellow_hsl, HSL { h: 60_f64, s: 1_f64, l: 0.5_f64 });
//! ```

#![deny(missing_docs)]

#[cfg(test)] extern crate quickcheck;

/// Color represented in HSL
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct HSL {
    /// Hue in 0-360 degree
    pub h: f64,
    /// Saturation in 0...1 (percent)
    pub s: f64,
    /// Luminosity in 0...1 (percent)
    pub l: f64,
}

impl HSL {
    /// Convert RGB pixel value to HSL
    ///
    /// Expects RGB pixel to be a slice of three `u8`s representing the red, green and blue values.
    ///
    /// ```rust
    /// use hsl::HSL;
    /// let blue = HSL::from_rgb(&[0, 0, 255]);
    /// ```
    ///
    /// Algorithm from [go-color] by Brandon Thomson <bt@brandonthomson.com>. (Iternally converts
    /// the pixel to RGB before converting it to HSL.)
    ///
    /// [go-color]: https://github.com/bthomson/go-color
    #[cfg_attr(feature = "dev", allow(float_cmp))]
    pub fn from_rgb(rgb: &[u8]) -> HSL {
        use std::cmp::{max, min};

        let mut h: f64;
        let s: f64;
        let l: f64;

        let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

        let max = max(max(r, g), b);
        let min = min(min(r, g), b);

        // Normalized RGB: Divide everything by 255 to get percentages of colors.
        let (r, g, b) = (r as f64 / 255_f64,
                         g as f64 / 255_f64,
                         b as f64 / 255_f64);
        let (min, max) = (min as f64 / 255_f64, max as f64 / 255_f64);

        // Luminosity is the average of the max and min rgb color intensities.
        l = (max + min) / 2_f64;

        // Saturation
        let delta: f64 = max - min;
        if delta == 0_f64 {
    		// it's gray
            return HSL { h: 0_f64, s: 0_f64, l: l };
        }

        // it's not gray
        if l < 0.5_f64 {
            s = delta / (max + min);
        } else {
            s = delta / (2_f64 - max - min);
        }

        // Hue
        let r2 = (((max - r) / 6_f64) + (delta / 2_f64)) / delta;
        let g2 = (((max - g) / 6_f64) + (delta / 2_f64)) / delta;
        let b2 = (((max - b) / 6_f64) + (delta / 2_f64)) / delta;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1_f64 / 3_f64) + r2 - b2,
            _ => (2_f64 / 3_f64) + g2 - r2,
        };

        // Fix wraparounds
        if h < 0 as f64 {
            h += 1_f64;
        } else if h > 1 as f64 {
            h -= 1_f64;
        }

        // Hue is precise to milli-degrees, e.g. `74.52deg`.
        let h_degrees = (h * 360_f64 * 100_f64).round() / 100_f64;

        HSL { h: h_degrees, s: s, l: l }
    }

    /// Convert HSL color to RGB
    ///
    /// ```rust
    /// use hsl::HSL;
    ///
    /// let cyan = HSL { h: 180_f64, s: 1_f64, l: 0.5_f64 };
    /// assert_eq!(cyan.to_rgb(), (0, 255, 255));
    /// ```
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        if self.s == 0.0 {
            // Achromatic, i.e., grey.
            let l = percent_to_byte(self.l);
            return (l, l, l);
        }

        let h = self.h / 360.0; // treat this as 0..1 instead of degrees
        let s = self.s;
        let l = self.l;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = 2.0 * l - q;

        (percent_to_byte(hue_to_rgb(p, q, h + 1.0 / 3.0)),
         percent_to_byte(hue_to_rgb(p, q, h)),
         percent_to_byte(hue_to_rgb(p, q, h - 1.0 / 3.0)))
    }
}

fn percent_to_byte(percent: f64) -> u8 {
    (percent * 255.0).round() as u8
}

/// Convert Hue to RGB Ratio
///
/// From <https://github.com/jariz/vibrant.js/> by Jari Zwarts
fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    // Normalize
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen, quickcheck};
    use super::*;

    #[derive(Clone, Debug, Hash, PartialEq)]
    struct RGB {
        r: u8, g: u8, b: u8,
    }

    impl Arbitrary for RGB {
        fn arbitrary<G: Gen>(g: &mut G) -> RGB {
            RGB {
                r: g.gen(),
                g: g.gen(),
                b: g.gen(),
            }
        }
    }

    fn sloppy_rgb_compare(a: RGB, b: RGB) -> bool {
        const EPSILON: i32 = 0;
        let res = (a.r as i32 - b.r as i32 <= EPSILON) &&
                  (a.g as i32 - b.g as i32 <= EPSILON) &&
                  (a.b as i32 - b.b as i32 <= EPSILON);

        if !res {
            println!("in: {:?}, out: {:?}", a, b);
        }

        res
    }

    fn idemponent(input: RGB) -> bool {
        let RGB { r, g, b } = input;
        let (r_out, g_out, b_out) = HSL::from_rgb(&[r, g, b]).to_rgb();
        sloppy_rgb_compare(input, RGB { r: r_out, g: g_out, b: b_out })
    }

    #[test]
    fn quickcheck_rgb_to_hsl_and_back() {
        quickcheck(idemponent as fn(RGB) -> bool);
    }
}
