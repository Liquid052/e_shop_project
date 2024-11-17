use crate::models::{Product, ProductComment};
use crate::plugins::window::states::helpers::get_or_insert_mut;
use crate::plugins::BusinessPlugin;
use anymap::AnyMap;
use eframe::emath::Align;
use egui::{CentralPanel, Key, Layout, ScrollArea, SidePanel, Ui};

#[derive(Debug, Default, Clone)]
struct ProductSearchCache {
    search: String,
    vec: Vec<Product>,

    selected: Option<Product>,
    comments: Vec<ProductComment>,
    comment_cache: String,
    username: String,
}

pub fn product_view(business: &mut BusinessPlugin, cache: &mut AnyMap, ui: &mut Ui) {
    if !cache.contains::<ProductSearchCache>() {
        cache.insert(ProductSearchCache {
            vec: business.search_products("".into()),
            ..Default::default()
        });
    }
    let cache = get_or_insert_mut::<ProductSearchCache>(cache);

    SidePanel::left("Product list")
        .min_width(300.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.label("Products");

            ui.horizontal(|ui| {
                let re = ui.text_edit_singleline(&mut cache.search);
                let mut enter = false;
                re.ctx
                    .input(|i| i.key_pressed(Key::Enter))
                    .then(|| enter = true);
                if re.lost_focus() && enter {
                    cache.vec = business.search_products(cache.search.clone());
                }

                ui.button("search").clicked().then(|| {
                    cache.vec = business.search_products(cache.search.clone());
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for product in &cache.vec {
                        ui.group(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::RIGHT), |ui| {
                                // Use a horizontal layout for the product details.
                                ui.horizontal(|ui| {
                                    ui.button("view").clicked().then(|| {
                                        cache.selected = product.clone().into();
                                        cache.comments = business.get_comments(product);
                                        cache.comment_cache.clear();
                                    });

                                    ui.label(format!("{}", &product.name));
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
        });

    // product description
    if cache.selected.is_none() {
        cache.comment_cache.clear();
        return;
    }

    let product = cache.selected.as_ref().unwrap();

    CentralPanel::default().show_inside(ui, |ui| {
        // product info
        ui.heading(&product.name);
        ui.separator();
        ui.add_space(4.0);
        ui.label(format!("description: {}", product.description));
        ui.add_space(4.0);
        ui.label(format!("price: {:.2} kč", product.price));

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
                    for comment in &cache.comments {
                        ui.group(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                ui.label(&comment.content);
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
        });

        ui.add_space(16.0);

        ui.label("new comment");
        ui.add_space(2.0);

        ui.text_edit_multiline(&mut cache.comment_cache);
        let mut enter = false;

        ui.add_space(4.0);
        ui.label("username:");
        ui.text_edit_singleline(&mut cache.username);
        ui.add_space(4.0);
        enter |= ui.button("add new").clicked();

        if enter {
            business.add_comment(
                cache.username.clone(),
                cache.selected.as_ref().unwrap(),
                cache.comment_cache.clone(),
            );
            cache.comment_cache.clear();
            cache.comments = business.get_comments(product);
        }
    });
}
