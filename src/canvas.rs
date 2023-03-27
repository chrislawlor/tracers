use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0); width * height],
        }
    }
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Color {
    canvas.pixels[y * canvas.width + x]
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    canvas.pixels[y * canvas.width + x] = color;
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut buf = String::new();
    let mut line = String::new();

    buf.push_str("P3\n");
    buf.push_str(format!("{} {}\n", canvas.width, canvas.height).as_str());
    buf.push_str("255\n");

    for y in 0..canvas.width {
        for x in 0..canvas.height {
            let (r, g, b) = pixel_at(canvas, x, y).as_rgb();
            line.push_str(format!("{} {} {}\n", r, g, b).as_str());
            if line.len() > 60 {
                buf.push_str(line.as_str());
                line = String::new();
            }
        }
    }
    buf.push_str(line.as_str());

    let last_char = buf.chars().last().unwrap();
    if last_char != '\n' {
        buf.push_str("\n");
    }
    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_canvas_init() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels[0], Color::new(0, 0, 0));
        assert_eq!(canvas.pixels[199], Color::new(0, 0, 0));
    }

    #[test]
    fn test_pixel_at() {
        let canvas = Canvas::new(10, 20);
        let black = Color::new(0, 0, 0);
        assert_eq!(pixel_at(&canvas, 0, 0), black);
        assert_eq!(pixel_at(&canvas, 9, 19), black);
    }

    #[test]
    fn test_write_pixel() {
        let white = Color::new(1, 1, 1);
        let mut canvas = Canvas::new(10, 20);

        write_pixel(&mut canvas, 0, 0, white);
        write_pixel(&mut canvas, 9, 19, white);

        assert_eq!(pixel_at(&canvas, 0, 0), white);
        assert_eq!(pixel_at(&canvas, 9, 19), white);
    }

    #[test]
    fn test_canvas_to_ppm() {
        let white = Color::new(1, 1, 1);
        let mut canvas = Canvas::new(2, 2);
        write_pixel(&mut canvas, 0, 0, white);

        let ppm = canvas_to_ppm(&canvas);
        let expected = "P3\n2 2\n255\n255 255 255\n0 0 0\n0 0 0\n0 0 0\n".to_string();

        assert_eq!(ppm, expected);
    }
}
