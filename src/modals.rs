use eframe::egui::{Context, self};
use egui_modal::Modal;

use crate::{LogicalBuilder, graphics};

pub fn new_shape(ctx: &Context, app: &mut LogicalBuilder) -> Modal {

    let modal = Modal::new(ctx, "New shape");

    modal.show(|ui| {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut app.new_file_name);
        });

        ui.with_layout(egui::Layout::right_to_left(eframe::emath::Align::TOP), |ui| {
            if ui.button("Create").clicked() {

                let mut new_shape_name = app.new_file_name.to_owned();
                new_shape_name.push_str(".shape");

                if let Some(project) = &mut app.project {
                    let new_shape_path = project.root()
                        .join("shapes")
                        .join(new_shape_name);

                    //todo handle this
                    let default_shape = graphics::new_default_shape();
                    let serialized_shape = graphics::path_shape_serialize(&default_shape);

                    let _ = project.new_file(&new_shape_path, &serialized_shape);
                }
                modal.close();
            }
            
            if ui.button("Close").clicked() {
                app.new_file_name = "".into();
                modal.close();
            }
        });
    });

    modal
}