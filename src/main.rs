use eframe::egui;
use std::time::{Duration, Instant};

static WINDOW_WIDTH: f32 = 1280.0;
static WINDOW_HEIGHT: f32 = 720.0;
static UPDATE_RATE: Duration = Duration::from_millis(1000 / 60);

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

    fn update(&mut self) {
        self.position += self.velocity;
    }
}

struct CollisionApp {
    controlled_object_index: usize,  // alternative is storing the object using refcells which look complicated and messier
    objects: Vec<Object>,
    last_update_time: Instant,
}

impl CollisionApp {
    fn new() -> Self {
        let controlled_object = Object::new(
            50.0,
            50.0,
            egui::Pos2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            egui::Vec2::new(0.0, 1.0),
        );

        let objects = vec![controlled_object];

        Self { controlled_object_index: 0, objects: objects, last_update_time: Instant::now() }
    }

    fn draw_object(&self, ui: &mut egui::Ui, object: &Object, centred: bool) {
        let position = match centred {
            true => egui::pos2(object.position.x - object.width / 2.0, object.position.y - object.height / 2.0),
            false => object.position,
        };

        let rectangle = egui::Rect::from_min_size(position, egui::vec2(object.width, object.height));
        ui.painter().rect_filled(rectangle, 0.0, egui::Color32::WHITE);
    }
}

impl eframe::App for CollisionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello,");
            ui.label("There should be a moveable object here soon.");

            let current_time = Instant::now();
            if current_time - self.last_update_time >= UPDATE_RATE {
                for object in &mut self.objects {
                    object.update();
                }
                self.last_update_time = current_time;
            }

            for object in &self.objects {
                self.draw_object(ui, object, true);
            }

            ctx.request_repaint();
        });
    }
}
