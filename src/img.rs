/*

    MIT License
    
    Copyright (c) 2022 Siandfrance
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/


use crate::math::Vec2;

use std::ops::{Index, IndexMut};
use std::fmt;
use std::path::Path;


use image::io::Reader as ImageReader;
use image::{Pixel, RgbImage};


/// RGB Color struct.
/// 
/// Fields:
/// ```
/// Color.r
/// Color.g
/// Color.b
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}


impl Color {

    pub const ALICE_BLUE         : Color = Color::hex(0xeff7ff);
    pub const ANTIQUE_WHITE      : Color = Color::hex(0xf9ebd6);
    pub const AQUA               : Color = Color::hex(0x00ffff);
    pub const AQUAMARINE         : Color = Color::hex(0x7effd4);
    pub const AZURE              : Color = Color::hex(0xefffff);
    pub const BEIGE              : Color = Color::hex(0xf4f4db);
    pub const BISQUE             : Color = Color::hex(0xffe4c3);
    pub const BLACK              : Color = Color::hex(0x000000);
    pub const BLANCHED_ALMOND    : Color = Color::hex(0xffebcd);
    pub const BLUE               : Color = Color::hex(0x0000ff);
    pub const BLUE_VIOLET        : Color = Color::hex(0x892ae2);
    pub const BROWN              : Color = Color::hex(0xa52a2a);
    pub const BURLYWOOD          : Color = Color::hex(0xddb887);
    pub const CADET_BLUE         : Color = Color::hex(0x5e9ea0);
    pub const CHARTREUSE         : Color = Color::hex(0x7eff00);
    pub const CHOCOLATE          : Color = Color::hex(0xd1691d);
    pub const CORAL              : Color = Color::hex(0xff7e50);
    pub const CORNFLOWER_BLUE    : Color = Color::hex(0x6495ed);
    pub const CORNSILK           : Color = Color::hex(0xfff7db);
    pub const CRIMSON            : Color = Color::hex(0xdb143b);
    pub const CYAN               : Color = Color::hex(0x00ffff);
    pub const DARK_BLUE          : Color = Color::hex(0x00008a);
    pub const DARK_CYAN          : Color = Color::hex(0x008a8a);
    pub const DARK_GOLDENROD     : Color = Color::hex(0xb8850b);
    pub const DARK_GRAY          : Color = Color::hex(0xa8a8a8);
    pub const DARK_GREEN         : Color = Color::hex(0x006400);
    pub const DARK_KHAKI         : Color = Color::hex(0xbcb66b);
    pub const DARK_MAGENTA       : Color = Color::hex(0x8a008a);
    pub const DARK_OLIVE_GREEN   : Color = Color::hex(0x546b2f);
    pub const DARK_ORANGE        : Color = Color::hex(0xff8c00);
    pub const DARK_ORCHID        : Color = Color::hex(0x9931cc);
    pub const DARK_RED           : Color = Color::hex(0x8a0000);
    pub const DARK_SALMON        : Color = Color::hex(0xe89579);
    pub const DARK_SEA_GREEN     : Color = Color::hex(0x8ebc8e);
    pub const DARK_SLATE_BLUE    : Color = Color::hex(0x483d8a);
    pub const DARK_SLATE_GRAY    : Color = Color::hex(0x2f4f4f);
    pub const DARK_TURQUOISE     : Color = Color::hex(0x00cdd1);
    pub const DARK_VIOLET        : Color = Color::hex(0x9300d3);
    pub const DEEP_PINK          : Color = Color::hex(0xff1493);
    pub const DEEP_SKY_BLUE      : Color = Color::hex(0x00bfff);
    pub const DIM_GRAY           : Color = Color::hex(0x696969);
    pub const DODGER_BLUE        : Color = Color::hex(0x1d90ff);
    pub const FIREBRICK          : Color = Color::hex(0xb12121);
    pub const FLORAL_WHITE       : Color = Color::hex(0xfff9ef);
    pub const FOREST_GREEN       : Color = Color::hex(0x218a21);
    pub const FUCHSIA            : Color = Color::hex(0xff00ff);
    pub const GAINSBORO          : Color = Color::hex(0xdbdbdb);
    pub const GHOST_WHITE        : Color = Color::hex(0xf7f7ff);
    pub const GOLD               : Color = Color::hex(0xffd600);
    pub const GOLDENROD          : Color = Color::hex(0xdaa51f);
    pub const GRAY               : Color = Color::hex(0xbdbdbd);
    pub const GREEN              : Color = Color::hex(0x00ff00);
    pub const GREEN_YELLOW       : Color = Color::hex(0xacff2f);
    pub const HONEYDEW           : Color = Color::hex(0xefffef);
    pub const HOT_PINK           : Color = Color::hex(0xff69b3);
    pub const INDIAN_RED         : Color = Color::hex(0xcd5b5b);
    pub const INDIGO             : Color = Color::hex(0x4b0082);
    pub const IVORY              : Color = Color::hex(0xffffef);
    pub const KHAKI              : Color = Color::hex(0xefe68c);
    pub const LAVENDER           : Color = Color::hex(0xe6e6f9);
    pub const LAVENDER_BLUSH     : Color = Color::hex(0xffeff4);
    pub const LAWN_GREEN         : Color = Color::hex(0x7cfb00);
    pub const LEMON_CHIFFON      : Color = Color::hex(0xfff9cd);
    pub const LIGHT_BLUE         : Color = Color::hex(0xacd8e6);
    pub const LIGHT_CORAL        : Color = Color::hex(0xef8080);
    pub const LIGHT_CYAN         : Color = Color::hex(0xdfffff);
    pub const LIGHT_GOLDENROD    : Color = Color::hex(0xf9f9d1);
    pub const LIGHT_GRAY         : Color = Color::hex(0xd3d3d3);
    pub const LIGHT_GREEN        : Color = Color::hex(0x90ed90);
    pub const LIGHT_PINK         : Color = Color::hex(0xffb6c1);
    pub const LIGHT_SALMON       : Color = Color::hex(0xffa079);
    pub const LIGHT_SEA_GREEN    : Color = Color::hex(0x1fb1aa);
    pub const LIGHT_SKY_BLUE     : Color = Color::hex(0x87cdf9);
    pub const LIGHT_SLATE_GRAY   : Color = Color::hex(0x778799);
    pub const LIGHT_STEEL_BLUE   : Color = Color::hex(0xafc3dd);
    pub const LIGHT_YELLOW       : Color = Color::hex(0xffffdf);
    pub const LIME               : Color = Color::hex(0x00ff00);
    pub const LIME_GREEN         : Color = Color::hex(0x31cd31);
    pub const LINEN              : Color = Color::hex(0xf9efe6);
    pub const MAGENTA            : Color = Color::hex(0xff00ff);
    pub const MAROON             : Color = Color::hex(0xaf2f60);
    pub const MEDIUM_AQUAMARINE  : Color = Color::hex(0x66cdaa);
    pub const MEDIUM_BLUE        : Color = Color::hex(0x0000cd);
    pub const MEDIUM_ORCHID      : Color = Color::hex(0xba54d3);
    pub const MEDIUM_PURPLE      : Color = Color::hex(0x9370db);
    pub const MEDIUM_SEA_GREEN   : Color = Color::hex(0x3bb370);
    pub const MEDIUM_SLATE_BLUE  : Color = Color::hex(0x7b67ed);
    pub const MEDIUM_SPRING_GREEN: Color = Color::hex(0x00f99a);
    pub const MEDIUM_TURQUOISE   : Color = Color::hex(0x48d1cc);
    pub const MEDIUM_VIOLET_RED  : Color = Color::hex(0xc61485);
    pub const MIDNIGHT_BLUE      : Color = Color::hex(0x181870);
    pub const MINT_CREAM         : Color = Color::hex(0xf4fff9);
    pub const MISTY_ROSE         : Color = Color::hex(0xffe4e1);
    pub const MOCCASIN           : Color = Color::hex(0xffe4b5);
    pub const NAVAJO_WHITE       : Color = Color::hex(0xffddac);
    pub const NAVY_BLUE          : Color = Color::hex(0x000080);
    pub const OLD_LACE           : Color = Color::hex(0xfdf4e6);
    pub const OLIVE              : Color = Color::hex(0x808000);
    pub const OLIVE_DRAB         : Color = Color::hex(0x6b8e23);
    pub const ORANGE             : Color = Color::hex(0xffa500);
    pub const ORANGE_RED         : Color = Color::hex(0xff4400);
    pub const ORCHID             : Color = Color::hex(0xda70d6);
    pub const PALE_GOLDENROD     : Color = Color::hex(0xede8aa);
    pub const PALE_GREEN         : Color = Color::hex(0x97fb97);
    pub const PALE_TURQUOISE     : Color = Color::hex(0xafeded);
    pub const PALE_VIOLET_RED    : Color = Color::hex(0xdb7093);
    pub const PAPAYA_WHIP        : Color = Color::hex(0xffefd4);
    pub const PEACH_PUFF         : Color = Color::hex(0xffdab8);
    pub const PERU               : Color = Color::hex(0xcd853f);
    pub const PINK               : Color = Color::hex(0xffbfca);
    pub const PLUM               : Color = Color::hex(0xdda0dd);
    pub const POWDER_BLUE        : Color = Color::hex(0xafdfe6);
    pub const PURPLE             : Color = Color::hex(0xa01fef);
    pub const REBECCA_PURPLE     : Color = Color::hex(0x663399);
    pub const RED                : Color = Color::hex(0xff0000);
    pub const ROSY_BROWN         : Color = Color::hex(0xbc8e8e);
    pub const ROYAL_BLUE         : Color = Color::hex(0x4169e1);
    pub const SADDLE_BROWN       : Color = Color::hex(0x8a4412);
    pub const SALMON             : Color = Color::hex(0xf98072);
    pub const SANDY_BROWN        : Color = Color::hex(0xf4a360);
    pub const SEA_GREEN          : Color = Color::hex(0x2d8a56);
    pub const SEASHELL           : Color = Color::hex(0xfff4ed);
    pub const SIENNA             : Color = Color::hex(0xa0522d);
    pub const SILVER             : Color = Color::hex(0xbfbfbf);
    pub const SKY_BLUE           : Color = Color::hex(0x87cdeb);
    pub const SLATE_BLUE         : Color = Color::hex(0x6959cd);
    pub const SLATE_GRAY         : Color = Color::hex(0x708090);
    pub const SNOW               : Color = Color::hex(0xfff9f9);
    pub const SPRING_GREEN       : Color = Color::hex(0x00ff7e);
    pub const STEEL_BLUE         : Color = Color::hex(0x4682b3);
    pub const TAN                : Color = Color::hex(0xd1b38c);
    pub const TEAL               : Color = Color::hex(0x008080);
    pub const THISTLE            : Color = Color::hex(0xd8bfd8);
    pub const TOMATO             : Color = Color::hex(0xff6246);
    pub const TRANSPARENT        : Color = Color::hex(0xffffff);
    pub const TURQUOISE          : Color = Color::hex(0x3fdfcf);
    pub const VIOLET             : Color = Color::hex(0xed82ed);
    pub const WEB_GRAY           : Color = Color::hex(0x808080);
    pub const WEB_GREEN          : Color = Color::hex(0x008000);
    pub const WEB_MAROON         : Color = Color::hex(0x800000);
    pub const WEB_PURPLE         : Color = Color::hex(0x800080);
    pub const WHEAT              : Color = Color::hex(0xf4ddb3);
    pub const WHITE              : Color = Color::hex(0xffffff);
    pub const WHITE_SMOKE        : Color = Color::hex(0xf4f4f4);
    pub const YELLOW             : Color = Color::hex(0xffff00);
    pub const YELLOW_GREEN       : Color = Color::hex(0x9acd31);


    /// Creates a color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b
        }
    }


    /// Creates a color.
    pub const fn hex(h: u32) -> Self {
        Self {
            r: ((h & 0x00FF0000) / 0x00010000) as u8,
            g: ((h & 0x0000FF00) / 0x00000100) as u8,
            b: ((h & 0x000000FF) / 0x00000001) as u8
        }
    }
}


impl fmt::Display for Color {

    /// Writes the CSI to set background or color (respectively when using {:-} or {:+}) to `f`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.sign_minus() {
            write!(f, "\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
        } else {
            write!(f, "\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
        }
    }
}


#[derive(Clone)]
/// Image struct. This is a Color buffer.
/// Pixels can be accessed by indexing with the pixel coordinates.
pub struct Image {
    data: Vec<Color>,
    size: Vec2
}


impl Image {

    /// Creates an image of size (`w`, `h`). All the pixels are set to black.
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![Color::BLACK; w * h],
            size: vec2!(w as i32, h as i32)
        }
    }


    /// Loads an image from a file.
    pub fn load<P>(path: P) -> Result<Self, String>
            where P: AsRef<Path> {
        let img = match ImageReader::open(path) {
            Ok(s) => match s.decode() {
                Ok(img) => img,
                Err(e) => return Err(format!("{}", e))
            }
            Err(e) => return Err(format!("{}", e))
        }.to_rgb8();
        let mut result = Image::new(img.width() as usize, img.height() as usize);
        for i in 0..img.width() {
            for j in 0..img.height() {
                let px = img.get_pixel(i, j).channels();
                result[vec2!(i as i32, j as i32)] = Color::rgb(px[0], px[1], px[2]);
            }
        }
        Ok(result)
    }


    /// Saves an image to a file.
    pub fn save<P>(&self, path: P) -> Result<(), String>
            where P: AsRef<Path> {
        let mut img = RgbImage::new(self.size.x as u32, self.size.y as u32);
        for i in 0..self.size.x {
            for j in 0..self.size.y {
                let mut pix = img.get_pixel_mut(i as u32, j as u32);
                let c = self[vec2!(i, j)];
                pix.0 = [c.r, c.g, c.b];
            }
        }
        match img.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e))
        }
    }



    /// Returns the size of the image.
    pub fn size(&self) -> Vec2 {
        self.size
    }


    /// Resizes the image. New pixels are set to black.
    /// It corrupts the image (pixel position is misinterpreted, but it is faster than Image::resize).
    pub fn raw_resize<A>(&mut self, new_size: A) 
        where A: AsRef<Vec2>
    {
        self.size = *new_size.as_ref();
        self.data.resize((self.size.x * self.size.y) as usize, Color::BLACK);
        self.data.shrink_to_fit();
    }


    /// Resizes the image keeping the top left part of the image
    pub fn resize<A>(&mut self, new_size: A) 
        where A: AsRef<Vec2>
    {
        // TOFIX: buffer overflow
        let new_size = new_size.as_ref();
        if new_size.x != self.size.x {
            let mut cnt: usize = new_size.x as usize;
            for j in 1..new_size.y {
                for i in 0..new_size.x {
                    self.data[cnt] = self[(i, j)];
                    cnt += 1;
                }
            }
        }
        self.raw_resize(new_size);
    }


    fn is_out_of_range<A>(&self, p: A) -> bool
        where A: AsRef<Vec2> 
    {
        let p = p.as_ref();
        p.x < 0 || p.y < 0 || p.x >= self.size.x || p.y >= self.size.y
    }


    /// Sets the pixel color at `p` to `c`.
    pub fn point<A>(&mut self, p: A, c: Color)
        where A: AsRef<Vec2>
    {
        self[*p.as_ref()] = c;
    }


    /// Draws a line of color `c` between `p1` and `p2`.
    pub fn line<A, B>(&mut self, p1: A, p2: B, c: Color)
        where A: AsRef<Vec2>, B: AsRef<Vec2> 
    {
        let mut p1 = *p1.as_ref();
        let p2 = p2.as_ref();

        let dx = (p2.x - p1.x).abs();
        let sx = if p1.x < p2.x {1} else {-1};
        let dy = -(p2.y - p1.y).abs();
        let sy = if p1.y < p2.y {1} else {-1};

        let mut err = dx + dy;

        self[p1] = c;

        while (p1.x != p2.x || p1.y != p2.y)
             && ((p1.x < self.size.x && sx > 0) || (p1.x >= 0 && sx < 0))
             && ((p1.y < self.size.y && sy > 0) || (p1.y >= 0 && sy < 0))
        {
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                p1.x += sx;
            }
            if e2 <= dx {
                err += dx;
                p1.y += sy;
            }

            self[p1] = c;
        }
    }


    /// Same as `rect` but draws only the four sides of the rectangle.
    pub fn rect_boudary<A, B>(&mut self, p: A, s: B, c: Color)
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        let p = p.as_ref();
        let s = s.as_ref();
        self.line((p.x      , p.y      ), (p.x + s.x, p.y      ), c);
        self.line((p.x + s.x, p.y      ), (p.x + s.x, p.y + s.y), c);
        self.line((p.x + s.x, p.y + s.y), (p.x      , p.y + s.y), c);
        self.line((p.x      , p.y + s.y), (p.x      , p.y      ), c);
    }


    /// Draws a rectangle of color `c` and of size `s`. 
    /// `p` is the coordinate of the top left corner of the rectangle.
    pub fn rect<A, B>(&mut self, p: A, s: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        let mut p = *p.as_ref();
        let mut s = *s.as_ref();

        if p.x < 0 {
            s.x += p.x - 1;
            p.x = 0;
        }
        if p.y < 0 {
            s.y += p.y - 1;
            p.y = 0;
        }

        let dx = if s.x > 0 {1} else {-1};
        let dy = if s.y > 0 {1} else {-1};

        for j in 0..(s.y.abs()) {
            let y = p.y + j * dy;
            if y >= self.size.y {break}
            for i in 0..(s.x.abs()) {
                let x = p.x + i * dx;
                if x >= self.size.x {break}

                self[(x, y)] = c;
            }
        }
    }


    /// Sets all the pixels' color in the screen to `c`.
    pub fn clear(&mut self, c: Color) {
        for i in 0..self.data.len() {
            self.data[i] = c;
        }
    }


    fn plot_ellipse_points<A, B>(&mut self, center: A, pos: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        let center = center.as_ref();
        let pos    = pos.as_ref();
        self[(center.x + pos.x, center.y + pos.y)] = c;
        self[(center.x + pos.x, center.y - pos.y)] = c;
        self[(center.x - pos.x, center.y + pos.y)] = c;
        self[(center.x - pos.x, center.y - pos.y)] = c;
    }


    /// Draws an ellipse of color `col`. `c` is the center of the ellipse and `s` is the size of the rectangle
    /// in which the ellipse is inscribed.
    pub fn ellipse_boundary<A, B>(&mut self, center: A, size: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        let center = center.as_ref();
        let size   = size.as_ref();

        let a = size.x / 2;
        let b = size.y / 2;

        //prepare to plot in the first region
        let mut x = 0;
        let mut y = b;
        let mut p    = b * b + (a * a * (1 - 4*b) - 2) / 4;
        let mut dpe  = 3 * b * b;
        let mut dpse  = dpe - 2 * a * a * (b - 1);
        let d2pe  = 2 * b * b;
        let d2pse = d2pe + 2 * a * a;

        //plot in the first region
        self.plot_ellipse_points(center, (x, y), c);
        while dpse < 2 * a * a + 3 * b * b {
            if p < 0 { //east
                p    += dpe;
                dpe  += d2pe;
                dpse += d2pe;
            } else {     //south-east
                p    += dpse;
                dpe  += d2pe;
                dpse += d2pse;
                y -= 1;
            }
            x += 1;
            self.plot_ellipse_points(center, (x, y), c);
        }

        //prepare to plot in the second region
        let mut p    = p - (a * a * (4 * y - 3) + b * b * (4 * x + 3) + 2) / 4;
        let mut dpse = 2 * b * b + 3 * a * a;
        let dps  = a * a * (3 - 2 * y);
        let d2ps = 2 * a * a;

        //plot in the second region
        while y > 0 {
            if p > 0 { //south
                p    += dps;
                dpe  += d2ps;
                dpse += d2ps;
            } else {     //south-east
                p    += dpse;
                dpe  += d2ps;
                dpse += d2pse;
                x += 1;
            }
            y -= 1;
            self.plot_ellipse_points(center, (x, y), c);
        }
    }


    /// Draws an image at position `pos`. 
    /// 
    /// Negative size results in flipped image. Alpha is used to ignore a given color while drawing.
    pub fn image<A, B, C>(&mut self, img: &Image, pos: A, size: B, offset: C, alpha: Option<Color>) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>, C: AsRef<Vec2>
    {
        let offset = offset.as_ref();
        let mut p = *pos.as_ref();
        let mut s = *size.as_ref();

        if p.x < 0 {
            s.x += p.x - 1;
            p.x = 0;
        }
        if p.y < 0 {
            s.y += p.y - 1;
            p.y = 0;
        }


        let dx = if s.x > 0 {1} else {-1};
        let dy = if s.y > 0 {1} else {-1};


        for j in 0..(s.y.abs()) {
            let y = p.y + j * dy;
            let src_y = offset.y + j;
            if y >= self.size.y {break}
            if src_y >= img.size.y {break}
            for i in 0..(s.x.abs()) {
                let x = p.x + i * dx;
                let src_x = offset.x + i;
                if x >= self.size.x {break}
                if src_x >= img.size.x {break}

                let pos = vec2!(x, y);
                let src_pos = vec2!(src_x, src_y);

                if let Some(acolor) = alpha {
                    if acolor == img[src_pos] {
                        continue;
                    }
                }
                self[pos] = img[src_pos];
            }
        }
    }


    /// Draws the whole image at `pos`, ignoring the color `alpha`.
    /// 
    /// Literally:
    /// ```
    /// <image>.image(img, pos, img.size(), Vec2::ZERO, Some(alpha));
    /// ```
    pub fn whole_image_alpha<A>(&mut self, img: &Image, pos: A, alpha: Color) 
        where A: AsRef<Vec2>
    {
        self.image(img, pos, img.size(), Vec2::ZERO, Some(alpha));
    }


    /// Draws the whole image at `pos`.
    /// 
    /// Literally:
    /// ```
    /// <image>.image(img, pos, img.size(), Vec2::ZERO, None);
    /// ```
    pub fn whole_image<A>(&mut self, img: &Image, pos: A) 
        where A: AsRef<Vec2>
    {
        self.image(img, pos, img.size(), Vec2::ZERO, None);
    }
}


impl<A: AsRef<Vec2>> Index<A> for Image {
    type Output = Color;

    fn index(&self, p: A) -> &Self::Output {
        let p = p.as_ref();
        if !self.is_out_of_range(p) {
            &self.data[(p.x + p.y * self.size.x) as usize]
        } else {
            &Color::BLACK
        }
    }
}


impl<A: AsRef<Vec2>> IndexMut<A> for Image {

    fn index_mut(&mut self, p: A) -> &mut Self::Output {
        static mut TEMP: Color = Color::BLACK;
        let p = p.as_ref();

        if !self.is_out_of_range(p) {
            &mut self.data[(p.x + p.y * self.size.x) as usize]
        } else {
            unsafe { &mut TEMP } // NOT GOOD, ignore index out of range
        }
    }
}