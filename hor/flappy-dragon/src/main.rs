#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 33.0;

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 33 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player: Player,
    frame_time: f32,
    obstacles: Vec<Obstacle>,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacles: vec![Obstacle::new(SCREEN_WIDTH, 0)],
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.mode = GameMode::Playing;
        self.obstacles = vec![Obstacle::new(SCREEN_WIDTH, 0)];
        self.frame_time = 0.0;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        for obstacle in &mut self.obstacles {
            obstacle.render(ctx, self.player.x);
        }

        let before = self.obstacles.len();
        self.obstacles
            .retain(|obstacle| self.player.x <= obstacle.x);
        let after = self.obstacles.len();

        if after < before {
            self.obstacles
                .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }

        self.score += (before - after) as i32;

        for obstacle in &self.obstacles {
            if self.player.y > SCREEN_HEIGHT || obstacle.hit_obstacle(&self.player) {
                self.mode = GameMode::End;
            }
        }

        if self.obstacles.len() == 1
            && (SCREEN_WIDTH as f32 / (self.obstacles[0].x - self.player.x) as f32) > 1.25
        {
            self.obstacles
                .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }
        if self.obstacles.len() == 2
            && (SCREEN_WIDTH as f32 / (self.obstacles[1].x - self.player.x) as f32) > 1.25
        {
            self.obstacles
                .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }
        if self.obstacles.len() == 3
            && (SCREEN_WIDTH as f32 / (self.obstacles[2].x - self.player.x) as f32) > 1.25
        {
            self.obstacles
                .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
