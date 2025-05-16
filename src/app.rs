use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::Style,
    widgets::{Block, Paragraph},
};

use std::io::Error;

use crate::enigma::machine::EnigmaMachine;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    encrypted_input: String,
    /// Position of cursor in the editor area.
    character_x_index: usize,
    character_y_index: usize,
    /// Current input mode
    input_mode: InputMode,
    enigma: EnigmaMachine,
}

enum InputMode {
    Normal,
}

impl App {
    pub const fn new(enigma: EnigmaMachine) -> Self {
        Self {
            input: String::new(),
            encrypted_input: String::new(),
            input_mode: InputMode::Normal,
            character_x_index: 0,
            character_y_index: 0,
            enigma,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_x_index.saturating_sub(1);
        self.character_x_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_x_index.saturating_add(1);
        self.character_x_index = self.clamp_cursor(cursor_moved_right);
    }

    fn move_cursor_up(&mut self) {
        let cursor_moved_up = self.character_y_index.saturating_sub(1);
        self.character_y_index = self.clamp_cursor(cursor_moved_up);
    }

    fn move_cursor_down(&mut self) {
        let cursor_moved_down = self.character_y_index.saturating_add(1);
        self.character_y_index = self.clamp_cursor(cursor_moved_down);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);

        let encrypted_char = if !new_char.is_alphabetic() {
            new_char
        } else {
            self.enigma.encrypt_char(new_char)
        };

        self.encrypted_input.insert(index, encrypted_char);
        self.move_cursor_right();
    }

    fn new_line(&mut self) {
        let index = self.byte_index();
        self.input.insert(index, '\u{000a}');
        self.encrypted_input.insert(index, '\u{000a}');
        self.move_cursor_down();
        self.character_x_index = 0;
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_x_index + self.character_y_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_x_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_x_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => self.new_line(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [input_area, output_area] = vertical.areas(frame.area());

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let output = Paragraph::new(self.encrypted_input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
            })
            .block(Block::bordered().title("Output"));
        frame.render_widget(output, output_area);

        match self.input_mode {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Normal => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.character_x_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + self.character_y_index as u16 + 1,
            )),
        }
    }
}
