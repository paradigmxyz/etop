use etop_core::EtopState;
use super::Component;
use crate::{action::Action, tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Header {}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Header {
    fn update(&mut self, _action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect, data: EtopState) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
            .split(rect);
        let rect = rects[0];
        let inner_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        let color = Color::Rgb(0, 255, 0);

        let s = data.dataset.clone();
        let style = Style::default().fg(color).bold();
        let title = block::Title::from(s.dim()).alignment(Alignment::Left);
        let block = Block::default().title(title).style(style);
        f.render_widget(block, inner_rects[0]);

        let s = get_block_number_string(data);
        let style = Style::default().fg(color).bold();
        let title = block::Title::from(s.dim()).alignment(Alignment::Right);
        let block = Block::default().title(title).style(style);
        f.render_widget(block, inner_rects[1]);

        Ok(())
    }
}

fn get_block_number_string(data: EtopState) -> String {
    match data.window.end_block {
        Some(block_number) => format!("block {}", block_number),
        None => "-".to_string(),
    }
}
