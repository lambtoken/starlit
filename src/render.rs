use raylib::prelude::*;

pub struct View {
    rl: RaylibHandle,
    thread: RaylibThread,
    width: i32,
    height: i32,
    zoom: f32,
    scroll: f32
}

impl View {
    pub fn new(w: i32, h: i32) -> Self {
        let (mut rl, thread ) = raylib::init()
            .size(w, h)
            .title("Starlit")
            .build();

        Self {
            rl,
            thread,
            width: w,
            height: h,
            zoom: 1.0,
            scroll: 0.0
        }
    }

    pub fn render(&mut self) {
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);

            d.clear_background(Color::WHITE);

            d.draw_text("Starlit", 12, 12, 20, Color::BLACK);
        }
    }
}
