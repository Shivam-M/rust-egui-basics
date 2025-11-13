use std::rc::Rc;
use std::cell::RefCell;
use eframe::egui;

static WINDOW_WIDTH: f32 = 1280.0;
static WINDOW_HEIGHT: f32 = 720.0;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..Default::default()
    };

    return eframe::run_native(
        "GUI Input Test - moving an object around with basic collision detection",
        options,
        Box::new(|_cc| Ok(Box::new(CollisionApp::new())))
    );
}

struct Object {
    width: f32,
    height: f32,
    position: egui::Pos2,
    velocity: egui::Vec2,
}

impl Object {
    fn new(width: f32, height: f32, position: egui::Pos2, velocity: egui::Vec2) -> Self {
        Self { width, height, position, velocity }
    }
}

struct CollisionApp {
    controlled_object_index: usize,  // alternative is storing the object using refcells which look complicated and messier
    objects: Vec<Object>,
}

impl CollisionApp {
    fn new() -> Self {
        let controlled_object = Object::new(
            50.0,
            50.0,
            egui::Pos2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            egui::Vec2::new(0.0, 0.0),
        );

        let objects = vec![controlled_object];

        Self { controlled_object_index: 0, objects }
    }
}

impl eframe::App for CollisionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello,");
            ui.label("There should be a moveable object here soon.");
        });
    }
}
