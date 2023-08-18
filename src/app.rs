use crate::canvas::{self, canvas::Canvas};

pub struct App {
    canvas: Canvas,
}

impl App {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.canvas.render(ctx)
    }
}
