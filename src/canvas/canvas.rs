use egui::{epaint::RectShape, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2};

use super::field::Field;

pub struct Canvas {
    field: Field,
    scale: f32,
}

impl Canvas {
    pub fn new(scale: f32) -> Self {
        Self {
            field: Field::new(1920 / 4, 1080 / 4),
            scale,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.field.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            let width = self.field.get_width() as f32 * self.scale;
            let height = self.field.get_height() as f32 * self.scale;

            let (mut response, painter) =
                ui.allocate_painter(Vec2::new(width as f32, height as f32), Sense::hover());

            let mut shp = RectShape {
                rect: Rect::NOTHING,
                rounding: Rounding::none(),
                fill: Color32::BLUE,
                stroke: Stroke::NONE,
            };

            shp.rect.min = Pos2 { x: 0.0, y: 0.0 };
            shp.rect.max = Pos2 {
                x: width,
                y: height,
            };

            shp.fill = Color32::from_rgb(0, 0, 0);

            painter.add(Shape::Rect(shp));
            for ((x, y), cell) in self.field.get_data().iter() {
                let x = *x as f32 * self.scale;
                let y = *y as f32 * self.scale;

                shp.rect.min = Pos2 { x: x, y: y };
                shp.rect.max = Pos2 {
                    x: x + 1.0,
                    y: y + 1.0,
                };

                shp.fill = Color32::from_rgb(255, 255, 255);

                painter.add(Shape::Rect(shp));
            }

            response.mark_changed();
            response
        });
    }
}
