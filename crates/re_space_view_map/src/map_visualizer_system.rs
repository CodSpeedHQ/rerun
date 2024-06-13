use re_data_store::LatestAtQuery;
use re_types::archetypes::GpsCoordinates;
use re_types::components;
use re_viewer_context::{
    IdentifiedViewSystem, SpaceViewSystemExecutionError, ViewContext, ViewContextCollection,
    ViewQuery, VisualizerQueryInfo, VisualizerSystem,
};
use walkers::Position;

// ---

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub position: Position,
    pub radii: Option<components::Radius>,
    pub color: Option<components::Color>,
}

impl Default for MapEntry {
    fn default() -> Self {
        MapEntry {
            position: Position::from_lat_lon(51.4934, 0.),
            radii: None,
            color: None,
        }
    }
}

/// A text scene, with everything needed to render it.
#[derive(Default)]
pub struct MapVisualizerSystem {
    pub map_entries: Vec<MapEntry>,
}

impl IdentifiedViewSystem for MapVisualizerSystem {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "Map".into()
    }
}

impl VisualizerSystem for MapVisualizerSystem {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<GpsCoordinates>()
    }

    fn execute(
        &mut self,
        ctx: &ViewContext<'_>,
        view_query: &ViewQuery<'_>,
        _context_systems: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError> {
        let timeline_query = LatestAtQuery::new(view_query.timeline, view_query.latest_at);

        for data_result in view_query.iter_visible_data_results(ctx, Self::identifier()) {
            let Some(position) = ctx
                .recording()
                .latest_at_component::<components::Position3D>(
                    &data_result.entity_path,
                    &timeline_query,
                )
                .map(|res| res.value)
            else {
                // Text component is required.
                continue;
            };

            let radii = ctx
                .recording()
                .latest_at_component::<components::Radius>(
                    &data_result.entity_path,
                    &timeline_query,
                )
                .map(|res| res.value);

            let color = ctx
                .recording()
                .latest_at_component::<components::Color>(&data_result.entity_path, &timeline_query)
                .map(|res| res.value);

            self.map_entries.push(MapEntry {
                position: Position::from_lat_lon(position.x() as f64, position.y() as f64),
                radii,
                color,
            });
        }

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_fallback_provider(&self) -> &dyn re_viewer_context::ComponentFallbackProvider {
        self
    }
}

re_viewer_context::impl_component_fallback_provider!(MapVisualizerSystem => []);
