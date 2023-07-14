use std::path::PathBuf;

use eframe::egui::{self, TopBottomPanel, SidePanel, Ui, Sense, Label};

use crate::{LogicalBuilder, modals, project::{Project, ProjectEntry}};

pub fn top_panel(app: &mut LogicalBuilder, ctx: &egui::Context) {

    let new_shape_modal = modals::new_shape(ctx, app);

    app.occupied_sides.top = TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |menu| {
                let new_project = menu.button("New Project");
                let open_project = menu.button("Open Project");
                menu.separator();

                if app.project.is_some() {
                    menu.menu_button("New File", |sub_menu| {
                        let new_shape = sub_menu.button("Shape");

                        if new_shape.clicked() {
                            app.new_file_name = "".to_owned();
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
                        app.project = Project::create_new(&root).ok();
                    }

                    menu.close_menu();
                }

                if open_project.clicked() {
                    if let Some(root) = rfd::FileDialog::new().pick_folder() {
                        // some notification if project is invalid would be good
                        app.project = Project::from_root(&root).ok();
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

pub fn left_panel(app: &mut LogicalBuilder, ctx: &egui::Context) {
    app.occupied_sides.left = SidePanel::left("left_panel").resizable(true).show(ctx, |ui| {

        if let Some(project) = &app.project {

            ui.label(project.root().to_str().unwrap());
            ui.separator();

            let selected_entry = project_directory(app, ui, project.entries());
            
            if selected_entry.is_some() {
                dbg!(&selected_entry);
                app.open_file = selected_entry;
            }
        }

        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

    })
    .response
    .rect
    .width();
}

fn project_directory(app: &LogicalBuilder, ui: &mut Ui, project_dir: &ProjectEntry) -> Option<PathBuf> {

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
                            let directory_selection = project_directory(app, ui, child);

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

pub fn bottom_panel(app: &mut LogicalBuilder, ctx: &egui::Context) {
    app.occupied_sides.bottom = TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
        ui.label("ganz viel text einfach damit ich was testen kann");

        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .height();
}

pub fn right_panel(app: &mut LogicalBuilder, ctx: &egui::Context) {
    app.occupied_sides.right = SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut app.name)
                .labelled_by(name_label.id);
        });
        ui.add(egui::Slider::new(&mut app.age, 0..=120).text("age"));
        if ui.button("Click each year").clicked() {
            app.age += 1;
        }
        ui.label(format!("Hello '{}', age {}", app.name, app.age));
        
        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .width();
}