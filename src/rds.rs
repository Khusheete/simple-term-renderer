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


extern crate libc;

use crate::math::Vec2;
use crate::img::{Image, Color};
use crate::input::Input;

use termios::*;

use std::mem;

use std::io::{stdout, Write};

use std::thread;
use std::sync::{mpsc, Barrier, Arc, Mutex};

use std::io::stdin;
use std::os::unix::io::AsRawFd;

const NCCS: usize = 32;


/// csi macro rule
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1b[", $( $l ),*) };
}


/// Commands that are sent to the rendering server by the Renderer singleton.
enum RenderingDirective {
    DrawLine(Vec2, Vec2, Color),
    DrawRect(Vec2, Vec2, Color),
    DrawRectBoudary(Vec2, Vec2, Color),
    DrawEllipseBoudary(Vec2, Vec2, Color),
    DrawPoint(Vec2, Color),

    DrawImage(Arc<Mutex<Image>>, Vec2, Vec2, Vec2, Option<Color>),
    DrawWholeImageAlpha(Arc<Mutex<Image>>, Vec2, Color),
    DrawWholeImage(Arc<Mutex<Image>>, Vec2),

    ClearScreen(Color),

    UpdateScreenSize(Vec2),
    BeginFrame,
    PushFrame
}


/// This is the core of the library. It will send commands to the rendering server to print on screen.
/// 
/// # Usage
/// 
/// ```
/// // get the renderer
/// let rdr = Renderer::get();
/// 
/// ...
/// 
/// // start drawing on a frame
/// rdr.begin_draw();
/// 
/// ... // use drawing functions (eg. draw_rect, draw_point...)
/// 
/// rdr.end_draw(); // this pushes the frame to the screen
/// 
/// ...
/// 
/// Renderer::exit(); // to quit the program and reset terminal settings
/// ```
/// 
/// Screen coordinates start in the top left at (0, 0)
pub struct Renderer {
    termios: Termios,
    default_c_lflags: u32,
    default_c_cc: [u8; NCCS],

    building_frame: bool,
    prev_screen_size: Vec2,

    _server_handle: Option<thread::JoinHandle<()>>,
    sender: mpsc::Sender<RenderingDirective>,

    frame_barrier: Arc<Barrier>
}


/// Renderer singleton
static mut RENDERER: Option<Renderer> = None;


impl Renderer {

