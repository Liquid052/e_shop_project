use crate::plugins::window::states::helpers::get_or_insert_mut;
use crate::plugins::window::states::{MainState, WindowState};
use crate::prelude::App;
use anymap::AnyMap;
use egui::{Align, Layout, Ui, Widget};
use log::error;
use std::env;
use std::ptr::eq;

#[derive(Debug, Clone, Default)]
pub struct Login {
    pub username: String,
    pub password: String,
}

const LOGIN: &str = "admin";
const PASSWORD: &str = "123456";

pub fn login(state: &mut WindowState, cache: &mut AnyMap, ui: &mut Ui) {
    if !cache.contains::<Login>() {
        cache.insert(Login {
            username: env::var("LOGIN").unwrap_or_default(),
            password: env::var("PASSWORD").unwrap_or_default(),
        });
    }

    let login = get_or_insert_mut::<Login>(cache);

    egui::CentralPanel::default().show_inside(ui, |ui| {
        // Add vertical space at the top to push the form down
        ui.add_space(130.0);

        // Create a centered column with maximum width
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            // Set maximum width for the form
            let max_width = 200.0;
            ui.set_max_width(max_width);

            // Style the title
            ui.heading("LOGIN");
            ui.add_space(20.0);

            // Username field with label
            ui.label("Username");
            ui.add(egui::TextEdit::singleline(&mut login.username).desired_width(max_width));
            ui.add_space(10.0);

            // Password field with label
            ui.label("Password");

            ui.add(
                egui::TextEdit::singleline(&mut login.password)
                    .password(true)
                    .desired_width(max_width),
            );

            ui.add_space(20.0);

            // Center the login button
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.button("Login").clicked().then(|| {
                    let predicate = login.username.eq(LOGIN) && login.password.eq(PASSWORD);
                    if !predicate {
                        error!("Login denied");
                        return;
                    }

                    *state = WindowState::Main(MainState::Products);
                });
            });
        });
    });
}
