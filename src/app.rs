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
    // Current value of the input box
    raw_input: Vec<Vec<char>>,
    encrypted_input: Vec<Vec<char>>,
    // Position of cursor in input
    // Position of cursor in the editor area.
    cursor_x_index: usize,
    cursor_y_index: usize,
    // Current input mode
    input_mode: InputMode,
    enigma: EnigmaMachine,
}

enum InputMode {
    Normal,
}

impl App {
    pub const fn new(enigma: EnigmaMachine) -> Self {
        Self {
            raw_input: Vec::new(),
            encrypted_input: Vec::new(),
            input_mode: InputMode::Normal,
            cursor_x_index: 0,
            cursor_y_index: 0,
            enigma,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_x_index.saturating_sub(1);
        self.cursor_x_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_x_index.saturating_add(1);
        self.cursor_x_index = self.clamp_cursor(cursor_moved_right);
    }

    fn move_cursor_up(&mut self) {
        let cursor_moved_up = self.cursor_y_index.saturating_sub(1);
        self.cursor_y_index = self.clamp_cursor(cursor_moved_up);
    }

    fn move_cursor_down(&mut self) {
        let cursor_moved_down = self.cursor_y_index.saturating_add(1);
        self.cursor_y_index = self.clamp_cursor(cursor_moved_down);
    }

    fn enter_char(&mut self, new_char: char) {
        let line = self.raw_input.get_mut(self.cursor_y_index);

        if let Some(line) = line {
            line.insert(self.cursor_x_index, new_char);
        } else {
            self.raw_input.push(vec![new_char]);
        }

        let encrypted_line = self.encrypted_input.get_mut(self.cursor_y_index);

        if let Some(line) = encrypted_line {
            line.insert(self.cursor_x_index, self.enigma.encrypt_char(new_char));
        } else {
            self.encrypted_input.push(vec![new_char]);
        }

        self.move_cursor_right();
    }

    fn new_line(&mut self) {
        self.raw_input.push(Vec::new());
        self.encrypted_input.push(Vec::new());
        self.move_cursor_down();
        self.cursor_x_index = 0;
    }

    fn delete_char(&mut self) {
        let is_not_cursor_top_left = self.cursor_y_index != 0 || self.cursor_x_index != 0;
        if is_not_cursor_top_left {
            if self.raw_input[self.cursor_y_index].is_empty() {
                self.raw_input.remove(self.cursor_y_index);
                self.encrypted_input.remove(self.cursor_y_index);
                self.move_cursor_up();
                self.cursor_x_index = self.raw_input[self.cursor_y_index].len();
            } else {
                let char_before_cursor = self.cursor_x_index - 1;
                self.raw_input[self.cursor_y_index].remove(char_before_cursor);
                self.encrypted_input[self.cursor_y_index].remove(char_before_cursor);
                self.move_cursor_left();
            }
        }
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.raw_input[self.cursor_y_index].len())
    }

    // Change to using a gap buffer and calculates the cursor pos based on gap buffer cursor and
    // max x of text area

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
        let [input_area, encrypted_area] = vertical.areas(frame.area());

        let input = Paragraph::new(
            self.raw_input
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .style(match self.input_mode {
            InputMode::Normal => Style::default(),
        })
        .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let output = Paragraph::new(
            self.encrypted_input
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .style(match self.input_mode {
            InputMode::Normal => Style::default(),
        })
        .block(Block::bordered().title("Output"));

        frame.render_widget(output, encrypted_area);

        match self.input_mode {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Normal => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.cursor_x_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + self.cursor_y_index as u16 + 1,
            )),
        }
    }
}
