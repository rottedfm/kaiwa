use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::{CursorMove, Input, TextArea};

use super::Component;
use crate::{action::Action, config::Config};

// TODO: Vim Modes
// TODO: Vim Mode Configuration/Keybinds
// TODO: Prompt actions
// TODO: Rendering
// TODO: Move Mode back to app for better keybind support

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

pub struct Prompt {
    command_tx: Option<UnboundedSender<Action>>,
    textarea: TextArea<'static>,
    mode: Mode,
    config: Config,
}

impl Prompt {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::default().borders(Borders::ALL).title("NORMAL MODE"));
        Self {
            command_tx: None,
            config: Config::default(),
            textarea,
            mode: Mode::Normal,
        }
    }

    // Change and update the 'Prompt' title accordingly
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        let title = match mode {
            Mode::Normal => "NORMAL MODE",
            Mode::Insert => "INSERT MODE",
            Mode::Visual => "VISUAL MODE",
        };
        self.textarea
            .set_block(Block::default().borders(Borders::ALL).title(title));
    }
}

impl Component for Prompt {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {}

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        // Check mode-specific bindings in the config
        if let Some(mode_bindings) = self.config.keybindings.get(&self.mode) {
            if let Some(action) = mode_bindings.get(&vec![key]) {
                return self.update(action.clone());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Paragraph::new("hello world"), area);
        Ok(())
    }
}
