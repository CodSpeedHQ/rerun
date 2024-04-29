use re_entity_db::{external::re_data_store::LatestAtQuery, EntityDb, EntityProperties};
use re_log_types::Timeline;
use re_viewer_context::{
    DataQueryResult, PerVisualizer, SpaceViewClassRegistry, StoreContext, VisualizableEntities,
};

pub struct EntityOverrideContext {
    pub legacy_space_view_properties: EntityProperties,

    pub default_visible_time_range: re_types::blueprint::datatypes::VisibleTimeRange,
}

/// Trait for resolving properties needed by most implementations of [`DataQuery`]
///
/// The `SpaceViewBlueprint` is the only thing that likely implements this today
/// but we use a trait here so we don't have to pick up a full dependency on `re_viewport`.
pub trait PropertyResolver {
    fn update_overrides(
        &self,
        blueprint: &EntityDb,
        blueprint_query: &LatestAtQuery,
        active_timeline: &Timeline,
        space_view_class_registry: &SpaceViewClassRegistry,
        query_result: &mut DataQueryResult,
    );
}

/// The common trait implemented for data queries
///
/// Both interfaces return [`re_viewer_context::DataResult`]s, which are self-contained description of the data
/// to be added to a `SpaceView` including both the [`re_log_types::EntityPath`] and context for any overrides.
pub trait DataQuery {
    /// Execute a full query, returning a `DataResultTree` containing all results.
    ///
    /// `auto_properties` is a map containing any heuristic-derived auto properties for the given `SpaceView`.
    ///
    /// This is used when building up the contents for a `SpaceView`.
    fn execute_query(
        &self,
        ctx: &StoreContext<'_>,
        visualizable_entities_for_visualizer_systems: &PerVisualizer<VisualizableEntities>,
    ) -> DataQueryResult;
}
