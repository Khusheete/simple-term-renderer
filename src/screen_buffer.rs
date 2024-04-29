use crate::buffer2d::Buffer2D;
use crate::img::{Color, Image};
use crate::math::Vec2;

use std::cmp;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharBackgroundMode {
    Blend,
    Colored(Color)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharForegroundMode {
    Opposite,
    Colored(Color)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharData {
    pub c: Option<char>,
    pub bg_mode: CharBackgroundMode,
    pub fg_mode: CharForegroundMode
}


impl CharData {
    fn default() -> Self {
        Self {
            c: None,
            bg_mode: CharBackgroundMode::Blend,
            fg_mode: CharForegroundMode::Opposite
        }
    }
}


#[derive(Clone)]
pub struct ScreeBuffer {
    image: Image,
    text: Buffer2D<CharData>,
}


impl ScreeBuffer {


    pub fn new<V>(size: V) -> Self
        where V: AsRef<Vec2>
    {
        Self {
            image: Image::new(*size.as_ref()),
            text : Buffer2D::new(*size.as_ref() / 2, CharData::default())
        }
    }


    pub fn clear_color(&mut self, c: Color) {
        self.image.clear(c);
    }


    pub fn get_color<V>(&self, pos: V) -> Color
        where V: AsRef<Vec2>
    {
        self.image[pos]
    }


    pub fn raw_resize<V>(&mut self, new_size: V) 
        where V: AsRef<Vec2>
    {
        self.image.raw_resize(*new_size.as_ref());
        self.text.raw_resize(*new_size.as_ref() / 2, CharData::default());
    }


    pub fn point<V>(&mut self, p: V, c: Color)
        where V: AsRef<Vec2>
    {
        self.image[p] = c;
    }


    pub fn line<V1, V2>(&mut self, p1: V1, p2: V2, c: Color)
        where V1: AsRef<Vec2>, V2: AsRef<Vec2>
    {
        self.image.line(p1, p2, c);
    }


    pub fn rect_boudary<V1, V2>(&mut self, p: V1, s: V2, c: Color)
        where V1: AsRef<Vec2>, V2: AsRef<Vec2>
    {
        self.image.rect_boudary(p, s, c);
    }


    pub fn rect<V1, V2>(&mut self, p: V1, s: V2, c: Color) 
        where V1: AsRef<Vec2>, V2: AsRef<Vec2>
    {
        self.image.rect(p, s, c);
    }


    pub fn ellipse_boundary<V1, V2>(&mut self, center: V1, size: V2, c: Color) 
        where V1: AsRef<Vec2>, V2: AsRef<Vec2>
    {
        self.image.ellipse_boundary(center, size, c);
    }


    pub fn image<V1, V2, V3>(&mut self, img: &Image, pos: V1, size: V2, offset: V3, alpha: Option<Color>) 
        where V1: AsRef<Vec2>, V2: AsRef<Vec2>, V3: AsRef<Vec2>
    {
        self.image.image(img, pos, size, offset, alpha);
    }


    pub fn whole_image_alpha<V>(&mut self, img: &Image, pos: V, alpha: Color) 
        where V: AsRef<Vec2>
    {
        self.image.whole_image_alpha(img, pos, alpha);
    }


    pub fn whole_image<V>(&mut self, img: &Image, pos: V) 
        where V: AsRef<Vec2>
    {
        self.image.whole_image(img, pos);
    }


    pub fn size(&self) -> Vec2 {
        self.image.size()
    }


    pub fn clear_text(&mut self) {
        self.text.fill(CharData::default());
    }


    pub fn print_text_raw<V>(&mut self, text: String, pos: V)
        where V: AsRef<Vec2>
    {
        let mut pos: Vec2 = *pos.as_ref();
        
        // Skip if the text goes offscreen in the y direction
        // the x direction cannot be skipped because the text may enter the screen again
        if pos.y < 0 || pos.y >= self.size().y {
            return;
        }

        pos.y /= 2; // Adapt coordinates to be in the text space
        

        let char_iter = text.chars().enumerate()
            .skip((-cmp::min(pos.x, 0i32)) as usize);

        
        for (i, c) in char_iter {
            let x = pos.x + (i as i32);
            let char_pos = vec2!(x, pos.y);
            if x < 0 {
                continue
            }
            if x >= self.size().x {
                return // We are now sure that the full text will go offscreen
                // so we skip drawing everything
            }

            // Add the character to the text buffer
            self.text[char_pos].c = Some(c);
            self.text[char_pos].bg_mode = CharBackgroundMode::Blend;
            self.text[char_pos].fg_mode = CharForegroundMode::Opposite;
        }
    }


    pub fn get_char_data<V>(&self, pos: V) -> CharData
        where V: AsRef<Vec2>
    {
        let mut pos = *pos.as_ref();
        pos.y /= 2;
        self.text.get(pos, CharData::default())
    }
}