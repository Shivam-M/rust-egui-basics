use eframe::egui;

fn main() -> eframe::Result<()> {
    return eframe::run_native(
        "GUI Input Test - moving an object around with basic collision detection",
        eframe::NativeOptions::default(),
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
