use re_types::components::{Color, Material};
use re_viewer_context::{UiContext, ViewerContext};

use crate::DataUi;

impl DataUi for Material {
    fn data_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        ui_context: UiContext,
        query: &re_data_store::LatestAtQuery,
        db: &re_entity_db::EntityDb,
    ) {
        let show_optional_albedo_factor = |ui: &mut egui::Ui| {
            if let Some(albedo_factor) = self.albedo_factor {
                Color(albedo_factor).data_ui(ctx, ui, ui_context, query, db);
            } else {
                ui.weak("(empty)");
            }
        };

        match ui_context {
            UiContext::List | UiContext::Tooltip => {
                show_optional_albedo_factor(ui);
            }
            UiContext::SelectionPanelFull | UiContext::SelectionPanelLimitHeight => {
                egui::Grid::new("material").num_columns(2).show(ui, |ui| {
                    ui.label("albedo_factor");
                    show_optional_albedo_factor(ui);
                    ui.end_row();
                });
            }
        }
    }
}
