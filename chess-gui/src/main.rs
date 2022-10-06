use ggez::graphics::Color;
/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Edited: Isak Larsson <isaklar@kth.se>
 * Last updated: 2022-09-28
 */
use jblomlof_chess::Game;

use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::{collections::HashMap, path};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const HIGHLIGHT: graphics::Color =
    graphics::Color::new(40.0 / 255.0, 90.0 / 255.0, 80.0 / 255.0, 0.3);

pub const NONE: u8 = 0;
pub const KING: u8 = 1;
pub const QUEEN: u8 = 2;
pub const BISHOP: u8 = 3;
pub const KNIGHT: u8 = 4;
pub const ROOK: u8 = 5;
pub const PAWN: u8 = 6;

#[derive(Debug, Copy, Clone)]
struct Piece {
    role: u8,
    position: (i16, i16),
    is_white: bool,
}

impl Piece {
    fn new(role: u8, position: (i16, i16), is_white: bool) -> Self {
        Piece {
            role,
            position,
            is_white,
        }
    }
}

/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<(bool, u8), graphics::Image>,
    // Example board representation.
    board: [[Option<Piece>; 8]; 8],
    // Imported game representation.
    game: Game,
    // places to highlight
    highlight_poses: Vec<(usize, usize)>,
    // which piece is being choosed
    highlight_piece: Option<Piece>,
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            board: [[None; 8]; 8],
            game: Game::new(),
            highlight_poses: Vec::new(),
            highlight_piece: None,
        };

        Ok(state)
    }

    fn load_board(&mut self) -> () {
        let board_str: String = Game::get_board(&self.game);

        for i in 0..8 {
            for j in 0..8 {
                let piece = board_str.chars().nth(i * 8 + (j + i)).unwrap(); // + 2i to skip "\n"

                if piece != '*' {
                    let is_white = piece.is_uppercase();
                    let role = match piece.to_ascii_lowercase() {
                        'k' => KING,
                        'q' => QUEEN,
                        'b' => BISHOP,
                        'n' => KNIGHT,
                        'r' => ROOK,
                        'p' => PAWN,
                        _ => NONE, // Should never happen
                    };
                    self.board[i][j] = Some(Piece::new(role, (i as i16, j as i16), is_white));
                } else {
                    self.board[i][j] = None;
                }
            }
        }
    }

    fn to_file_rank(&self, _column: usize, _row: usize) -> String {
        let files: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let rank: String = (7 - _column + 1).to_string();
        let mut file: String = files[_row].to_string();

        file.push_str(&rank);
        file
    }

    fn to_row_column(&self, filerank: &str) -> (usize, usize) {
        let chars: Vec<char> = String::from(filerank).chars().collect::<Vec<char>>();

        let column: usize = match chars[0] {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => 99,
        };

        let row: usize = 7 - (chars[1].to_digit(10).expect("to row column place") as usize - 1);

        (column, row)
    }

    fn to_tuple_moves(&self, _moves: Vec<String>) -> Vec<(usize, usize)> {
        let mut tuple_moves: Vec<(usize, usize)> = Vec::new();

        for i in 0.._moves.len() {
            tuple_moves.push(self.to_row_column(&_moves[i]));
        }

        tuple_moves
    }

    fn add_color(&self, _color1: Color, _color2: Color) -> Color {
        let r: f32 = _color1.r + _color2.r;
        let g: f32 = _color1.g + _color2.g;
        let b: f32 = _color1.b + _color2.b;

        graphics::Color::new(r, g, b, 1.0)
    }

    #[rustfmt::skip] // Skips formatting on this function (not recommended)
                     /// Loads chess piese images into hashmap, for ease of use.
    fn load_sprites(ctx: &mut Context) -> HashMap<(bool, u8), graphics::Image> {
        [
            ((false, KING), "/black_king.png".to_string()),
            ((false, QUEEN), "/black_queen.png".to_string()),
            ((false, ROOK), "/black_rook.png".to_string()),
            ((false, PAWN), "/black_pawn.png".to_string()),
            ((false, BISHOP), "/black_bishop.png".to_string()),
            ((false, KNIGHT), "/black_knight.png".to_string()),
            ((true, KING), "/white_king.png".to_string()),
            ((true, QUEEN), "/white_queen.png".to_string()),
            ((true, ROOK), "/white_rook.png".to_string()),
            ((true, PAWN), "/white_pawn.png".to_string()),
            ((true, BISHOP), "/white_bishop.png".to_string()),
            ((true, KNIGHT), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| {
                (*piece, graphics::Image::new(ctx, path).unwrap())
            })
            .collect::<HashMap<(bool, u8), graphics::Image>>()
    }
}

