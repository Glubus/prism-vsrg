//! Page module - full-screen page layouts.
//!
//! Each page represents a complete screen in the application:
//! - `main_menu`: Title screen with navigation

pub mod main_menu;
pub mod song_select;

pub use main_menu::MainMenuPage;
