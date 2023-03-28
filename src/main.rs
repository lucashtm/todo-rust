mod ui;
mod db;

use termion::input::TermRead;
use::termion::event::Key;
use ui::draw_double_box;
use ui::draw_row;
use std::env;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use crate::ui::{WIDTH, HEIGHT, Buffer, Position, TextObject, clear_screen, draw_buffer, draw_options, draw_text_object};
use crate::db::{Column, create_db, load_data};

const COLUMNS_AMOUNT: usize = 3;
const COLUMN_WIDTH: usize = WIDTH/COLUMNS_AMOUNT;

enum Mode {
    Normal,
    Add
}

fn draw_column_title(buffer: &mut Buffer, column_index: usize, text: &String) {
    let column_width = WIDTH / 3;
    let mut column_title_text_object = TextObject {
        text: text.to_string(),
        position: Position { x: column_index*column_width + column_width/2 - text.len()/2, y: 1 },
        width: column_width,
        height: None
    };
    draw_text_object(buffer, &mut column_title_text_object);
}

fn draw_text_at_column(buffer: &mut Buffer, text: &String, position: Position) -> TextObject {
    let mut text_object = TextObject {
        text: text.to_string(),
        position,
        width: WIDTH/3 - 3,
        height: None
    };
    draw_text_object(buffer, &mut text_object);
    return text_object
}

fn draw_column(mut buffer: &mut Buffer, column_index: usize, column: &Column) {
    draw_column_title(buffer, column_index, &column.title);
    let mut last_height: usize = 2;
    for (_i, task) in column.tasks.iter().enumerate() {
        let text_object = draw_text_at_column(buffer, &task.text, Position { x: column_index*WIDTH/3+2, y: last_height+1 });
        draw_double_box(&mut buffer, COLUMN_WIDTH, text_object.height.unwrap(), Position { x: column_index*COLUMN_WIDTH, y: last_height });
        last_height = text_object.height.unwrap();
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

    let data = load_data();

    let columns: [&mut Column; 3] = [
        &mut Column {
            title: String::from("TODO"),
            tasks: data.todo
        },
        &mut Column {
            title: String::from("DOING"),
            tasks: data.doing
        },
        &mut Column {
            title: String::from("DONE"),
            tasks: data.done
        }
    ];



    let mut selected_column: usize = 0;
    // let mut focused_task: Option<&mut db::Task> = None;
    let typed_text: &mut String = &mut String::from("");
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
        for i in 0..columns.len() {
            draw_column(&mut buffer, i, columns[i]);
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
                        write!(stdout, "{}", termion::cursor::Show).unwrap();
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
                        write!(stdout, "{}{}{}> {}", termion::cursor::Show, termion::cursor::Goto(0, 40), termion::clear::CurrentLine, typed_text).unwrap();
                        break;
                    }
                    _ => {}
                },
                Mode::Add => {
                    match character.unwrap() {
                        Key::Esc => {
                            mode = Mode::Normal;
                            break;
                        }
                        Key::Backspace => {
                            typed_text.pop();
                        }
                        Key::Char(char_value) => {
                            if char_value == '\n' {
                                columns[selected_column].add_task(typed_text.to_string());
                                typed_text.clear();
                                mode = Mode::Normal;
                                break;
                            } else {
                                typed_text.push(char_value);
                            }
                        }
                        _ => {}
                    }
                    write!(stdout, "{}{}{}> {}", termion::cursor::Show, termion::cursor::Goto(0, 40), termion::clear::CurrentLine, typed_text).unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
    }
}
