use crate::models::*;
use crate::plugins::window::states::helpers::get_or_insert_mut;
use crate::plugins::BusinessPlugin;
use anymap::AnyMap;
use eframe::emath::Align;
use egui::{CentralPanel, Layout, ScrollArea, SidePanel, Ui};

#[derive(Debug, Default, Clone)]
struct ProductSearchCache {
    search: String,
    vec: Vec<Order>,

    selected: Option<(Order, Account, Vec<(usize, Product)>)>,

    delete_acc: bool,
}

pub fn orders_view(business: &mut BusinessPlugin, cache: &mut AnyMap, ui: &mut Ui) {
    if !cache.contains::<ProductSearchCache>() {
        cache.insert(ProductSearchCache {
            vec: business.get_all_orders(),
            ..Default::default()
        });
    }
    let cache = get_or_insert_mut::<ProductSearchCache>(cache);

    SidePanel::left("Product list")
        .min_width(300.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.add_space(12.0);
            ui.label("Orders");
            ui.add_space(19.0);
            ui.separator();
            ui.add_space(4.0);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for order in &cache.vec {
                        ui.group(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::RIGHT), |ui| {
                                // Use a horizontal layout for the product details.
                                ui.horizontal(|ui| {
                                    ui.button("view").clicked().then(|| {
                                        let (account, products) =
                                            business.get_order_details(order).unwrap();
                                        cache.selected = Some((order.clone(), account, products));
                                        cache.delete_acc = false;
                                    });

                                    ui.label(format!("Order {}", &order.order_id));
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
        });

    // product description
    let Some((order, account, products)) = &cache.selected else {
        return;
    };

    // let product = cache.selected.as_ref().unwrap();
    let mut clear_flag = false;
    CentralPanel::default().show_inside(ui, |ui| {
        // product info
        ui.heading(&format!("Order {}", order.order_id));
        ui.separator();
        ui.add_space(4.0);
        ui.label(format!("username: {}", account.username));
        ui.add_space(4.0);
        ui.label(format!("total price: {:.2} kč", order.total_price));

        // comments of the product
        ui.add_space(8.0);
        ui.label("comments");
        ui.add_space(2.0);

        ui.group(|ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .min_scrolled_height(150.0)
                .min_scrolled_width(ui.available_width())
                .max_height(150.0)
                .show(ui, |ui| {
                    for (count, product) in products {
                        ui.group(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                ui.label(&format!("{}, amount: {}", product.name, count));
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
        });

        ui.add_space(16.0);

        ui.label("options");
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            ui.checkbox(&mut cache.delete_acc, "Delete account with order");
        });

        ui.button("Cancel order").clicked().then(|| {
            business.cancel_order(order);

            if cache.delete_acc {
                business.delete_account(account);
            }

            clear_flag = true;
        });
        ui.add_space(2.0);
    });
    if clear_flag {
        cache.delete_acc = false;
        cache.selected = None;
        cache.vec = business.get_all_orders();
    }
}
