use super::Component;
use crate::{action::Action, tui::Frame};
use color_eyre::eyre::{eyre, Result};
use etop_core::EtopState;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Body {}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

impl Body {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Body {
    fn update(&mut self, _action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect, data: EtopState) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Min(0)])
            .split(rect);
        let rect = rects[1];

        // let color = Color::Rgb(0, 255, 0);
        let color = Color::Gray;

        let s = if data.can_render() {
            match data.format_window((rect.height + 1).into(), rect.width.into()) {
                Ok(s) => s,
                Err(_) => return Err(eyre!("could not format window")),
            }
        } else {
            format!("warehouse has {} entries", data.warehouse.data.len())
        };

        let s = format!("{}\n\n\n{}", s, data.messages.join("\n"));

        let style = Style::default().fg(color);
        let content = Paragraph::new(s).style(style);
        f.render_widget(content, rect);

        Ok(())
    }
}
