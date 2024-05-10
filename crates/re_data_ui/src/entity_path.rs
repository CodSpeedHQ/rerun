use re_entity_db::InstancePath;
use re_viewer_context::{UiContext, ViewerContext};

use super::DataUi;

impl DataUi for re_entity_db::EntityPath {
    fn data_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        ui_context: UiContext,
        query: &re_data_store::LatestAtQuery,
        db: &re_entity_db::EntityDb,
    ) {
        InstancePath::entity_all(self.clone()).data_ui(ctx, ui, ui_context, query, db);
    }
}
