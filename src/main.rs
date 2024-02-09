use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

struct Player {
    rect: Rect, 
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 50f32,    // updated wid..
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        //let mut x_move = 0f32;
        // if is_key_down(KeyCode::Left) {
        //     x_move -= 1f32;
        // }
        // if is_key_down(KeyCode::Right) {
        //     x_move += 1f32;
        // }
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y), 
            lives: 2,
        }
    }
    pub fn draw(&self) {
        let color = match self.lives {
            3 => GREEN,
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

fn iscollide(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = match a.intersect(*b) {     // Early exit if no intersection... 
        Some(intersection) => intersection,
        None => return false,
    };
    // vel.y *= -1f32;
    // return true;
    let a_center = a.point() + a.size() * 0.5f32;
    let b_centre = b.point() + b.size() * 0.5f32;
    let to = b_centre - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce to y dir...
            a.y -= to_signum.y * intersection.h;    // push the block "a" by "h" on "y-axis"...
            vel.y = (- to_signum.y) * vel.y.abs();  // signam is form dir.. (b - a) bcs of that -ve of that is taken to make di..r +ve
        }
        false => {
            // bounce to x dir...
            a.x -= to_signum.x * intersection.w;
            vel.x = (- to_signum.x) * vel.x.abs();
        }
    }
    true // last line without semicolon ll always act as return value...
}

pub fn give_txt(text: &str, font: Font) {
    let menutext = measure_text(text, Some(font), 50u16, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - menutext.width * 0.5f32 , 
        screen_height()*  0.5f32 - menutext.height * 0.5f32,
        TextParams { font: font, font_size: 50u16, color: BLACK, ..Default::default() }
    );
}

fn reset_game(score: &mut i32, player_life: &mut i32, blocks: &mut Vec<Block>, balls: &mut Vec<Ball>, player: &mut Player) {
    *player = Player::new();
    *score = 0;
    *player_life = 3;
    balls.clear();
    init_blocks(blocks);
}

fn still_alive(_blocks: &mut Vec<Block>, balls: &mut Vec<Ball>, player: &mut Player) {
    *player = Player::new();
    balls.clear();
    //init_blocks(blocks);
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (10, 6);
    let pad = 5f32;
    let total_bsize = BLOCK_SIZE + vec2(pad, pad); 
    let board_start_pos = vec2((screen_width() - (total_bsize.x * width as f32)) * 0.5f32, 50f32); 
    for i in 0..(width * height) {
        let block_x = (i % width) as f32 * total_bsize.x;
        let block_y = (i / width) as f32 * total_bsize.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }
} 

#[macroquad::main("BreakOut")]
async fn main() {
    let font = load_ttf_font("D:/vcode/breakout/src/slkscr.ttf").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_life = 3;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls: Vec<Ball> = Vec::new(); 

    init_blocks(&mut blocks);   // this fun.. setsup the blocks into board...
    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 80f32))); // to initate the first ball.. in motion...

    loop {
        match game_state {
            GameState::Menu => { // moving from Menu to Game state by pressing SPACE...
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 80f32))); // to initate the first ball.. in motion...
                }
            },
            GameState::Game => {
                // if is_key_down(KeyCode::Space) {
                //     balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
                // }
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    iscollide(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if iscollide(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                        //    balls.push(Ball::new(vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32, -50f32)))
                            if block.lives <= 0{
                                score += 10;
                                spawn_later.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 80f32)));
                            }
                        }
                    }
                }
                for later_ball in spawn_later.into_iter() {     // .into_iter() will drop out the value before moving to next index... leaving us nothing...
                    balls.push(later_ball);
                }
        
                let balls_len = balls.len();
                //let last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height()); // ball disappear logic...
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_life -= 1;
                    if player_life > 0 && balls.is_empty() {
                        still_alive(&mut blocks, &mut balls, &mut player);
                    //    game_state = GameState::Menu;
                        balls.clear();
                        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 80f32))); // to initate the first ball.. in motion...
                        game_state = GameState::Menu;
                    }
                    if player_life <= 0  {   // moving to dead state is all lives are over...
                        game_state = GameState::Dead; 
                    }
                }
                blocks.retain(|block| block.lives > 0); // this fun revmoves ele of the index if the condition is not met...
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            },
            GameState::LevelCompleted => {  // moving from completed to Menu state
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_life, &mut blocks, &mut balls, &mut player); // reseting the game state 
                }
            },
            GameState::Dead => {    // moving from Dead to Menu state
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_life, &mut blocks, &mut balls, &mut player); // reseting the game state
                    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 80f32))); // to initate the first ball.. in motion...
                }
            },
        }
        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {    // spawninf the vector of blocks...
            block.draw();
        }
        for ball in balls.iter() {      // capturing the motion of each ball.. 
            ball.draw();
        }

        // this game state match allows to display all kinds of texts like "Press SPACE to start" throught the game...
        match game_state {
            GameState::Menu => {
                give_txt(&format!("Press SPACE to start"), font);
            },
            GameState::Game => {
                        // to allign the score at centre...
                let score_txt = format!("score : {}", score);
                let score_txt_dim = measure_text(&score_txt, Some(font), 25u16, 1.0);
                // rendering the score to the screen 
                draw_text_ex(
                    //&format!("score {}", score),
                    &score_txt, 
                    screen_width() * 0.5f32 - score_txt_dim.width *  0.5f32,
                    40.0,
                    TextParams { font: font, font_size: 25u16, color: DARKGREEN, ..Default::default() }
                );
                draw_text_ex(
                    &format!("lives : {}", player_life),
                    30.0,
                    40.0,
                    TextParams { font: font, font_size: 25u16, color: PINK, ..Default::default() }
                );
            },
            GameState::LevelCompleted => {
                give_txt(&format!("You win! {}\n score ", score), font);
            },
            GameState::Dead => {
                give_txt(&format!("You DIed"), font);
            },
        }
        request_new_screen_size(1000f32, 700f32);
        next_frame().await
    }
}