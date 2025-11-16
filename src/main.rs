use eframe::egui;
use std::time::{Duration, Instant};

static WINDOW_WIDTH: f32 = 1280.0;
static WINDOW_HEIGHT: f32 = 720.0;
static UPDATE_RATE: Duration = Duration::from_millis(1000 / 120);

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
    colour: egui::Color32,
}

impl Object {
    fn new(width: f32, height: f32, position: egui::Pos2, velocity: egui::Vec2, colour: egui::Color32) -> Self {
        Self { width, height, position, velocity, colour }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn overlaps_with(&self, other: &Object) -> bool {
        let rect_1 = egui::Rect::from_center_size(self.position, egui::vec2(self.width, self.height));
        let rect_2 = egui::Rect::from_center_size(other.position, egui::vec2(other.width, other.height));
        return rect_1.intersects(rect_2)
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
            egui::Vec2::new(0.0, 0.0),
            egui::Color32::LIGHT_RED,
        );

        let mut objects = vec![controlled_object];

        objects.push(Object::new(
            200.0,
            100.0,
            egui::Pos2::new(300.0, 300.0),
            egui::Vec2::new(0.0, 0.0),
            egui::Color32::LIGHT_BLUE,
        ));

        objects.push(Object::new(
            50.0,
            300.0,
            egui::Pos2::new(1000.0, 200.0),
            egui::Vec2::new(0.0, 0.0),
            egui::Color32::LIGHT_GREEN,
        ));

        objects.push(Object::new(
            350.0,
            250.0,
            egui::Pos2::new(200.0, 550.0),
            egui::Vec2::new(2.5, 0.0),
            egui::Color32::LIGHT_YELLOW,
        ));

        Self { controlled_object_index: 0, objects: objects, last_update_time: Instant::now() }
    }

    fn draw_object(&self, ui: &mut egui::Ui, object: &Object, centred: bool) {
        let position = match centred {
            true => egui::pos2(object.position.x - object.width / 2.0, object.position.y - object.height / 2.0),
            false => object.position,
        };

        let rectangle = egui::Rect::from_min_size(position, egui::vec2(object.width, object.height));
        ui.painter().rect_filled(rectangle, 0.0, object.colour);
    }

    // only resolves collisions for the controlled object
    fn resolve_collisions(&mut self) {
        let mut collided = false;
        
        for (index, object) in self.objects.iter().enumerate() {
            if index == self.controlled_object_index {
                continue;
            }

            if self.objects[self.controlled_object_index].overlaps_with(object) {
                collided = true;
            }
        }

        if collided {
            let controlled_object = &mut self.objects[self.controlled_object_index];
            controlled_object.position -= controlled_object.velocity;
            controlled_object.velocity = egui::Vec2::new(0.0, 0.0);
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        let controlled_object = &mut self.objects[self.controlled_object_index];
        let speed = 5.0;

        // maybe refactor to check which key was last pressed on each axis use that instead of stopping movement when both are pressed
        ctx.input(|input| {
            let left = input.key_down(egui::Key::ArrowLeft);
            let right = input.key_down(egui::Key::ArrowRight);

            controlled_object.velocity.x = match (left, right) {
                (true, false) => -speed,
                (false, true) => speed,
                _ => 0.0,
            };

            let up = input.key_down(egui::Key::ArrowUp);
            let down = input.key_down(egui::Key::ArrowDown);

            controlled_object.velocity.y = match (up, down) {
                (true, false) => -speed,
                (false, true) => speed,
                _ => 0.0,
            };
        });
    }
}

impl eframe::App for CollisionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello,");
            ui.label("There is a moveable object here (use the arrow keys to move it around)");

            let controlled_object_position = self.objects[self.controlled_object_index].position;
            ui.label(format!("Object position: ({:.2}, {:.2})", controlled_object_position.x, controlled_object_position.y));

            let current_time = Instant::now();
            if current_time - self.last_update_time >= UPDATE_RATE {
                self.handle_input(ctx);

                for object in &mut self.objects {
                    object.update();
                }

                self.resolve_collisions();

                self.last_update_time = current_time;
            }

            for object in &self.objects {
                self.draw_object(ui, object, true);
            }

            ctx.request_repaint();
        });
    }
}
