use std::cmp::min;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new<T: Into<f64>>(red: T, green: T, blue: T) -> Self {
        Self {
            red: red.into(),
            blue: blue.into(),
            green: green.into(),
        }
    }

    pub fn as_rgb(&self) -> (usize, usize, usize) {
        (
            min((self.red * 255f64).round() as usize, 255),
            min((self.green * 255f64).round() as usize, 255),
            min((self.blue * 255f64).round() as usize, 255),
        )
    }
}

impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            blue: self.blue - other.blue,
            green: self.green - other.green,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            red: self.red * scalar,
            blue: self.blue * scalar,
            green: self.green * scalar,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

#[cfg(test)]
mod test {
    extern crate approx;
    use super::*;
    use approx::assert_abs_diff_eq;

    fn compare(c1: Color, c2: Color) {
        assert_abs_diff_eq!(c1.red, c2.red);
        assert_abs_diff_eq!(c1.blue, c2.blue);
        assert_abs_diff_eq!(c1.green, c2.green);
    }

    #[test]
    fn test_add_color() {
        let expected = Color::new(1.6, 0.7, 1.0);
        let actual = Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25);
        compare(expected, actual);
    }

    #[test]
    fn test_subtract_color() {
        let expected = Color::new(0.2, 0.5, 0.5);
        let actual = Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25);
        compare(expected, actual);
    }

    #[test]
    fn test_mul_color_by_scalar() {
        compare(Color::new(0.2, 0.3, 0.4) * 2f64, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn test_mul_colors() {
        let expected = Color::new(0.9, 0.2, 0.04);
        let actual = Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1);
        compare(expected, actual);
    }

    #[test]
    fn test_as_rgb() {
        let expected = (255, 128, 0);
        let actual = Color::new(1.5, 0.5, 0.0).as_rgb();
        assert_eq!(expected, actual);
    }
}