    /// Creates the Input singleton, will only be called once
    fn init() -> Renderer {
        let stdinfd = stdin().as_raw_fd();

        let mut termios = match Termios::from_fd(stdinfd) {
            Ok(t)  => t,
            Err(_) => panic!("Could not read stdin fd")
        };

        // save and update settings
        let default_c_lflags = termios.c_lflag;
        let default_c_cc = termios.c_cc;

        termios.c_lflag &= !(ECHO | ICANON | ISIG);
        termios.c_cc[VMIN] = 1;
        termios.c_cc[VTIME] = 0;

        tcsetattr(stdinfd, TCSANOW, &mut termios).expect("could not set stdin attributes");
        
        print!("{}{}", 
            csi!("?25l"),                                   // hide cursor
            csi!("?1049h")                                 // use alternate screen buffer
        );
        stdout().flush().expect("Could not write to stdout"); 

        // setup and start server
        let (rx, tx) = mpsc::channel();
        let barrier = Arc::new(Barrier::new(2));
        let frame_barrier = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            let mut screen_size = Renderer::get_size();
            let mut screen: Image = Image::new(0, 0);
            let mut prev_screen: Image = Image::new(0, 0);

            let mut back: Color = Color::BLACK;
            let mut fore: Color = Color::BLACK;
            print!("{:-}{:+}", back, fore);


            loop {
                match tx.recv().expect("RenderingServer channel was destroyed") {
                    RenderingDirective::DrawLine(p1, p2, c) => screen.line(p1, p2, c),
                    RenderingDirective::DrawRect(p, s, c) => screen.rect(p, s, c),
                    RenderingDirective::DrawRectBoudary(p, s, c) => screen.rect_boudary(p, s, c),
                    RenderingDirective::DrawEllipseBoudary(center, s, c) => screen.ellipse_boundary(center, s, c),
                    RenderingDirective::DrawPoint(p, c) => screen.point(p, c),

                    RenderingDirective::DrawImage(img, pos, size, off, alpha) => screen.image(&(*img.lock().unwrap()), pos, size, off, alpha),
                    RenderingDirective::DrawWholeImageAlpha(img, pos, alpha) => screen.whole_image_alpha(&(*img.lock().unwrap()), pos, alpha),
                    RenderingDirective::DrawWholeImage(img, pos) => screen.whole_image(&(*img.lock().unwrap()), pos),

                    RenderingDirective::ClearScreen(c) => screen.clear(c),

                    RenderingDirective::UpdateScreenSize(size) => {
                        screen_size = size;
                        screen.raw_resize(size); // TODO: raw_resize
                    }

                    RenderingDirective::BeginFrame => {frame_barrier.wait(); ()},
                    RenderingDirective::PushFrame => {
                        // position cursor
                        print!("\x1b[H");

                        let mut skiped = false;

                        for j in (0..screen_size.y).step_by(2) {
                            for i in 0..screen_size.x {
                                let pos1 = vec2!(i, j);
                                let pos2 = vec2!(i, j + 1);

                                if screen.size() == prev_screen.size() && screen[pos1] == prev_screen[pos1] && screen[pos2] == prev_screen[pos2] {
                                    skiped = true;
                                    continue;
                                }
                                
                                // update color
                                if screen[pos1] != back && screen[pos1] != fore && screen[pos2] == back {
                                    fore = screen[pos1];
                                    print!("{:+}", fore);
                                } else if screen[pos1] != back && screen[pos1] != fore && screen[pos2] == fore {
                                    back = screen[pos1];
                                    print!("{:-}", back);
                                } else if screen[pos2] != back && screen[pos2] != fore && screen[pos1] == back {
                                    fore = screen[pos2];
                                    print!("{:+}", fore);
                                } else if screen[pos2] != back && screen[pos2] != fore && screen[pos1] == fore {
                                    back = screen[pos2];
                                    print!("{:-}", back);
                                } else if screen[pos1] != back && screen[pos1] != fore && screen[pos2] != back && screen[pos2] != fore {
                                    fore = screen[pos1];
                                    back = screen[pos2];
                                    print!("{:+}", fore);
                                    print!("{:-}", back);
                                }

                                if skiped {
                                    print!("\x1b[{};{}H", j/2 + 1, i + 1);
                                    skiped = false;
                                }

                                // print pixel
                                if screen[pos1] == back && screen[pos2] == back {
                                    print!(" ");
                                } else if screen[pos1] == back && screen[pos2] == fore {
                                    print!("▄");
                                } else if screen[pos1] == fore && screen[pos2] == back {
                                    print!("▀");
                                } else if screen[pos1] == fore && screen[pos2] == fore {
                                    print!("█");
                                }
                            }
                        }
                        stdout().flush().expect("Could not write to stdout");
                        prev_screen = screen.clone();
                    }
                }
            }
        });

        Renderer {
            termios: termios,
            default_c_lflags: default_c_lflags,
            default_c_cc: default_c_cc,

            building_frame: false,
            prev_screen_size: Vec2::ZERO,

            _server_handle: Some(handle),
            sender: rx,

            frame_barrier: barrier
        }
    }


    /// Exits the program and reset terminal setttings (should be called before the program ends).
    pub fn exit() {
        unsafe {
            RENDERER = None;
        }
    }


    /// Returns the Renderer instance.
    pub fn get() -> &'static mut Renderer {
        unsafe {
            match &mut RENDERER {
                None => { // construct the renderer, and initialize
                    RENDERER = Some(Renderer::init());
                    Renderer::get()
                }
                Some(r) => r
            }
        }
    }


    /// Returns the screen dimension.
    /// ```
    /// let size = Renderer::get_size();
    /// 
    /// size.x // width of the screen
    /// size.y // height of the screen
    /// ```
    pub fn get_size() -> Vec2 {
        unsafe {
            let mut size: TermSize = mem::zeroed();
            libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size as *mut _);
            vec2!(size.col as i32, 2 * size.row as i32)
        }
    }

    
    /// panics if we are not in a draw loop
    fn can_draw(&self) {
        if !self.building_frame { panic!("drawing outside of a frame build (call begin_draw)"); }
    }


    /// Starts drawing a frame.
    /// 
    /// Will panic if called twice before an end_draw
    pub fn begin_draw(&mut self) {
        if self.building_frame {
            panic!("begin_draw called when already building a frame");
        }
        self.building_frame = true;
        let new_size = Renderer::get_size();
        if self.prev_screen_size != new_size {
            self.sender.send(RenderingDirective::UpdateScreenSize(new_size)).expect("Rendering thread stoped");
            self.prev_screen_size = new_size;
        }

        self.sender.send(RenderingDirective::BeginFrame).expect("Rendering thread stoped");
        self.frame_barrier.wait();
    }


    /// Ends drawing a frame and pushes it to the screen.
    pub fn end_draw(&mut self) {
        if !self.building_frame {
            panic!("end_draw called when already building a frame");
        }
        self.building_frame = false;
        self.sender.send(RenderingDirective::PushFrame).expect("Rendering thread stoped");
    }


    /// Sets all the pixels' color in the screen to `c`.
    pub fn clear_screen(&mut self, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::ClearScreen(c)).expect("Rendering thread stoped");
    }


    /// Draws a line of color `c` between `p1` and `p2`.
    pub fn draw_line<A, B>(&mut self, p1: A, p2: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawLine(*p1.as_ref(), *p2.as_ref(), c))
            .expect("Rendering thread stoped");
    }


    /// Draws a rectangle of color `c` and of size `s`. 
    /// `p` is the coordinate of the top left corner of the rectangle.
    pub fn draw_rect<A, B>(&mut self, p: A, s: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawRect(*p.as_ref(), *s.as_ref(), c))
            .expect("Rendering thread stoped");
    }


    /// Same as `draw_rect` but draws only the four sides of the rectangle.
    pub fn draw_rect_boundary<A, B>(&mut self, p: A, s: B, c: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawRectBoudary(*p.as_ref(), *s.as_ref(), c))
            .expect("Rendering thread stoped");
    }


    /// Draws an ellipse of color `col`. `c` is the center of the ellipse and `s` is the size of the rectangle
    /// in which the ellipse is inscribed.
    pub fn draw_ellipse_boundary<A, B>(&mut self, c: A, s: B, col: Color) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawEllipseBoudary(*c.as_ref(), *s.as_ref(), col))
            .expect("Rendering thread stoped");
    }


    /// Sets the color of the pixel at `p` to `c`.
    pub fn draw_point<A>(&mut self, p: A, c: Color) 
        where A: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawPoint(*p.as_ref(), c)).expect("Rendering thread stoped");
    }


    /// Draws an image at position `pos`. 
    /// 
    /// Negative size results in flipped image. Alpha is used to ignore a given color while drawing.
    pub fn draw_image<A, B, C>(&mut self, 
        img: Arc<Mutex<Image>>, pos: A, size: B, offset: C, alpha: Option<Color>) 
        where A: AsRef<Vec2>, B: AsRef<Vec2>, C: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawImage(img, *pos.as_ref(), *size.as_ref(), *offset.as_ref(), alpha))
            .expect("Rendering thread stoped");
    }


    /// Draws the whole image at `pos`, ignoring the color `alpha`.
    /// 
    /// Equivalent to:
    /// ```
    /// rdr.image(img, pos, img.size(), Vec2::ZERO, Some(alpha));
    /// ```
    pub fn draw_whole_image_alpha<A>(&mut self, img: Arc<Mutex<Image>>, pos: A, alpha: Color) 
        where A: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawWholeImageAlpha(img, *pos.as_ref(), alpha))
            .expect("Rendering thread stoped");
    }


    /// Draws the whole image at `pos`.
    /// 
    /// Equivalent to:
    /// ```
    /// rdr.image(img, pos, img.size(), Vec2::ZERO, None);
    /// ```
    pub fn draw_whole_image<A>(&mut self, img: Arc<Mutex<Image>>, pos: A) 
        where A: AsRef<Vec2>
    {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawWholeImage(img, *pos.as_ref())).expect("Rendering thread stoped");
    }



    /// Rings the terminal bell. Can only be called during the creation of a frame
    /// 
    /// Technical note: the bell will ring when calling `end_draw`
    pub fn ring_bell(&self) {
        self.can_draw();
        print!("\x07");
    }
}


impl Drop for Renderer {

    /// When the renderer singleton is droped, reset terminal settings and exit.
    fn drop(&mut self) {
        // return settings to default
        self.termios.c_cc = self.default_c_cc;
        self.termios.c_lflag = self.default_c_lflags;

        print!("{}{}",
            csi!("?25h"),                                   // show cursor
            csi!("?1049l")                                  // use main screen buffer
        );
        stdout().flush().expect("Could not write to stdout");
        Input::disable_mouse();

        std::process::exit(0);
    }
}


struct TermSize {
    row: libc::c_ushort,
    col: libc::c_ushort,
    _x : libc::c_ushort,
    _y : libc::c_ushort
}