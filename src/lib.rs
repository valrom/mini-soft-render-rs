#[derive(Clone, Copy, Default)]
pub struct Point2D(pub i32, pub i32);

pub struct Canvas {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn get_render(&mut self) -> &[u32] {
        &self.data
    }

    pub fn draw_pixel(&mut self, point: Point2D, code: u32) {
        if point.0 > self.width as i32 || point.0 < 0 {
            panic!("X out of range");
        }

        if point.1 > self.height as i32 || point.1 < 0 {
            panic!("Y out of range");
        }

        self.data[point.0 as usize + point.1 as usize * self.width] = code;
    }

    pub fn draw_line(&mut self, from: Point2D, to: Point2D, code: u32) {
        if (to.1 - from.1).abs() < (to.0 - from.0).abs() {
            if from.0 > to.0 {
                self.line_low(to, from, code);
            } else {
                self.line_low( from, to, code);
            }
        } else {
            if from.1 > to.1 {
                self.line_high( to, from ,code);
            } else {
                self.line_high(from, to, code);
            }
        }
    }

    fn line_low(&mut self, from: Point2D, to: Point2D, code: u32) {
        let dx = to.0 - from.0;
        let mut dy = to.1 - from.1;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = (2 * dy) - dx;
        let mut y = from.1;

        for x in from.0..to.0 {
            self.draw_pixel(Point2D(x, y), code);

            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }

    fn line_high(&mut self, from: Point2D, to: Point2D, code: u32) {
        let mut dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut d = ( 2 * dx) - dy;
        let mut x = from.0;

        for y in from.1..to.1 {
            self.draw_pixel(Point2D(x, y), code);

            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2*dx;
            }
        }

    }

    pub fn clear(&mut self) {
        for i in &mut self.data {
            *i = 0;
        }
    }
}