// This is where we implement the functions that ggez requires to function
impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    /// It won't be necessary to touch this unless you are implementing something that's not triggered by the user, like a clock
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.load_board();

        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                text_dimensions.w as f32 + 16.0,
                text_dimensions.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");

        // draw grid
        for row in 0..8 {
            for col in 0..8 {
                // draw tile
                let mut color = match col % 2 {
                    0 => {
                        if row % 2 == 0 {
                            WHITE
                        } else {
                            BLACK
                        }
                    }
                    _ => {
                        if row % 2 == 0 {
                            BLACK
                        } else {
                            WHITE
                        }
                    }
                };
                if self.highlight_poses.contains(&(col as usize, row as usize)) {
                    color = self.add_color(color, HIGHLIGHT);
                }

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        col * GRID_CELL_SIZE.0 as i32,
                        row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    color,
                )
                .expect("Failed to create tile.");

                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                // draw piece
                if let Some(piece) = self.board[row as usize][col as usize] {
                    graphics::draw(
                        ctx,
                        self.sprites.get(&(piece.is_white, piece.role)).unwrap(),
                        graphics::DrawParam::default()
                            .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                col as f32 * GRID_CELL_SIZE.0 as f32,
                                row as f32 * GRID_CELL_SIZE.1 as f32,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }
            }
        }

        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            // println!("xy: {}, {}", x, y);
            // println!("xy: {}, {}", x / 90.0, y / 90.0);

            let board_row: usize = (x / 90.0) as usize; // left is 0, right is 7
            let board_column: usize = (y / 90.0) as usize; // Top is 0 bottom is 7
            // println!("pressed: rowboard{}, {}", board_row, board_column);

            let tmp = self.to_file_rank(board_row, board_column);
            let tmp2 = !self.board[board_column][board_row].is_none();
            println!("Filerank: {}, there is a piece: {}", tmp, tmp2);

            if self.highlight_poses.contains(&(board_row, board_column)) {
                println!(
                    "from: {}, to: {}",
                    self.highlight_piece.unwrap().position.0 as usize,
                    self.highlight_piece.unwrap().position.1 as usize
                );
                self.game.make_move(
                    &self.to_file_rank(
                        self.highlight_piece.unwrap().position.0 as usize,
                        self.highlight_piece.unwrap().position.1 as usize,
                    ),
                    &self.to_file_rank(board_column, board_row),
                );
                self.highlight_piece = None;
                self.highlight_poses = Vec::new();
            } else if !self.board[board_column][board_row].is_none() {
                // println!("first thing");

                self.highlight_poses = Vec::new();

                let piece = self.board[board_column][board_row].unwrap();
                // println!("role: {}, is white: {}", piece.role, piece.is_white);

                if piece.is_white == self.game.is_white_turn() {
                    let file_rank = self.to_file_rank(board_column, board_row);

                    // println!("Filerank: {}", file_rank);
                    let moves = self.game.get_possible_moves(&file_rank);

                    if !moves.is_none() {
                        self.highlight_poses = self.to_tuple_moves(moves.unwrap());
                        self.highlight_piece = self.board[board_column][board_row];
                        // TODO: convert it to positions I can use
                    }
                }

                for item in &self.highlight_poses {
                    println!("can move to {}, {}", item.0, item.1);
                }
            }
            /* check click position and update board accordingly */
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "viola")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Schack") // Set window title "Schack"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        );
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}
