/// # canvas
/// `canvas` is a module to represent the canvas of the scene

use std::iter::Iterator;
use super::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let pixels = (0..height).map(|_| (0..width).map(|_| Color::new(0.0, 0.0, 0.0)).collect()).collect();

        Canvas {width, height, pixels}
    }

    pub fn pixel_at(&self, row: i32, col: i32) -> &Color {
        &self.pixels[row as usize][col as usize]
    }

    pub fn write_pixel(&mut self, row: i32, col: i32, color: &Color) {
        self.pixels[row as usize][col as usize] = Color::new(color.red.value(), color.green.value(), color.blue.value());
    }
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut str = String::new();
    let max_color_val = 255.0;

    // Push header
    str.push_str("P3\n");
    str.push_str(format!("{} {}\n", canvas.width, canvas.height).as_ref());
    str.push_str(format!("{}\n", max_color_val).as_ref());

    // Push pixels
    for i in 0..canvas.height {
        for j in 0..canvas.width {
            let color = canvas.pixel_at(i, j);
            let red = (&color.red * max_color_val).clamp(0.0, max_color_val);
            let green = (&color.green * max_color_val).clamp(0.0, max_color_val);
            let blue = (&color.blue * max_color_val).clamp(0.0, max_color_val);
            str.push_str(format!("{:.0} {:.0} {:.0} ", red, green, blue).as_ref());
        }
        str.push_str("\n");
    }
    str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_creation() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for i in 0..c.height {
            for j in 0..c.width {
                assert_eq!(c.pixels[i as usize][j as usize], Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn canvas_operations() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        &c.write_pixel(2, 3, &red);
        assert_eq!(c.pixel_at(2, 3), &red);
    }

    #[test]
    fn canvas_export() {
        // To PPM
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, &Color::new(1.5, 0.0, 0.0));
        c.write_pixel(1, 2, &Color::new(0.0, 0.5, 0.0));
        c.write_pixel(2, 4, &Color::new(-0.5, 0.0, 1.0));
        let actual = canvas_to_ppm(&c);
        let expected =
        "\
        P3\n\
        5 3\n\
        255\n\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n\
        ";
        assert_eq!(actual, expected);
    }
}
