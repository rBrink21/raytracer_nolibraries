
use crate::prelude::*;
pub struct Canvas{
    width: i32,
    pub height: i32,
    pixels: Vec<Vec<Color>>
}
impl Canvas{
    #[allow(clippy::cast_sign_loss)]
    pub fn new(width:i32,height:i32) ->Self{
        Self{
            width,
            height,
            pixels: vec![vec![Color::zero(); width as usize]; height as usize]
        }
    }
    #[allow(clippy::cast_sign_loss)]
    pub fn write_pixel (&mut self,x:i32,y:i32, color: Color) {
        if x < 0 || x > self.width - 1 {return}
        if y < 0 || y > self.height - 1{return}
        
        self.pixels[y as usize ][x as usize] = color;
    }
    
    pub fn to_pmm(&self) -> String { 
        const MAX_CHARS_PER_LINE :i32 = 58;
        
        let mut header = format!("P3\n{} {}\n255\n",self.width,self.height);
        let mut pixel_data = String::new();
        let mut character_count = 0;
        
        
        for row in &self.pixels {
            for pixel in row {
                let pixel_str = format!("{} {} {} ", pixel.red, pixel.green, pixel.blue);
                pixel_data += &*pixel_str;
                character_count += pixel_str.len() as i32;
            }
            if character_count > MAX_CHARS_PER_LINE { character_count = 0; pixel_data += "\n";}
            
            pixel_data += "\n";
        }
        header += pixel_data.as_str();
        header
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::canvas::{Canvas};
    use crate::color::Color;

    #[test]
    fn test_creating_canvas(){
        let canvas = Canvas::new(10,20);
        
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        
        for row in canvas.pixels {
            for pixel in row {
                assert_eq!(pixel.red,0.0);
                assert_eq!(pixel.blue,0.0);
                assert_eq!(pixel.green,0.0);
            }
        }
    }
    
    #[test]
    fn test_write_pixel(){
        let mut canvas = Canvas::new(20,20);
        canvas.write_pixel(1,1,Color::new(20.0,0.0,0.0));
        assert_eq!(canvas.pixels[1][1].red,20.0)
    }
    
    #[test]
    fn test_canvas_to_pmm(){
        let mut canvas = Canvas::new(2,2);
        canvas.write_pixel(0,0,Color::new(255.0,0.0,0.0));
        let expected_string = String::from("P3\n2 2\n255\n255 0 0 0 0 0 \n0 0 0 0 0 0 \n");
        assert_eq!(canvas.to_pmm(),expected_string);
    }
}