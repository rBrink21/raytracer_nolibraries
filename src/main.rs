#![forbid(clippy::all)]
#![allow(unused_assignments,unused_imports)]
mod points;
mod color;
mod canvas;
mod matrix;

mod prelude {
    #![allow(unused_imports)]
    pub use crate::points::*;
    pub use crate::color::*;
    pub use crate::canvas::*;
    pub const EPSILON: f32 = 0.001;
}

use std::fs;
use prelude::*;


fn main() {
    
}

fn write_canvas_to_file(canvas :&Canvas){
    fs::write("./output/image.ppm",canvas.to_pmm()).expect("Unable to write to file");
}

fn draw_projectile(){
    let mut canvas = Canvas::new(800,400);
    let mut point = Vector::new(0.1,1.0,0.0);
    let mut velocity = Vector::new(1.0,1.8,0.6);
    velocity.normalize();
    velocity.scale(9.25);
    let gravity = Vector::new(0.0,-0.1,0.0);
    let wind = Vector::new(-0.01,0.0,0.0);

    while point.y > 0.0{
        point.apply(&velocity);
        velocity.apply(&gravity);
        velocity.apply(&wind);

        let canvas_y = canvas.height - point.y as i32;
        canvas.write_pixel(point.x as i32, point.z as i32, Color::from_green());
        println!("X:{} Y:{}",point.x,point.y);
    }
    write_canvas_to_file(&canvas);
}

pub fn compare_float(a: f32, b: f32) -> bool {
    let mut result = 0.0;
    if a < b {
        result = b - a;
    } else {
        result = a - b;
    }

    if result < EPSILON {
        return true;
    }
    false
}
