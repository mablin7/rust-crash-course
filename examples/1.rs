// Pattern matching and error handling

use rust_crash_course::Vector2;

struct Ball {
    pos: Vector2,
}

struct World {
    ball: Option<Ball>,
}

impl World {
    fn ball_offset(&self) -> Option<Vector2> {
        let offset = Vector2 { x: 100.0, y: 0.0 };
        self.ball.as_ref().map(|ball| ball.pos + offset)
    }

    fn try_kick(&self) -> Result<f64, String> {
        // Kicks the ball if it exists
        match self.ball.as_ref() {
            Some(_ball) => Ok(100.0),
            None => Err("No ball to kick".to_string()),
        }
    }
}

fn test() -> Result<(), String> {
    let world = World {
        ball: Some(Ball {
            pos: Vector2 { x: 10.0, y: 20.0 },
        }),
    };

    let v = world.try_kick()?;

    Ok(())
}

fn main() {
    let world = World {
        ball: Some(Ball {
            pos: Vector2 { x: 10.0, y: 20.0 },
        }),
    };

    let offset = world.ball_offset();
    println!("Offset ball position {:?}", offset);

    world.try_kick()
}
