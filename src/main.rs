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
        Box::new(|_cc| Ok(Box::new(CollisionApp {})))
    );
}

struct CollisionApp {}

impl eframe::App for CollisionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello,");
            ui.label("There should be a moveable object here soon.");
        });
    }
}
