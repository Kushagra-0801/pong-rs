use ggez;
use ggez::event::EventHandler;
use ggez::mint::Point2;
use ggez::GameResult;
use ggez::{conf, graphics};

const SCREEN_SIZE: (f32, f32) = (600.0, 500.0);
const BAT_SIZE: (f32, f32) = (10.0, 50.0);
const BALL_RADIUS: f32 = 10.0;

type Point = Point2<f32>;

#[derive(Debug, Clone)]
struct State {
    player1: Player,
    player2: Player,
    ball: Ball,
}

impl State {
    pub fn new() -> Self {
        State {
            player1: Player {
                bat: Point {
                    x: SCREEN_SIZE.0 / 50.0 * 3.0,
                    y: SCREEN_SIZE.1 / 50.0 * 5.0,
                },
                score: 0,
            },
            player2: Player {
                bat: Point {
                    x: SCREEN_SIZE.0 / 50.0 * 47.0 - BAT_SIZE.0,
                    y: SCREEN_SIZE.1 / 50.0 * 5.0,
                },
                score: 0,
            },
            ball: Ball {
                pos: Point {
                    x: SCREEN_SIZE.0 / 2.0,
                    y: SCREEN_SIZE.1 / 2.0,
                },
                vel: Point { x: 1.0, y: 1.0 },
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    bat: Point,
    score: u32,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
}

impl Player {
    fn update(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.bat.y -= 5.0,
            Dir::Down => self.bat.y += 5.0,
        }
        if self.bat.y < 0.0 {
            self.bat.y = 0.0;
        } else if self.bat.y > SCREEN_SIZE.1 - BAT_SIZE.1 {
            self.bat.y = SCREEN_SIZE.1 - BAT_SIZE.1;
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        let color = graphics::WHITE;
        let bat = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.bat.x,
                y: self.bat.y,
                w: BAT_SIZE.0,
                h: BAT_SIZE.1,
            },
            color,
        )?;
        graphics::draw(ctx, &bat, graphics::DrawParam::default())
    }
}

fn write_scores(state: &State, ctx: &mut ggez::Context) -> GameResult {
    let pl1_score = state.player1.score;
    let pl2_score = state.player2.score;
    let text = format!("{} | {}", pl1_score, pl2_score);
    let mut text = graphics::Text::new(text);
    text.set_font(graphics::Font::default(), graphics::Scale::uniform(35.0));
    let text_position = Point {
        x: SCREEN_SIZE.0 / 2.0 - (text.width(ctx) as f32 / 2.0).ceil(),
        y: 0.0,
    };
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default().dest(text_position),
    )
}

#[derive(Debug, Clone)]
struct Ball {
    pos: Point,
    vel: Point,
}

impl Ball {
    fn reset(&mut self) {
        self.pos = Point {
            x: SCREEN_SIZE.0 / 2.0,
            y: SCREEN_SIZE.1 / 2.0,
        };
        self.vel = Point { x: 1.0, y: 1.0 };
    }

    fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        if self.pos.y < 0.0 {
            self.pos.y *= -1.0;
            self.vel.y *= -1.0;
        } else if self.pos.y > SCREEN_SIZE.1 {
            self.pos.y -= 2.0 * (self.pos.y - SCREEN_SIZE.1);
            self.vel.y *= -1.0;
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            BALL_RADIUS,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &ball, graphics::DrawParam::default())
    }
}

fn check_and_update_collision(ball: &mut Ball, pl1: &mut Player, pl2: &mut Player) {
    let pl1_x_bound = pl1.bat.x + BAT_SIZE.0;
    let pl2_x_bound = pl2.bat.x;
    let pl1_y_bound_up = pl1.bat.y;
    let pl1_y_bound_down = pl1.bat.y + BAT_SIZE.1;
    let pl2_y_bound_up = pl2.bat.y;
    let pl2_y_bound_down = pl2.bat.y + BAT_SIZE.1;

    if ball.pos.x > pl1_x_bound && ball.pos.x < pl2_x_bound {
        return;
    }
    if ball.vel.x < 0.0 {
        if ball.pos.x <= pl1_x_bound
            && ball.pos.y >= pl1_y_bound_up
            && ball.pos.y <= pl1_y_bound_down
        {
            ball.vel.x *= -1.1;
            ball.vel.y *= 1.1;
        } else {
            pl2.score += 1;
            ball.reset();
        }
    } else {
        if ball.pos.x >= pl2_x_bound
            && ball.pos.y >= pl2_y_bound_up
            && ball.pos.y <= pl2_y_bound_down
        {
            ball.vel.x *= -1.1;
            ball.vel.y *= 1.1;
        } else {
            pl1.score += 1;
            ball.reset();
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::input::keyboard::{
            is_key_pressed,
            KeyCode::{Down, Up, Q, S, W},
        };
        if is_key_pressed(ctx, Up) {
            self.player2.update(Dir::Up);
        } else if is_key_pressed(ctx, Down) {
            self.player2.update(Dir::Down);
        }
        if is_key_pressed(ctx, W) {
            self.player1.update(Dir::Up);
        } else if is_key_pressed(ctx, S) {
            self.player1.update(Dir::Down);
        }
        if is_key_pressed(ctx, Q) {
            ggez::event::quit(ctx);
        }
        self.ball.update();
        check_and_update_collision(&mut self.ball, &mut self.player1, &mut self.player2);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, ggez::graphics::BLACK);
        self.player1.draw(ctx)?;
        self.player2.draw(ctx)?;
        self.ball.draw(ctx)?;
        write_scores(self, ctx)?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let state = &mut State::new();
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Hello-ggez", "Kushagra")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    ggez::event::run(&mut ctx, &mut event_loop, state)
}
