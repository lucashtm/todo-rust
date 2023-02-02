use std::cmp;
use serde::{Deserialize, Serialize};
use std::fs;

const WIDTH: usize = 80;
const HEIGHT: usize = 24;

type Buffer = [[char; WIDTH]; HEIGHT];

struct Position {
    x: usize,
    y: usize
}

struct TextObject {
    text: String,
    position: Position,
    width: usize
}

#[derive(Serialize, Deserialize)]
struct JsonData {
    todo: Vec<String>,
    doing: Vec<String>,
    done: Vec<String>,
}

fn draw_buffer(buffer: Buffer) {
    for line in buffer.iter() {
        println!("{}", String::from_iter(line));
    }
}

fn clear_screen() {
    print!("\x2B[2J\x1B[1;1H");
}

fn draw_text_object(buffer: &mut Buffer, text_object: TextObject) {
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
}

fn draw_column_title(buffer: &mut Buffer, column_index: usize, text: &String) {
    let column_width = WIDTH / 3;
    let column_title_text_object = TextObject {
        text: text.to_string(),
        position: Position { x: column_index*column_width + column_width/2 - text.len()/2, y: 1 },
        width: column_width
    };
    draw_text_object(buffer, column_title_text_object);
}

fn draw_text_at_column(buffer: &mut Buffer, column_index: usize, text: &String) {
    let text_object = TextObject {
        text: text.to_string(),
        position: Position { x: column_index*WIDTH/3 + 2, y: 3 },
        width: WIDTH/3 - 3
    };
    draw_text_object(buffer, text_object);
}

fn main() {

    let mut buffer: Buffer = [[' '; WIDTH]; HEIGHT];

    
    buffer[0][0] = '*';
    buffer[0][WIDTH-1] = '*';
    buffer[HEIGHT-1][0] = '*';
    buffer[HEIGHT-1][WIDTH-1] = '*';

    for i in 1..HEIGHT-1 {
        buffer[i][0] = '|';
        buffer[i][WIDTH-1] = '|';
        buffer[i][WIDTH/3] = '|';
        buffer[i][2*WIDTH/3] = '|';
    }
    for i in 1..WIDTH-1 {
        buffer[0][i] = '-';
        buffer[HEIGHT-1][i] = '-';
    }

    draw_column_title(&mut buffer, 0, &String::from("TODO"));
    draw_column_title(&mut buffer, 1, &String::from("DOING"));
    draw_column_title(&mut buffer, 2, &String::from("DONE"));

    let data = fs::read_to_string("/home/lucas/Dev/todo-rust/src/data.json").expect("Couldnt open file");
    let data: JsonData = serde_json::from_str(data.as_str()).expect("Couldnt parse json");
    let iterations = cmp::max(cmp::max(data.todo.len(), data.doing.len()), data.done.len());
    for i in 0..iterations { 
        if i < data.todo.len() {
            draw_text_at_column(&mut buffer, 0, &data.todo[i]);
        }
        if i < data.doing.len() {
            draw_text_at_column(&mut buffer, 1, &data.doing[i]);
        }
        if i < data.done.len() {
            draw_text_at_column(&mut buffer, 2, &data.done[i]);
        }
    }
    clear_screen();
    draw_buffer(buffer);
    
}
