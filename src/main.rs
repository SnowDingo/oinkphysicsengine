use eframe::egui::{self, Pos2, Rect};

// data templates

struct Position {
    x: f64,
    y: f64,
}
struct Velocity{
    x:f64,
    y:f64,
}
struct Body {
    position: Position,
    velocity: Velocity,
}
struct App {
    body: Body,
    dt: f64,
}

const g: f64 = -9.81;
// the bounciness 
const e: f64 = 0.6;



// process to execute each frame grouped together

// for egui I used https://hackmd.io/@Hamze/Sys9nvF6Jl to learn
fn step(body: &mut Body, dt: f64) {
    // the first kinematics equation applied here!
    // This is called euiler's integration formula. 
    //  Basically by using dt we approximate the next state. 
    // I decided to first compute these equations because then we have to do the calculation to stop the floor noclip glitch.
    body.velocity.y += g * dt;
    body.position.y += body.velocity.y * dt;
    
    // basically I derived -188 by doing -200(the floor's position - the ball's radius)
    // the second part of the ineqality is like clamping or setting the max number of calculations so the ball doesn't do the infinite bounces.
    if body.position.y < -188.0 &&  body.velocity.y.abs() >=0.01 {
        body.velocity.y= -e*body.velocity.y;
        body.position.y=-188.0;
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
                    velocity: Velocity{x:0.0,y:-10.0},
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

            if ui.button("Restart").clicked() {
                self.body.velocity.y=-10.0;
                self.body.position.y=200.0;
                self.body.position.x=200.0;
            }

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
