mod ui;
mod db;

use termion::input::TermRead;
use::termion::event::Key;
use ui::draw_double_box;
use ui::draw_row;
use std::fs;
use std::env;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use crate::ui::{WIDTH, HEIGHT, Buffer, Position, TextObject, clear_screen, draw_buffer, draw_options, draw_text_object};
use crate::db::{Column, create_db, JsonData, load_data};

const COLUMNS_AMOUNT: usize = 3;
const COLUMN_WIDTH: usize = WIDTH/COLUMNS_AMOUNT;

enum Mode {
    Normal,
    Add
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

    let data = load_data();

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
        draw_row(&mut buffer, COLUMNS_AMOUNT);
        if selected_column == 2 {
            draw_double_box(&mut buffer, COLUMN_WIDTH + 1, HEIGHT-1, Position { x: selected_column*COLUMN_WIDTH, y: 0 });
        } else {
            draw_double_box(&mut buffer, COLUMN_WIDTH, HEIGHT-1, Position { x: selected_column*COLUMN_WIDTH, y: 0 });
        }
        draw_buffer(buffer, &mut stdout);
        draw_options(&mut stdout);
        write!(stdout, "\r\n    Selected column: {}\r\n", match selected_column {
            0 => "TODO",
            1 => "DOING",
            2 => "DONE",
            _ => "Unknown column"
        }).unwrap();
        stdout.flush().unwrap();
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
