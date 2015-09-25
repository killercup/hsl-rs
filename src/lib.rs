/// Color represented in HSL
#[derive(Debug, PartialEq, PartialOrd, Default)]
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
        let (r, g, b) = (r as f64 / 255_f64, g as f64 / 255_f64, b as f64 / 255_f64);
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

        HSL { h: h * 360_f64, s: s, l: l }
    }

    /// Convert HSL color to RGB
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
mod test {
    macro_rules! test_rgb_to_hsl {
        ($name:ident, ($r:expr, $g:expr, $b:expr) <=> ($h:expr, $s:expr, $l:expr)) => {
            #[test]
            fn $name() {
                // Round gracefully to half a percent
                const EPSILON: f64 = 0.05;
                // Round gracefully to half a degree
                const EPSILON_DEGREE: f64 = 0.5;
                // Round gracefully for RGB bytes
                const EPSILON_RGB: i32 = 2;

                let rgb = [$r as u8, $g as u8, $b as u8];
                let hsl = HSL::from_rgb(&rgb);

                assert!(
                    ($h - hsl.h).abs() <= EPSILON_DEGREE,
                    "Converting {:?} to HSL: H differs too much. Expected {:?}, got {}. ({:?})",
                        rgb, $h, hsl.h, hsl
                );
                assert!(
                    ($s - hsl.s).abs() <= EPSILON,
                    "Converting {:?} to HSL: S differs too much. Expected {}, got {}. ({:?})",
                        rgb, $s, hsl.s, hsl
                );
                assert!(
                    ($l - hsl.l).abs() <= EPSILON,
                    "Converting {:?} to HSL: L differs too much. Expected {}, got {}. ({:?})",
                        rgb, $l, hsl.l, hsl
                );

                // ...and back!
                let hsl = HSL { h: $h, s: $s, l: $l };
                let rgb = hsl.to_rgb();
                let (r, g, b) = rgb;
                let expectation = ($r, $g, $b, 255);

                assert!(
                    ($r as i32 - r as i32).abs() <= EPSILON_RGB,
                    "Converting {:?} to RGB: R differs too much. Expected {:?}, got {:?}.",
                        hsl, expectation, rgb
                );
                assert!(
                    ($g as i32 - g as i32).abs() <= EPSILON_RGB,
                    "Converting {:?} to RGB: G differs too much. Expected {:?}, got {:?}.",
                        hsl, expectation, rgb
                );
                assert!(
                    ($b as i32 - b as i32).abs() <= EPSILON_RGB,
                    "Converting {:?} to RGB: B differs too much. Expected {:?}, got {:?}.",
                        hsl, expectation, rgb
                );
            }
        };
    }

    use super::HSL;

    // black
    test_rgb_to_hsl!(black, (0, 0, 0) <=> (0_f64, 0_f64, 0_f64) );

    // white
    test_rgb_to_hsl!(white, (255, 255, 255) <=> (0_f64, 0_f64, 1_f64) );

    // http://rgb.to/rgb/18,35,67
    test_rgb_to_hsl!(blue, (18, 35, 67) <=> (219_f64, 0.58_f64, 0.17_f64) );

    // http://rgb.to/hex/93c6cd
    test_rgb_to_hsl!(lightblue, (147, 198, 205) <=> (187_f64, 0.37_f64, 0.69_f64) );

    // http://rgb.to/hex/bada55
    test_rgb_to_hsl!(bada55, (186, 218, 85) <=> (74_f64, 0.64_f64, 0.59_f64) );

    // http://rgb.to/hex/ff0
    test_rgb_to_hsl!(yellow, (255, 255, 0) <=> (60_f64, 1_f64, 0.5_f64) );

    // http://rgb.to/rgb/198,250,172
    test_rgb_to_hsl!(lightgreen, (198, 250, 172) <=> (100_f64, 0.89_f64, 0.83_f64) );

    // http://rgb.to/hex/faadc7
    test_rgb_to_hsl!(lightpink, (250, 173, 199) <=> (340_f64, 0.89_f64, 0.83_f64) );
}
