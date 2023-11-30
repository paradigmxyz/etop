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
            .constraints(vec![Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)])
            .split(rect);
        let rect = rects[0];
        let inner_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        let inner_rects2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rects[1]);

        // let color = Color::Rgb(0, 255, 0);
        let color = Color::Rgb(255, 255, 255);

        let s = data.dataset.clone();
        let style = Style::default().fg(color).bold();
        let title = block::Title::from(s.dim()).alignment(Alignment::Left);
        let block = Block::default().title(title).style(style);
        f.render_widget(block, inner_rects[0]);

        let s = get_block_window_string(&data);
        let style = Style::default().fg(color).bold();
        let title = block::Title::from(s.dim()).alignment(Alignment::Right);
        let block = Block::default().title(title).style(style);
        f.render_widget(block, inner_rects[1]);

        let s = get_current_block_string(&data);
        let style = Style::default().fg(color).bold();
        let title = block::Title::from(s.dim()).alignment(Alignment::Right);
        let block = Block::default().title(title).style(style);
        f.render_widget(block, inner_rects2[1]);

        Ok(())
    }
}

fn get_block_window_string(data: &EtopState) -> String {
    match (data.window.start_block, data.window.end_block) {
        (Some(start_block), Some(end_block)) => {
            if start_block == end_block {
                format!("showing block {}", end_block)
            } else {
                format!("showing block {} to {}", start_block, end_block)
            }
        },
        (_, Some(end_block)) => format!("block {}", end_block),
        (_, None) => "-".to_string(),
    }
}

fn get_current_block_string(data: &EtopState) -> String {
    match data.latest_block {
        Some(block) => format!("latest block: {}", block),
        None => "latest block: -".to_string(),
    }
}
