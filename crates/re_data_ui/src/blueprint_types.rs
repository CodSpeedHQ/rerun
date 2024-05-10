use re_types_blueprint::blueprint::components::{IncludedSpaceView, SpaceViewMaximized};
use re_viewer_context::{SpaceViewId, UiContext, ViewerContext};

use crate::DataUi;

// ---

impl DataUi for IncludedSpaceView {
    #[allow(clippy::only_used_in_recursion)]
    fn data_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        ui_context: UiContext,
        query: &re_data_store::LatestAtQuery,
        db: &re_entity_db::EntityDb,
    ) {
        let space_view: SpaceViewId = self.0.into();
        space_view.data_ui(ctx, ui, ui_context, query, db);
    }
}

impl DataUi for SpaceViewMaximized {
    #[allow(clippy::only_used_in_recursion)]
    fn data_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        ui_context: UiContext,
        query: &re_data_store::LatestAtQuery,
        db: &re_entity_db::EntityDb,
    ) {
        let space_view: SpaceViewId = self.0.into();
        space_view.data_ui(ctx, ui, ui_context, query, db);
    }
}
