use egui::{self};
use egui::{Ui, Window};
use walkers::{sources::Attribution, MapMemory};

pub fn zoom(ui: &Ui, map_pos: &egui::Rect, map_memory: &mut MapMemory) {
    Window::new("Zoom")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .current_pos(egui::Pos2::new(map_pos.max.x - 80., map_pos.min.y + 10.))
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("➕").heading()).clicked() {
                    let _ = map_memory.zoom_in();
                }

                if ui.button(egui::RichText::new("➖").heading()).clicked() {
                    let _ = map_memory.zoom_out();
                }
            });
        });
}

pub fn acknowledge(ui: &Ui, map_pos: &egui::Rect, attribution: Attribution) {
    Window::new("Acknowledge")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .current_pos(egui::Pos2::new(map_pos.min.x + 10., map_pos.max.y - 40.))
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if let Some(logo) = attribution.logo_light {
                    ui.add(egui::Image::new(logo).max_height(30.0).max_width(80.0));
                }
                ui.hyperlink_to(attribution.text, attribution.url);
            });
        });
}
