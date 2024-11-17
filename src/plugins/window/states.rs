#![allow(dead_code)]
mod helpers;
mod login_display;
mod main_display;

pub(super) use helpers::*;
pub(super) use login_display::*;
pub(super) use main_display::*;

#[derive(Debug, Default, Copy, Clone)]
pub enum WindowState {
    #[default]
    Login,
    Main(MainState),
}

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub enum MainState {
    #[default]
    Products,
    Orders,
    Accounts,
}
