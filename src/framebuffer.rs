
use crate::line_impl::Line;
extern crate nalgebra_glm as glm;
use glm::Vec3;
extern crate image;


pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub background_color: u32, 
    pub current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: u32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![background_color; width * height],
            background_color,
            current_color: 0x000000,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn draw_polygon(&mut self, points: &[(f32, f32)]) {
        if points.len() < 3 {
            return;
        }
        
        let mut j = points.len() - 1;
        for i in 0..points.len() {
            let p1 = Vec3::new(points[i].0, points[i].1, 0.0);
            let p2 = Vec3::new(points[j].0, points[j].1, 0.0);
            self.line(p1, p2);
            j = i;
        }
    }

    pub fn fill_polygon(&mut self, points: &[(f32, f32)]) {
        self.current_color = 0xFFFF00; 
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for &(x, y) in points {
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        for y in min_y as usize..=max_y as usize {
            let mut node_x: Vec<f32> = Vec::new();
            let mut j = points.len() - 1;
            for i in 0..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                if y1 as usize <= y && y2 as usize > y || y2 as usize <= y && y1 as usize > y {
                    let x = x1 + (y as f32 - y1) * (x2 - x1) / (y2 - y1);
                    node_x.push(x);
                }
                j = i;
            }

            node_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for i in (1..node_x.len()).step_by(2) {
                for x in node_x[i-1] as usize..=node_x[i] as usize {
                    self.set_pixel(x, y, self.current_color);
                }
            }
        }
    }
}
