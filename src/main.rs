use std::path::PathBuf;

use eframe::{egui::{self, Rect, Ui, TopBottomPanel, SidePanel, Label, Sense}, epaint::{Color32, Pos2, Stroke, Vec2}};
use project::{Project, ProjectEntry};

mod graphics;
mod modals;
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

        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);  
        self.right_panel(ctx);

        self.caluclate_draw_area(frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_editor(ui);
        });
    }
}

impl LogicalBuilder {
    fn top_panel(&mut self, ctx: &egui::Context) {

        let new_shape_modal = modals::new_shape(ctx, self);

        self.occupied_sides.top = TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |menu| {
                    let new_project = menu.button("New Project");
                    let open_project = menu.button("Open Project");
                    menu.separator();

                    if self.project.is_some() {
                        menu.menu_button("New File", |sub_menu| {
                            let new_shape = sub_menu.button("Shape");
    
                            if new_shape.clicked() {
                                self.new_file_name = "".to_owned();
                                new_shape_modal.open();
                                sub_menu.close_menu();
                            }
                        });
                    }

                    menu.separator();
                    let _ = menu.button("Save");
                    let _ = menu.button("Save As");

                    if new_project.clicked() {
                        if let Some(root) = rfd::FileDialog::new().pick_folder() {
                            // some notification if project is invalid would be good
                            self.project = Project::create_new(&root).ok();
                        }

                        menu.close_menu();
                    }

                    if open_project.clicked() {
                        if let Some(root) = rfd::FileDialog::new().pick_folder() {
                            // some notification if project is invalid would be good
                            self.project = Project::from_root(&root).ok();
                        }

                        menu.close_menu();
                    }
                })
            })
        })
        .response
        .rect
        .height();
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        self.occupied_sides.left = SidePanel::left("left_panel").resizable(true).show(ctx, |ui| {

            if let Some(project) = &self.project {

                ui.label(project.root().to_str().unwrap());
                ui.separator();

                let selected_entry = self.project_directory(ui, project.entries());
                
                if selected_entry.is_some() {
                    dbg!(&selected_entry);
                    self.open_file = selected_entry;
                }
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

        })
        .response
        .rect
        .width();
    }

    fn project_directory(&self, ui: &mut Ui, project_dir: &ProjectEntry) -> Option<PathBuf> {

        let mut selected_file = None;

        match project_dir {
            ProjectEntry::File(_) => todo!(),
            ProjectEntry::Directory(_, children) => {
                ui.collapsing(project_dir.name(), |ui| {

                    for child in children {
                        match child {
                            ProjectEntry::File(path) => {
                                let label = Label::new(child.name()).sense(Sense::click());
                                if ui.add(label).clicked() {
                                    selected_file = Some(path.to_owned());
                                }
                            },
                            ProjectEntry::Directory(_, _) => {
                                let directory_selection = self.project_directory(ui, child);

                                if directory_selection.is_some() {
                                    selected_file = directory_selection;
                                }
                            },
                        }
                    }
                });

                selected_file
            },
        }

    }

    fn bottom_panel(&mut self, ctx: &egui::Context) {
        self.occupied_sides.bottom = TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
            ui.label("ganz viel text einfach damit ich was testen kann");

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    }

    fn right_panel(&mut self, ctx: &egui::Context) {
        self.occupied_sides.right = SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    }

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
