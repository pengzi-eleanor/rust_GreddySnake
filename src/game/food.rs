// food.rs
use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, MeshBuilder};
use rand::Rng;

pub struct Food {
    pub position: (f32, f32),
    pub direction: (f32, f32),
    pub is_moving: bool,
}

impl Food {
    pub fn new(is_moving: bool) -> GameResult<Food> {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..=31) as f32;
        let b = rng.gen_range(0..=23) as f32;
        let x = a * 20.0;
        let y = b * 20.0;
        
        let direction = if is_moving {
            // Random direction (either horizontal or vertical)
            if rng.gen_bool(0.5) {
                (10.0, 0.0)
            } else {
                (0.0, 10.0)
            }
        } else {
            (0.0, 0.0)
        };
        
        Ok(Food { 
            position: (x, y),
            direction,
            is_moving,
        })
    }

    pub fn update(&mut self) {
        if !self.is_moving {
            return;
        }
        
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        
        // Change direction if hitting boundaries
        if self.position.0 < 0.0 || self.position.0 >= 640.0 {
            self.direction.0 *= -1.0;
            self.position.0 = self.position.0.max(0.0).min(620.0);
        }
        if self.position.1 < 0.0 || self.position.1 >= 480.0 {
            self.direction.1 *= -1.0;
            self.position.1 = self.position.1.max(0.0).min(460.0);
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.position.0, self.position.1, 20.0, 20.0),
            Color::RED,
        )?;
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }
}