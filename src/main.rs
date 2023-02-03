use serde::{Deserialize, Serialize};
use termion::input::TermRead;
use termion::raw::RawTerminal;
use::termion::event::Key;
use std::fs;
use std::env;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, Stdout, stdin};

const WIDTH: usize = 80;
const HEIGHT: usize = 24;

const COLUMNS_AMOUNT: usize = 3;
const COLUMN_WIDTH: usize = WIDTH/COLUMNS_AMOUNT;

enum Mode {
    Normal,
    Add
}

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
struct Task {
    text: String,
    position: usize
}

struct Column<'a> {
    title: String,
    tasks: &'a Vec<Task>
}

#[derive(Serialize, Deserialize)]
struct JsonData {
    todo: Vec<Task>,
    doing: Vec<Task>,
    done: Vec<Task>,
}

fn draw_buffer(buffer: Buffer, stdout: &mut RawTerminal<Stdout>) {
    for line in buffer.iter() {
        let mut output = String::from_iter(line);
        output.push_str("\r\n");
        write!(stdout, "{output}").unwrap();
    }
}

fn draw_options(stdout: &mut RawTerminal<Stdout>) {
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

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
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

fn draw_column(buffer: &mut Buffer, column_index: usize, column: &Column) {
    draw_column_title(buffer, column_index, &column.title);
    for task in column.tasks {
        draw_text_at_column(buffer, column_index, &task.text);
    }
}

fn draw_frame(buffer: &mut Buffer, selected_column: usize) {
    // Corners
    buffer[0][0] = match selected_column { 0 => '\u{250F}', _ => '\u{250C}'};
    buffer[0][WIDTH-1] = match selected_column { 2 => '\u{2513}', _ => '\u{2510}'};
    buffer[HEIGHT-1][0] = match selected_column { 0 => '\u{2517}', _ =>'\u{2514}'};
    buffer[HEIGHT-1][WIDTH-1] = match selected_column { 2 => '\u{251B}', _ =>'\u{2518}'};

    // Vertical
    for i in 1..HEIGHT-1 {
        buffer[i][0] = match selected_column { 0 => '\u{2503}', _ => '\u{2502}'};
        buffer[i][WIDTH-1] = match selected_column { 2 => '\u{2503}', _ => '\u{2502}' };
        buffer[i][WIDTH/3] = match selected_column { 2 => '\u{2502}', _ => '\u{2503}' };
        buffer[i][2*WIDTH/3] = match selected_column { 0 => '\u{2502}', _ => '\u{2503}' };
    }

    // Horizontal
    for i in 1..WIDTH-1 {
        let in_selected = i > selected_column*COLUMN_WIDTH && i < selected_column*COLUMN_WIDTH + COLUMN_WIDTH;
        let normal_char: char = match in_selected { false => '\u{2500}', true => '\u{2501}' };
        let bottom_split: char = '\u{2534}';
        let top_split: char = '\u{252C}';

        if i == WIDTH/3 || i == 2*WIDTH/3 {
            buffer[0][i] = top_split;
            buffer[HEIGHT-1][i] = bottom_split;
            continue;
        }
        buffer[0][i] = normal_char;
        buffer[HEIGHT-1][i] = normal_char;
    }
}

fn create_db() {
    fs::create_dir_all("data").expect("Cannot create data dir");
    let empty_data: JsonData = JsonData { todo: vec![], doing: vec![], done: vec![] };
    let empty_data = serde_json::to_string(&empty_data).expect("Invalid json");
    fs::write("data/data.json", empty_data).expect("Cannot write to data/data.json");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Migrate database
    if args.len() == 2 && args[1] == "migrate" {
        create_db();
        return
    }

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut buffer: Buffer = [[' '; WIDTH]; HEIGHT];

    let mut columns: [Column; 3] = [
        Column {
            title: String::from("TODO"),
            tasks: &vec![]
        },
        Column {
            title: String::from("DOING"),
            tasks: &vec![]
        },
        Column {
            title: String::from("DONE"),
            tasks: &vec![]
        }
    ];


    for i in 0..columns.len() {
        draw_column(&mut buffer, i, &columns[i]);
    }

    let data = fs::read_to_string("data/data.json").expect("Couldnt open file");
    let data: JsonData = serde_json::from_str(data.as_str()).expect("Couldnt parse json");

    for i in 0..columns.len() {
        columns[i].tasks = match i {
            0 => &data.todo,
            1 => &data.doing,
            2 => &data.done,
            _ => panic!("Too many columns")
        };

        draw_column(&mut buffer, i, &columns[i]);
    }

    let mut selected_column: usize = 0;
    let mut mode: Mode = Mode::Normal;
    loop {
        let stdin = stdin();
        clear_screen();
        draw_frame(&mut buffer, selected_column);
        draw_buffer(buffer, &mut stdout);
        draw_options(&mut stdout);
        write!(stdout, "\r\n    Selected column: {}\r\n", match selected_column {
            0 => "TODO",
            1 => "DOING",
            2 => "DONE",
            _ => "Unknown column"
        }).unwrap();
        for character in stdin.keys() {
            match mode {
                Mode::Normal => match character.unwrap() {
                    Key::Char('Q') => {
                        // Save
                        return
                    }
                    Key::Char('h') => {
                        if selected_column == 0 {
                            selected_column = 2;
                            break;
                        }
                        selected_column -= 1;
                        break;
                    }
                    Key::Char('l') => {
                        selected_column = (selected_column + 1) % 3;
                        break;
                    }
                    Key::Char('a') => {
                        mode = Mode::Add;
                        break;
                    }
                    _ => {}
                },
                Mode::Add => {}
            }
        }
    }
}
