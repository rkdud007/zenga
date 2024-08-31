use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

use crate::rigid_body::{RigidBody, Shape};
use crate::vector::Vector;
use crate::world::World;

pub mod rigid_body;
pub mod vector;
pub mod world;

struct MainState {
    world: World,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let world = World::new(Vector::new(0.0, 9.81));
        Ok(MainState { world })
    }

    fn add_random_body(&mut self, x: f64, y: f64) {
        let mut rng = rand::thread_rng();
        let mass = rng.gen_range(1.0..5.0);
        let shape = if rng.gen_bool(0.5) {
            Shape::Circle {
                radius: rng.gen_range(10.0..30.0),
            }
        } else {
            Shape::Rectangle {
                width: rng.gen_range(20.0..60.0),
                height: rng.gen_range(20.0..60.0),
            }
        };
        let body = RigidBody::new(Vector::new(x, y), mass, shape);
        self.world.add_body(body);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.world.step(1.0 / 60.0); // 60 FPS
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for i in 0..self.world.body_count() {
            if let Some(body) = self.world.get_body(i) {
                let color = match body.shape {
                    Shape::Circle { .. } => Color::RED,
                    Shape::Rectangle { .. } => Color::GREEN,
                };

                match body.shape {
                    Shape::Circle { radius } => {
                        let circle = graphics::Mesh::new_circle(
                            ctx,
                            DrawMode::fill(),
                            Point2::from([body.position.x as f32, body.position.y as f32]),
                            radius as f32,
                            0.1,
                            color,
                        )?;
                        graphics::draw(ctx, &circle, DrawParam::default())?;
                    }
                    Shape::Rectangle { width, height } => {
                        let rect = graphics::Mesh::new_rectangle(
                            ctx,
                            DrawMode::fill(),
                            graphics::Rect::new(
                                body.position.x as f32 - width as f32 / 2.0,
                                body.position.y as f32 - height as f32 / 2.0,
                                width as f32,
                                height as f32,
                            ),
                            color,
                        )?;
                        graphics::draw(ctx, &rect, DrawParam::default())?;
                    }
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.add_random_body(x as f64, y as f64);
        }
    }
}

pub fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("physics_simulation", "pia")
        .window_setup(ggez::conf::WindowSetup::default().title("Physics Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
