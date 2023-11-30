use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use etop_core::{EtopState, Window, WindowSize};
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{body::Body, header::Header, Component},
    tui,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

pub struct App {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub data: EtopState,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64, data: Option<EtopState>) -> Result<Self> {
        let header = Header::new();
        let body = Body::new();

        let window = Window {
            start_block: Some(17_000_001),
            end_block: Some(18_000_000),
            live: false,
            size: WindowSize::Block(100),
        };
        let data = match data {
            Some(data) => data,
            None => EtopState {
                dataset: "transactions by to_address".to_string(),
                window,
                ..Default::default()
            },
        };

        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![Box::new(header), Box::new(body)],
            should_quit: false,
            should_suspend: false,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            data,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        tui.tick_rate(self.tick_rate);
        tui.frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init()?;
        }

        //  initialize
        action_tx.clone().send(Action::LoadDataset(self.data.dataset.clone()))?;
        if self.data.rpc_source.is_some() {
            action_tx.clone().send(Action::BeginBlockSubscription)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => match key.code {
                        KeyCode::Backspace => action_tx.send(Action::PreviousWindow)?,
                        KeyCode::Char('l') => action_tx.send(Action::LiveWindow)?,
                        KeyCode::Char('q') => action_tx.send(Action::Quit)?,
                        KeyCode::Char('[') => action_tx.send(Action::DecrementWindow)?,
                        KeyCode::Char(']') => action_tx.send(Action::IncrementWindow)?,
                        _ => {}
                    },
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                match action {
                    Action::Tick => {}
                    Action::Render => {}
                    _ => log::debug!("{action:?}"),
                }
                match action.clone() {
                    Action::BeginBlockSubscription => {
                        let action_tx = action_tx.clone();
                        let data = self.data.clone();
                        tokio::spawn(async move {
                            let rpc_source = match data.rpc_source {
                                Some(rpc_source) => rpc_source,
                                None => return,
                            };
                            loop {
                                if let Ok(latest_block) = rpc_source.fetcher.get_block_number().await {
                                    let _result = action_tx
                                        .send(Action::BlockSeen(latest_block.as_u32()));
                                };
                                tokio::time::sleep(Duration::from_secs(1)).await;
                            }
                        });
                    }
                    Action::LoadDataset(_dataset) => {
                        let action_tx = action_tx.clone();
                        let data = self.data.clone();
                        tokio::spawn(async move {
                            let data_dir = data.file_source.data_dir;
                            let dataspec = etop_core::load_dataspec(data.dataset);
                            if let (Some(data_dir), Ok(dataspec)) = (data_dir, dataspec) {
                                if let Ok(warehouse) =
                                    etop_core::load_warehouse_from_filesystem(&*dataspec, data_dir)
                                {
                                    let _result = action_tx.send(Action::NewWarehouse(warehouse));
                                }
                            };
                        });
                    }
                    Action::NewWarehouse(warehouse) => self.data.warehouse = warehouse,
                    Action::BlockSeen(seen_block) => {
                        self.data.see_block(seen_block)
                    },
                    Action::IncrementWindow => {
                        self.data.increment_window(1);
                    }
                    Action::DecrementWindow => {
                        self.data.decrement_window(1);
                    }
                    Action::LiveWindow => {
                        self.data.enable_live_mode();
                    }
                    Action::RequestQuery(query, block_range) => {
                        let action_tx = action_tx.clone();
                        let data = self.data.clone();
                        tokio::spawn(async move {
                            if let Ok(df) = data.query(query.clone(), block_range).await {
                                let _result = action_tx.send(Action::ReceiveQuery(query, block_range, df));
                            };
                        });
                    }
                    Action::ReceiveQuery(query, _block_range, df) => {
                        let result = self.data.warehouse.add_dataset(query, df);
                    }
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.draw(f, f.size(), self.data.clone());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.draw(f, f.size(), self.data.clone());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?;
                tui.tick_rate(self.tick_rate);
                tui.frame_rate(self.frame_rate);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
