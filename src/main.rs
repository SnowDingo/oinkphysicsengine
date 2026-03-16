use eframe::egui::{self, Pos2, Rect};

// data templates

struct Position {
    x: f64,
    y: f64,
}
struct Body {
    position: Position,
    velocity: f64,
}
struct App {
    body: Body,
    dt: f64,
}

const g: f64 = -9.81;

// process to execute each frame grouped together

// for egui I used https://hackmd.io/@Hamze/Sys9nvF6Jl to learn
fn step(body: &mut Body, dt: f64) {
    // the first kinematics equation applied here!
    if body.position.y > -200.0 {
        body.velocity += g * dt;
        body.position.y += body.velocity * dt;
    } else {
        
        body.velocity = 0.0;
    }
}

// returns result to prepare against any failure
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Oink Physics Sim",
        options,
        Box::new(|_cc_| {
            Ok(Box::new(App {
                body: Body {
                    position: Position { x: 200.0, y: 200.0 },
                    velocity: -10.0,
                },
                dt: 0.016,
            }))
        }),
    )?;
    Ok(())
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        step(&mut self.body, self.dt);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Oink physics simulation");
            ui.label(format!("Position: {:.2}", self.body.position.y));

            let cordinate = egui::Pos2::new(
                self.body.position.x as f32,
                300.0 - self.body.position.y as f32,
            );
            let painter = ui.painter();
            painter.circle_filled(cordinate, 12.0, egui::Color32::WHITE);
            // the floor:
            let floory=500.0;
            painter.line_segment([
                Pos2::new(0.0,floory as f32),
                Pos2::new(ui.available_width()+20.0,floory as f32)
            ], egui::Stroke::new(4.0, egui::Color32::WHITE))
        });
        
        ctx.request_repaint();
    }
}
