use std::{f32::consts, time::Instant};

use colors_transform::{Color, Hsl};
use egui::{epaint::RectShape, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2};

use super::field::Field;

pub struct Canvas {
    field: Field,
    scale: f32,
}

impl Canvas {
    pub fn new(scale: f32) -> Self {
        Self {
            field: Field::new(1920 / 2, 1080 / 2),
            scale,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = self.field.get_width() as f32 * self.scale;
            let height = self.field.get_height() as f32 * self.scale;

            let (_, painter) =
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
                if cell.died {
                    continue;
                }

                let x = *x as f32 * self.scale;
                let y = *y as f32 * self.scale;

                shp.rect.min = Pos2 {
                    x: x - 0.25 * self.scale,
                    y: y - 0.25 * self.scale,
                };
                shp.rect.max = Pos2 {
                    x: x + 0.5 * self.scale,
                    y: y + 0.5 * self.scale,
                };

                let rgb =
                    Hsl::from(cell.get_dna() * 360.0, 100.0, cell.get_energy() * 50.0).to_rgb();

                shp.fill = Color32::from_rgb(
                    rgb.get_red() as u8,
                    rgb.get_green() as u8,
                    rgb.get_blue() as u8,
                );

                painter.add(Shape::Rect(shp));
            }
            self.field.update();

            ctx.request_repaint()
        });
    }
}
