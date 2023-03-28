use termion::{clear, cursor, raw::{RawTerminal, IntoRawMode}, input::TermRead, event::Key};
use std::{fs, env, io::{Write, stdout, Stdout, stdin}};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;
pub type Buffer = [[char; WIDTH]; HEIGHT];

const HORIZONTAL: char = '\u{2500}';
const VERTICAL: char = '\u{2502}';
const UPPER_LEFT: char = '\u{250C}';
const UPPER_RIGHT: char = '\u{2510}';
const LOWER_LEFT: char = '\u{2514}';
const LOWER_RIGHT: char = '\u{2518}';
const DOWN_HORIZONTAL: char = '\u{252C}';
const UP_HORIZONTAL: char = '\u{2534}';
const DOUBLE_HORIZONTAL: char = '\u{2550}';
const DOUBLE_VERTICAL: char = '\u{2551}';
const DOUBLE_UPPER_LEFT: char = '\u{2554}';
const DOUBLE_UPPER_RIGHT: char = '\u{2557}';
const DOUBLE_LOWER_LEFT: char = '\u{255A}';
const DOUBLE_LOWER_RIGHT: char = '\u{255D}';
const DOUBLE_DOWN_HORIZONTAL: char = '\u{2566}';
const DOUBLE_UP_HORIZONTAL: char = '\u{2569}';

pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct TextObject {
    pub text: String,
    pub position: Position,
    pub width: usize,
    pub height: Option<usize>
}

pub fn draw_buffer(buffer: Buffer, stdout: &mut RawTerminal<Stdout>) {
    for line in buffer.iter() {
        let mut output = String::from_iter(line);
        output.push_str("\r\n");
        write!(stdout, "{output}").unwrap();
    }
}

fn draw_borders(buffer: &mut Buffer) {
    draw_box(buffer, WIDTH-1, HEIGHT-1, Position { x: 0, y: 0 });
}

pub fn draw_row(buffer: &mut Buffer, size: usize) {
    if size <= 0 {
        panic!("Size cant be less than or equal zero");
    }

    draw_borders(buffer);
    let row_width = WIDTH/size;
    for i in 1..size {
        let line_position = i*row_width;
        buffer[0][line_position] = DOWN_HORIZONTAL;
        buffer[HEIGHT-1][line_position] = UP_HORIZONTAL;
        for j in 1..HEIGHT-1 {
            buffer[j][line_position] = VERTICAL;
        }
    }
}

pub fn draw_box(buffer: &mut Buffer, width: usize, height: usize, position: Position) {
    if position.x+width > buffer[0].len() || position.y+height > buffer.len() {
        panic!("Box out of bounds");
    }

    // Corners
    buffer[position.y][position.x] = UPPER_LEFT;
    buffer[position.y][position.x+width] = UPPER_RIGHT;
    buffer[position.y+height][position.x] = LOWER_LEFT;
    buffer[position.y+height][position.x+width] = LOWER_RIGHT;

    // Vertical
    for i in 1..height {
        buffer[i][position.x] = VERTICAL;
        buffer[i][position.x+width] = VERTICAL;
    }
    // Horizontal
    for i in 1..width {
        buffer[position.y][i] = HORIZONTAL;
        buffer[position.y+height][i] = HORIZONTAL;
    }
}

pub fn draw_double_box(buffer: &mut Buffer, width: usize, height: usize, position: Position) {
    if position.x+width > buffer[0].len() || position.y+height > buffer.len() {
        panic!("Box out of bounds");
    }

    // Corners
    buffer[position.y][position.x] = DOUBLE_UPPER_LEFT;
    buffer[position.y][position.x+width] = DOUBLE_UPPER_RIGHT;
    buffer[position.y+height][position.x] = DOUBLE_LOWER_LEFT;
    buffer[position.y+height][position.x+width] = DOUBLE_LOWER_RIGHT;

    // Vertical
    for i in position.y+1..position.y+height {
        buffer[i][position.x] = DOUBLE_VERTICAL;
        buffer[i][position.x+width] = DOUBLE_VERTICAL;
    }
    // Horizontal
    for i in position.x+1..position.x+width {
        buffer[position.y][i] = DOUBLE_HORIZONTAL;
        buffer[position.y+height][i] = DOUBLE_HORIZONTAL;
    }
}
pub fn draw_options(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "\r\n").unwrap();
    write!(stdout, "Commands:\r\n").unwrap();
    write!(stdout, "\r\n").unwrap();
    write!(stdout, "    h/l -> Cycle through selected columns\r\n").unwrap();
    write!(stdout, "    j/k -> Cycle through selected tasks\r\n").unwrap();
    write!(stdout, "    a -> Add task to selected column\r\n").unwrap();
    write!(stdout, "    H/L -> Move selected task to next column\r\n").unwrap();
    write!(stdout, "    D -> Remove selected task\r\n").unwrap();
    write!(stdout, "    Q -> Save and quit\r\n").unwrap();
}

pub fn clear_screen() {
    print!("{}{}{}", clear::All, cursor::Hide, cursor::Goto(1,1));
}

pub fn draw_text_object(buffer: &mut Buffer, mut text_object: &mut TextObject) {
    let mut line_index = text_object.position.y;
    let mut column_index = 0;
    for character in text_object.text.chars() {
        buffer[line_index][text_object.position.x + column_index] = character;
        column_index += 1;
        if column_index >= text_object.width {
            line_index += 1;
            column_index = 0;
        }
    }
    text_object.height = Some(line_index+1);
}
