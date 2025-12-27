//! Main menu rendering.

use crate::input::events::GameAction;
use crate::views::components::menu::main_menu::{MainMenuAction, MainMenuScreen};

pub fn render(ctx: &egui::Context, actions: &mut Vec<GameAction>) {
    let action = MainMenuScreen::render(ctx);
    match action {
        MainMenuAction::Play => {
            actions.push(GameAction::Confirm);
        }
        MainMenuAction::Quit => {
            actions.push(GameAction::Back);
        }
        MainMenuAction::None => {}
    }
}
