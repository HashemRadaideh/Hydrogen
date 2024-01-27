use std::io::{stdout, Result, Stdout, Write};

use crossterm::cursor::{position, MoveToColumn};
use crossterm::event::KeyModifiers;
use crossterm::terminal;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand, QueueableCommand,
};

use crate::hash::validate;
use crate::repl::cell::Cell;
use crate::repl::linebuffer::LineBuffer;
use crate::repl::mode::CursorMode;

mod cell;
mod linebuffer;
mod mode;

fn print_message(stdout: &mut Stdout, msg: &str) -> Result<()> {
    stdout
        .queue(Print("\n"))?
        .queue(MoveToColumn(0))?
        .queue(Print(msg))?
        .queue(Print("\n"))?
        .queue(MoveToColumn(0))?;
    stdout.flush()?;
    Ok(())
}

pub fn repl(mode: String) -> Result<()> {
    let edit_mode = CursorMode::new(mode);
    let mut line = LineBuffer::new();
    let mut stdout: Stdout = stdout();

    'repl: loop {
        terminal::enable_raw_mode()?;
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print("> "))?
            .execute(ResetColor)?;
        stdout.flush()?;

        let mut start: Cell = position()
            .map(|(col, row)| Cell::new(col, row))
            .unwrap_or_else(|_| Cell::new(1, 1));
        start.col += 1;
        start.row += 1;
        line.caret.col = start.col;
        line.caret.row = start.row;

        stdout.flush()?;
        'input: loop {
            match edit_mode {
                CursorMode::Normal => match read()? {
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => match code {
                        KeyCode::Char(c) => {
                            if modifiers == KeyModifiers::CONTROL && c == 'd' {
                                break 'repl;
                            }

                            line.buffer.push(c);
                            stdout.queue(Print(c))?;
                            stdout.flush()?;
                        }
                        KeyCode::Enter => {
                            break 'input;
                        }
                        KeyCode::Backspace => {}
                        KeyCode::Delete => {}
                        KeyCode::Up => {}
                        KeyCode::Left => {}
                        KeyCode::Down => {}
                        KeyCode::Right => {}
                        _ => {}
                    },
                    Event::Mouse(_) => {}
                    Event::Resize(width, height) => {
                        print_message(&mut stdout, &format!("width: {width}, height: {height}"))?;
                    }
                    _ => {}
                },
                CursorMode::Vi => match read()? {
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => match code {
                        KeyCode::Char(c) => {
                            if modifiers == KeyModifiers::CONTROL && c == 'd' {
                                break 'repl;
                            }

                            line.buffer.push(c);
                            stdout.queue(Print(c))?;
                            stdout.flush()?;
                        }
                        KeyCode::Enter => {}
                        KeyCode::Backspace => {}
                        KeyCode::Delete => {}
                        KeyCode::Up => {}
                        KeyCode::Left => {}
                        KeyCode::Down => {}
                        KeyCode::Right => {}
                        _ => {}
                    },
                    Event::Mouse(_) => {}
                    Event::Resize(width, height) => {
                        print_message(&mut stdout, &format!("width: {width}, height: {height}"))?;
                    }
                    _ => {}
                },
                CursorMode::Emacs => match read()? {
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => match code {
                        KeyCode::Char(c) => {
                            if modifiers == KeyModifiers::CONTROL && c == 'd' {
                                break 'repl;
                            }

                            line.buffer.push(c);
                            stdout.queue(Print(c))?;
                            stdout.flush()?;
                        }
                        KeyCode::Enter => {}
                        KeyCode::Backspace => {}
                        KeyCode::Delete => {}
                        KeyCode::Up => {}
                        KeyCode::Left => {}
                        KeyCode::Down => {}
                        KeyCode::Right => {}
                        _ => {}
                    },
                    Event::Mouse(_) => {}
                    Event::Resize(width, height) => {
                        print_message(&mut stdout, &format!("width: {width}, height: {height}"))?;
                    }
                    _ => {}
                },
            }
        }

        terminal::disable_raw_mode()?;
        println!();
        validate(&line.buffer);
        line.buffer.clear();
    }

        terminal::disable_raw_mode()?;
        println!();
    Ok(())
}
