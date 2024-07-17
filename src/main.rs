extern crate nalgebra_glm as glm;

mod framebuffer;
mod color;
mod bmp;
mod line_impl;

use framebuffer::Framebuffer;
use color::Color;
use bmp::BmpRenderable;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, 0x000000); 

    let points1 = vec![(165.0, 380.0), (185.0, 360.0), (180.0, 330.0), (207.0, 345.0), (233.0, 330.0), (230.0, 360.0), (250.0, 380.0), (220.0, 385.0), (205.0, 410.0), (193.0, 383.0)];
    let points2 = vec![(321.0, 335.0), (288.0, 286.0), (339.0, 251.0), (374.0, 302.0)];
    let points3 = vec![(377.0, 249.0), (411.0, 197.0), (436.0, 249.0)];

    framebuffer.fill_polygon(&points1, 0xFFFF00, 0xFFFFFF);
    framebuffer.fill_polygon(&points2, 0x0000FF, 0xFFFFFF); 


    framebuffer.fill_polygon(&points3, 0xFF0000, 0xFFFFFF); 


    framebuffer.render_to_bmp("Poligon-3.bmp").unwrap();
}
