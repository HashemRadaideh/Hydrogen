use std::io::{stdout, Result, Stdout, Write};

use crossterm::cursor::{position, MoveToColumn};
use crossterm::event::KeyModifiers;
use crossterm::terminal;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand, QueueableCommand,
};

use crate::hash::evaluator::Evaluator;
use crate::repl::cell::Cell;
use crate::repl::linebuffer::LineBuffer;
use crate::repl::mode::CursorMode;

/// Module containing REPL-related functionality.
mod cell;
/// Module containing line buffer implementation.
mod linebuffer;
/// Module containing cursor modes for the REPL.
mod mode;

/// Prints a message to the standard output with proper formatting.
///
/// # Arguments
///
/// * `stdout` - The standard output.
/// * `msg` - The message to be printed.
///
/// # Returns
///
/// * `Result<()>` - Ok(()) if printing is successful, Err(io::Error) otherwise.
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

/// Displays the REPL prompt with the provided message.
///
/// # Arguments
///
/// * `stdout` - The standard output.
/// * `prompt` - The prompt message to be displayed.
///
/// # Returns
///
/// * `Result<()>` - Ok(()) if displaying the prompt is successful, Err(io::Error) otherwise.
fn prompt(stdout: &mut Stdout, prompt: &str) -> Result<()> {
    stdout
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print(prompt))?
        .execute(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

/// Runs the Read-Eval-Print Loop (REPL) for interactive input.
///
/// # Arguments
///
/// * `mode` - The initial cursor mode for the REPL ("normal", "vi", or "emacs").
///
/// # Returns
///
/// * `Result<()>` - Ok(()) if the REPL runs successfully, Err(io::Error) otherwise.
pub fn repl(mode: String) -> Result<()> {
    let edit_mode = CursorMode::new(mode);
    let mut line = LineBuffer::new();
    let mut stdout: Stdout = stdout();

    terminal::enable_raw_mode()?;
    'repl: loop {
        prompt(&mut stdout, "> ")?;

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
                            // let (_, ok) = validate(&line.buffer);
                            // if ok {
                            break 'input;
                            // } else {
                            //     prompt(&mut stdout, "... ")?;
                            // }
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
        let mut evaluator = Evaluator::new(&line.buffer);
        evaluator.eval();
        line.buffer.clear();
    }

    terminal::disable_raw_mode()?;
    println!();
    Ok(())
}
