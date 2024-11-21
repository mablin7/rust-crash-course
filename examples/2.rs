use rust_crash_course::Vector2;

#[derive(Debug, Clone, Copy)]
struct PlayerId(usize);

struct Player {
    id: PlayerId,
    pos: Vector2,
}

trait WorldView {
    fn get_players(&self) -> &[Player];

    fn get_closest_player(&self, pos: Vector2) -> Option<PlayerId> {
        // Task: Return the id of the player closest to the given position
        todo!()
    }
}

struct World {
    players: Vec<Player>,
}

impl WorldView for World {
    fn get_players(&self) -> &[Player] {
        &self.players
    }
}

struct PlayerCtx {
    player: Player,
    world: World,
}

impl WorldView for PlayerCtx {
    // Task: Implement the get_players method
}

fn main() {
    let player_ctx = PlayerCtx {
        player: Player {
            id: PlayerId(0),
            pos: Vector2 { x: 10.0, y: 20.0 },
        },
        world: World {
            players: vec![
                Player {
                    id: PlayerId(1),
                    pos: Vector2 { x: 30.0, y: 40.0 },
                },
                Player {
                    id: PlayerId(2),
                    pos: Vector2 { x: 50.0, y: 60.0 },
                },
            ],
        },
    };

    let closest_player = player_ctx.get_closest_player(Vector2 { x: 0.0, y: 0.0 });
}
