use crate::plugins::window::states::{display_main, login, WindowState};
use crate::prelude::*;
use anymap::AnyMap;
use egui::Ui;

pub(super) fn get_or_insert_mut<T: Default + 'static>(cache: &mut AnyMap) -> &mut T {
    if cache.contains::<T>() {
        return cache.get_mut::<T>().unwrap();
    }

    cache.insert(T::default());
    cache.get_mut().unwrap()
}

pub fn show_state(
    state: &mut WindowState,
    domain: &mut BusinessPlugin,
    cache: &mut AnyMap,
    ui: &mut Ui,
) {
    let current_state = *state;

    match current_state {
        WindowState::Login => login(state, cache, ui),
        WindowState::Main(_main) => {
            if let WindowState::Main(main) = state {
                display_main(main, domain, cache, ui)
            }
        }
    }
}
