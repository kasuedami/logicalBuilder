use std::path::PathBuf;

use eframe::{egui::{self, Rect, Ui}, epaint::{Color32, Pos2, Stroke, Vec2}};
use project::Project;

mod graphics;
mod modals;
mod panels;
mod project;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(680.0, 360.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<LogicalBuilder>::default()),
    )
}

#[derive(Default)]
struct OccupiedSides {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

pub struct LogicalBuilder {
    name: String,
    age: u32,
    occupied_sides: OccupiedSides,
    draw_area: Rect,
    grid_spacing: Vec2,
    gird_stroke: Stroke,
    project: Option<Project>,
    open_file: Option<PathBuf>,
    pub new_file_name: String,
}

impl Default for LogicalBuilder {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            occupied_sides: OccupiedSides::default(),
            draw_area: Rect::NOTHING,
            grid_spacing: Vec2::splat(10.0),
            gird_stroke: Stroke::new(1.0, Color32::DARK_GRAY),
            project: None,
            open_file: None,
            new_file_name: String::default(),
        }
    }
}

impl eframe::App for LogicalBuilder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // self.top_panel(ctx);
        // self.bottom_panel(ctx);
        // self.left_panel(ctx);  
        // self.right_panel(ctx);

        panels::top_panel(self, ctx);
        panels::bottom_panel(self, ctx);
        panels::left_panel(self, ctx);
        panels::right_panel(self, ctx);

        self.caluclate_draw_area(frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_editor(ui);
        });
    }
}

impl LogicalBuilder {

    fn draw_editor(&self, ui: &mut Ui) {
        self.draw_grid(ui);
    }

    fn draw_grid(&self, ui: &mut Ui) {
        let painter = ui.painter_at(self.draw_area);

        let mut current_height = 0.0;
        
        while current_height <= self.draw_area.height() {
            let start = self.offset_pos(0.0, current_height);
            let end = self.offset_pos(self.draw_area.width(), current_height);
            
            painter.line_segment([start, end], self.gird_stroke);
            current_height += self.grid_spacing.y;
        }

        let mut current_width = 0.0;

        while current_width <= self.draw_area.width() {
            let start = self.offset_pos(current_width, 0.0);
            let end = self.offset_pos(current_width, self.draw_area.height());
            
            painter.line_segment([start, end], self.gird_stroke);
            current_width += self.grid_spacing.x;
        }

    }

    fn caluclate_draw_area(&mut self, frame: &eframe::Frame) {
        let top_left = Pos2::new(self.occupied_sides.left, self.occupied_sides.top);
        let bottom_right = (frame.info().window_info.size - Vec2::new(self.occupied_sides.right, self.occupied_sides.bottom)).to_pos2();
        
        self.draw_area = Rect::from_min_max(top_left, bottom_right);
    }

    fn offset_pos(&self, x: f32, y: f32) -> Pos2 {
        Pos2::new(self.occupied_sides.left + x, self.occupied_sides.top + y)
    }
}
