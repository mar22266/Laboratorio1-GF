
extern crate nalgebra_glm as glm;
use glm::Vec3;
use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let x1 = start.x as isize;
        let y1 = start.y as isize;
        let x2 = end.x as isize;
        let y2 = end.y as isize;

        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x as usize, y as usize, self.current_color);
            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
