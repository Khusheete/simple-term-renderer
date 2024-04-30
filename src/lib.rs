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


/* 
    Credit to `redox-os` for the termion crate that helped with the design of this library
    termion gitlab: https://gitlab.redox-os.org/redox-os/termion
*/


extern crate termios;
extern crate image;
extern crate palette;


#[macro_use]
pub mod math;
pub mod img;

pub mod rds;
pub mod input;


mod buffer2d;
mod screen_buffer;


#[cfg(test)]
mod tests {

    use crate::rds::Renderer;

    use crate::math::{*};
    use crate::img::*;
    use crate::input::{Input, InputEvent, KeyEvent, MouseEvent};

    use std::f64::consts::TAU;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use std::thread::sleep;


    #[test]
    fn renderer() {
        // load an image and draw it on screen
        let img = Arc::new(Mutex::new(Image::load("icon.png").unwrap()));

        // get the renderer
        let rdr = Renderer::get();


        // draw a frame on screen
        rdr.begin_draw();
        rdr.draw_line((2, 7), (28, 6), Color::WHITE);
        rdr.draw_rect((40, 15), (15, -10), Color::RED);
        rdr.draw_rect_boundary((40, 15), (15, -10), Color::CHOCOLATE);
        rdr.draw_ellipse_boundary((45, 25), (25, 8), Color::AQUAMARINE);

        rdr.draw_ellipse_boundary((60, 30), (4, 4), Color::DEEP_PINK);

        rdr.draw_rect((80, 5), (16, 8), Color::CORAL);
        rdr.draw_whole_image_alpha(img.clone(), (80, 5), Color::BLACK);

        rdr.ring_bell();

        rdr.end_draw();
        
        
        // wait for input and exit
        Input::get().get_event_blocking();

        // exit properly
        Renderer::exit();
    }


    #[test]
    fn input() {
        let rdr = Renderer::get();
        let inp = Input::get();
        Input::enable_mouse();

        let mut pos = Renderer::get_size() / 2;

        loop {
            let size = Renderer::get_size();

            // manage input
            match inp.get_event() {
                Some(event) => {
                    match event {
                    InputEvent::Key(event) => match event {
                        KeyEvent::Ctrl('c') => Renderer::exit(),
                        KeyEvent::Up        => if pos.y >  1            {pos.y -= 1},
                        KeyEvent::Down      => if pos.y <= size.y - 2   {pos.y += 1},
                        KeyEvent::Left      => if pos.x >  1            {pos.x -= 1},
                        KeyEvent::Right     => if pos.x <= size.x - 2   {pos.x += 1},
                        _ => ()
                    }
                    InputEvent::Mouse(event) => match event {
                        MouseEvent::ButtonPressed(_, mpos) | MouseEvent::Hold(_, mpos)
                            => pos = mpos,
                        _ => ()
                    }
                    _ => ()
                }
                }
                None => ()
            };

            // draw on screen
            rdr.begin_draw();
            rdr.clear_color(Color::BLACK);
            rdr.draw_rect_boundary(Vec2::ZERO, size - vec2!(1, 1), Color::BROWN);
            rdr.draw_point(pos, Color::WHITE);
            rdr.end_draw();
        }
    }


    #[test]
    fn text() {
        let rdr = Renderer::get();
        let inp = Input::get();
        
        let dynamic_text = String::from("Some dynamic text !!!");
        let dyn_text_char_count = dynamic_text.chars().count();

        let mut dyn_text_pos: Vec2f = Vec2f::ZERO;
        let mut dyn_text_speed: Vec2f = Vec2f::new(TAU * 10.0, 17.0);

        let mut instant = Instant::now();
        let max_frame_rate: f64 = 60.0;
        let max_frame_time: f64 = 1.0 / max_frame_rate;

        loop {
            // Limit frame rate
            let delta: f64 = instant.elapsed().as_secs_f64();
            if delta < max_frame_time {
                sleep(Duration::from_secs_f64(max_frame_time - delta));
            }
            let delta: f64 = instant.elapsed().as_secs_f64();
            instant = Instant::now();
            

            // Handle input
            match inp.get_event() {
                Some(event) => {
                    match event {
                        InputEvent::Key(event) => match event {
                            KeyEvent::Ctrl('c') => Renderer::exit(),
                            _ => ()
                        }
                        _ => ()
                    }
                }
                None => ()
            };

            // Update text values
            let size = Vec2f::from(Renderer::get_size());
            dyn_text_pos += dyn_text_speed * delta;
            if dyn_text_pos.x <= 1.0 {
                dyn_text_speed.x = dyn_text_speed.x.abs();
            }
            if dyn_text_pos.x >= size.x - dyn_text_char_count as f64 - 1.0 {
                dyn_text_speed.x = -dyn_text_speed.x.abs();
            }
            if dyn_text_pos.y <= 1.0 {
                dyn_text_speed.y = dyn_text_speed.y.abs();
            }
            if dyn_text_pos.y >= size.y - 1.0 {
                dyn_text_speed.y = -dyn_text_speed.y.abs();
            }


            // Draw frame
            rdr.begin_draw();
            rdr.clear(Color::BLACK);
            rdr.print_blended_text_raw(&format!("Time delta: {}", delta), (1, 15));

            rdr.print_colored_text_raw(
                &String::from("This text goes a bit off the window"),
                (-2, 0),
                Color::WHITE,
                Color::BLACK
            );

            rdr.draw_rect((3, 5), (2, 6), Color::LIGHT_BLUE);
            rdr.draw_rect((7, 3), (2, 6), Color::LIGHT_BLUE);
            rdr.draw_rect((10, 3), (2, 6), Color::WHITE);
            rdr.draw_rect((12, 3), (2, 6), Color::RED);
            rdr.draw_rect((14, 3), (2, 6), Color::GREEN);
            rdr.draw_rect((16, 3), (2, 6), Color::BLUE);
            rdr.draw_rect((18, 3), (2, 6), Color::GRAY);
            rdr.draw_rect((20, 3), (2, 6), Color::DARK_KHAKI);
            rdr.draw_rect((22, 3), (2, 6), Color::CADET_BLUE);
            rdr.draw_rect((24, 3), (2, 6), Color::PINK);
            rdr.draw_rect((26, 3), (2, 6), Color::PURPLE);
            rdr.draw_rect((28, 3), (2, 6), Color::GAINSBORO);
            rdr.print_blended_text_raw(&String::from("I can draw text that will automagically change color to be readable"), (0, 4));

            rdr.draw_rect((20, 20), (49, 7), Color::DARK_RED);
            rdr.print_blended_text_raw(&dynamic_text, Vec2::from(dyn_text_pos));
            rdr.end_draw();
        }
    }
}
