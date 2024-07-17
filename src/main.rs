
extern crate nalgebra_glm as glm;

mod framebuffer;
mod color;
mod bmp;
mod line_impl;

use framebuffer::Framebuffer;
use color::Color;
use bmp::BmpRenderable;

fn main() {
    let mut framebuffer = Framebuffer {
        width: 800,
        height: 600,
        buffer: vec![0x000000; 800 * 600],
        current_color: 0xFFFF00,
        background_color: 0xFFFFFF 
    };

    let points = vec![
        (165.0, 380.0), (185.0, 360.0), (180.0, 330.0), (207.0, 345.0),
        (233.0, 330.0), (230.0, 360.0), (250.0, 380.0), (220.0, 385.0),
        (205.0, 410.0), (193.0, 383.0)
    ];

    framebuffer.current_color = 0xFFFFFF; 
    framebuffer.draw_polygon(&points);

    framebuffer.current_color = 0xFFFF00;
    framebuffer.fill_polygon(&points);

    framebuffer.current_color = 0xFFFFFF; 
    framebuffer.draw_polygon(&points);

    framebuffer.render_to_bmp("Poligon-1.bmp").unwrap();
}

