use crate::plugins::window::states::main_display::accounts::accounts_view;
use crate::plugins::window::states::MainState;
use crate::plugins::BusinessPlugin;
use crate::prelude::*;
use anymap::AnyMap;
use egui::Ui;
use orders::orders_view;
use products::product_view;

mod accounts;
mod orders;
mod products;

pub fn display_main(
    state: &mut MainState,
    business: &mut BusinessPlugin,
    cache: &mut AnyMap,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        ui.radio_value(state, MainState::Products, "Products");
        ui.radio_value(state, MainState::Accounts, "Accounts");
        ui.radio_value(state, MainState::Orders, "Orders");
    });

    ui.separator();

    match state {
        MainState::Products => product_view(business, cache, ui),
        MainState::Accounts => accounts_view(business, cache, ui),
        MainState::Orders => orders_view(business, cache, ui),
    }
}
