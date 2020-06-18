use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;

pub struct Display {
    width: usize,
    height: usize,
    scale: usize,

    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    frame: Vec<u32>,
    context: sdl2::Sdl,
}

impl Display {
    pub fn new(width: usize, height: usize, scale: usize, title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, (width * scale) as u32, (height * scale) as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        Self {
            context: sdl_context,
            width: width,
            height: height,
            scale: scale,
            canvas: window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())
                .unwrap(),
            frame: vec![0; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.frame[y * self.width + x] = val;
    }

    pub fn update(&mut self) -> bool {
        let mut event_pump = self.context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }

        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        self.canvas.clear();
        for (xy, &pixel) in self.frame.iter().enumerate() {
            if pixel == 0 {
                continue;
            }

            self.canvas.set_draw_color(Color::RGB(20, 220, 20));

            let x = ((xy % self.width) * self.scale) as i32;
            let y = ((xy / self.width) * self.scale) as i32;
            let _ = self
                .canvas
                .fill_rect(Rect::new(x, y, self.scale as u32, self.scale as u32));
        }
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        true
    }
}
