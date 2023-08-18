use egui::{epaint::RectShape, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2};

use super::field::Field;

pub struct Canvas {
    field: Field,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            field: Field::new(1920 / 2, 1080 / 2),
        }
    }

    pub fn render(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = self.field.get_width();
            let height = self.field.get_height();

            let (mut response, painter) =
                ui.allocate_painter(Vec2::new(width as f32, height as f32), Sense::hover());

            let mut shp = RectShape {
                rect: Rect::NOTHING,
                rounding: Rounding::none(),
                fill: Color32::BLUE,
                stroke: Stroke::NONE,
            };

            let len = (width * height) as usize;
            for i in 0..len {
                let x = (i as u16 % width) as f32;
                let y = (i as u16 / width) as f32;

                shp.rect.min = Pos2 { x, y };
                shp.rect.max = Pos2 {
                    x: x + 1.0,
                    y: y + 1.0,
                };

                shp.fill = Color32::from_rgb(0, 0, 0);

                painter.add(Shape::Rect(shp));
            }

            response.mark_changed();
            response
        });
    }
}
