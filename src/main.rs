#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Sense};
use epaint::Pos2;
use log::info;

fn main() -> Result<(), eframe::Error> {
    info!("Hello from egui-template!");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Debug, Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                eframe::egui::Vec2::new(ui.available_width(), ui.available_height()),
                Sense::hover(),
            );

            let hover_pos = response.hover_pos().unwrap_or_default();
            let horizontal_point_1 = Pos2 {
                x: 0.0,
                y: hover_pos.y,
            };
            let horizontal_point_2 = Pos2 {
                x: response.rect.width(),
                y: hover_pos.y,
            };
            let vertical_point_1 = Pos2 {
                x: hover_pos.x,
                y: 0.0,
            };
            let vertical_point_2 = Pos2 {
                x: hover_pos.x,
                y: response.rect.height(),
            };
            // Draw the line
            painter.line_segment(
                [horizontal_point_1, horizontal_point_2],
                egui::Stroke::new(1.0, egui::Color32::RED),
            );
            painter.line_segment(
                [vertical_point_1, vertical_point_2],
                egui::Stroke::new(1.0, egui::Color32::RED),
            );
        });
    }
}
