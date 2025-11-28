pub struct View {
    width: u32,
    height: u32,
    zoom: f32,
    scroll: f32
}

impl View {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            zoom: 1.0,
            scroll: 0.0
        }
    }

    pub fn render(&self) {
        // to implement
        println!("Rendering!");
    }
}
