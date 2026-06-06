/*
 * Copyright 2026 Jhe-An Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use ratatui::backend::CrosstermBackend;
use ratatui::text::Text;
use ratatui::widgets::{Widget, Wrap};
use ratatui::{
    Terminal,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Paragraph},
};
use std::io::Stdout;
use tui_input::Input;

#[derive(Debug)]
pub struct TUI {
    pub input: Input,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TUI {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            input: Default::default(),
            terminal,
        }
    }

    pub fn push_message(&mut self, message: String, style: Style) -> Result<(), std::io::Error> {
        let width = self.terminal.size()?.width;

        let text = Text::from(message.as_str());

        let mut total_height = 0;
        for line in text.lines.iter() {
            let line_width = line.width() as u16;
            if line_width == 0 {
                total_height += 1;
            } else {
                total_height += (line_width + width - 1) / width;
            }
        }

        self.terminal.insert_before(total_height, |frame| {
            let area = frame.area();
            let paragraph = Paragraph::new(text).style(style).wrap(Wrap { trim: false });
            paragraph.render(*area, frame);
        })?;

        self.input.reset();
        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), std::io::Error> {
        let TUI {
            input: _input,
            terminal,
        } = self;
        terminal.draw(|frame| {
            let [area] = Layout::vertical([Constraint::Length(3)]).areas(frame.area());

            let width = area.width.max(3) - 3;
            let scroll = self.input.visual_scroll(width as usize);
            let input = Paragraph::new(self.input.value())
                .style(Style::default())
                .scroll((0, scroll as u16))
                .block(Block::bordered().title("Input"));
            frame.render_widget(input, area);

            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        })?;
        Ok(())
    }
}
