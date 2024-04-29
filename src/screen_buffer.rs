use crate::buffer2d::Buffer2D;
use crate::img::{Color, Image};
use crate::math::Vec2;

use std::ops::{Index, IndexMut};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CharBackgroundMode {
    Blend,
    Colored(Color)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CharForegroundMode {
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


    pub fn get_color<V>(&self, pos: V) -> Color
        where V: AsRef<Vec2>
    {
        self.image[pos]
    }


    pub fn raw_resize<V>(&mut self, new_size: V) 
        where V: AsRef<Vec2>
    {
        self.image.raw_resize(*new_size.as_ref());
        self.text.raw_resize(*new_size.as_ref(), CharData::default());
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


    pub fn clear(&mut self, c: Color) {
        self.image.clear(c);
        self.text.fill(CharData::default());
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
}