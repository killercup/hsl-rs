extern crate hsl;
use hsl::HSL;

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
