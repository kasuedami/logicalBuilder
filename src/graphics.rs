use eframe::epaint::{PathShape, Stroke, Color32, pos2};
use ron::ser::PrettyConfig;

pub fn new_default_shape() -> PathShape {
    let points = vec![pos2(0.0, 0.0), pos2(100.0, 0.0), pos2(100.0, 100.0), pos2(0.0, 100.0)];
    let stroke = Stroke::new(5.0, Color32::WHITE);

    PathShape::closed_line(points, stroke)
}

pub fn path_shape_serialize(shape: &PathShape) -> String {
    ron::ser::to_string_pretty(shape, PrettyConfig::default()).unwrap()
}