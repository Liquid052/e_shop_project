use crate::models::Account;
use crate::plugins::window::states::helpers::get_or_insert_mut;
use crate::plugins::BusinessPlugin;
use anymap::AnyMap;
use eframe::emath::Align;
use egui::{Key, Layout, ScrollArea, SidePanel, Ui};

#[derive(Debug, Default, Clone)]
struct ProductSearchCache {
    search: String,
    vec: Vec<Account>,
}

pub fn accounts_view(business: &mut BusinessPlugin, cache: &mut AnyMap, ui: &mut Ui) {
    if !cache.contains::<ProductSearchCache>() {
        cache.insert(ProductSearchCache {
            vec: business.search_accounts("".into()),
            ..Default::default()
        });
    }
    let cache = get_or_insert_mut::<ProductSearchCache>(cache);

    SidePanel::left("Accounts list")
        .min_width(300.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.label("Accounts");

            ui.horizontal(|ui| {
                let re = ui.text_edit_singleline(&mut cache.search);
                let mut enter = false;
                re.ctx
                    .input(|i| i.key_pressed(Key::Enter))
                    .then(|| enter = true);
                if re.lost_focus() && enter {
                    cache.vec = business.search_accounts(cache.search.clone());
                }

                ui.button("search").clicked().then(|| {
                    cache.vec = business.search_accounts(cache.search.clone());
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for account in &cache.vec {
                        ui.group(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::RIGHT), |ui| {
                                // Use a horizontal layout for the product details.
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "{}, {} {}",
                                        account.username, account.first_name, account.last_name
                                    ));
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
        });
}
