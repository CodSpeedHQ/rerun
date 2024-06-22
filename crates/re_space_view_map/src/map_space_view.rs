use re_types::blueprint::archetypes::MapOptions;
use re_viewport_blueprint::ViewProperty;

use {
    egui::{self, Color32, Context},
    re_log_types::EntityPath,
    re_space_view::suggest_space_view_for_each_entity,
    re_types::blueprint::components::MapProvider,
    re_types::{
        components::{Color, Radius},
        SpaceViewClassIdentifier, View,
    },
    re_viewer_context::{
        SpaceViewClass, SpaceViewClassLayoutPriority, SpaceViewClassRegistryError, SpaceViewId,
        SpaceViewSpawnHeuristics, SpaceViewState, SpaceViewStateExt as _,
        SpaceViewSystemExecutionError, SpaceViewSystemRegistrator, SystemExecutionOutput,
        ViewQuery, ViewerContext,
    },
    walkers::{HttpTiles, Map, MapMemory, Plugin, Tiles},
};

use crate::map_visualizer_system::{MapEntry, MapVisualizerSystem};
use crate::map_windows;

// walkers plugin to visualize points on a Map
pub struct PositionsOnMap {
    positions: Vec<MapEntry>,
}

impl Plugin for PositionsOnMap {
    fn run(
        &mut self,
        _response: &egui::Response,
        painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        for entry in &self.positions {
            // Position of the point we want to put our shapes.
            let position = entry.position;

            // Project it into the position on the screen.
            let position = projector.project(position).to_pos2();

            // Radius of the circle
            let radius = f32::from(entry.radii.unwrap_or(Radius(10.)));

            // Color of the circle
            let color = entry.color.unwrap_or(Color::new(Color32::RED));

            painter.circle_filled(position, radius, color);
        }
    }
}

#[derive(Default)]
pub struct MapSpaceViewState {
    tiles: Option<HttpTiles>,
    map_memory: MapMemory,
    selected_provider: MapProvider,
    mapbox_access_token: String,
}

impl MapSpaceViewState {
    // This method ensures that tiles is initialized and returns mutable references to tiles and map_memory.
    pub fn ensure_and_get_mut_refs(
        &mut self,
        ctx: &egui::Context,
    ) -> Result<(&mut HttpTiles, &mut MapMemory), SpaceViewSystemExecutionError> {
        if self.tiles.is_none() {
            let tiles = get_tile_manager(self.selected_provider, &self.mapbox_access_token, ctx);
            self.tiles = Some(tiles);
        }

        // Now that tiles is guaranteed to be Some, unwrap is safe here.
        let tiles_ref = self
            .tiles
            .as_mut()
            .ok_or(SpaceViewSystemExecutionError::MapTilesError)?;
        Ok((tiles_ref, &mut self.map_memory))
    }
}

impl SpaceViewState for MapSpaceViewState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct MapSpaceView;

type ViewType = re_types::blueprint::views::MapView;

impl SpaceViewClass for MapSpaceView {
    fn identifier() -> SpaceViewClassIdentifier {
        ViewType::identifier()
    }

    fn display_name(&self) -> &'static str {
        "Map"
    }

    fn icon(&self) -> &'static re_ui::Icon {
        &re_ui::icons::SPACE_VIEW_MAP
    }

    fn help_text(&self, _egui_ctx: &egui::Context) -> egui::WidgetText {
        "Map view".into()
    }

    fn on_register(
        &self,
        system_registry: &mut SpaceViewSystemRegistrator<'_>,
    ) -> Result<(), SpaceViewClassRegistryError> {
        system_registry.register_visualizer::<MapVisualizerSystem>()
    }

    fn new_state(&self) -> Box<dyn SpaceViewState> {
        Box::<MapSpaceViewState>::new(MapSpaceViewState {
            tiles: None,
            map_memory: MapMemory::default(),
            selected_provider: MapProvider::default(),
            // TODO(tfoldi): this should come from the app configuration or blueprint
            mapbox_access_token: std::env::var("MAPBOX_ACCESS_TOKEN").unwrap_or_default(),
        })
    }

    fn preferred_tile_aspect_ratio(&self, _state: &dyn SpaceViewState) -> Option<f32> {
        // Prefer a square tile if possible.
        Some(1.0)
    }

    fn layout_priority(&self) -> SpaceViewClassLayoutPriority {
        Default::default()
    }

    fn spawn_heuristics(&self, ctx: &ViewerContext<'_>) -> SpaceViewSpawnHeuristics {
        suggest_space_view_for_each_entity::<MapVisualizerSystem>(ctx, self)
    }

    fn selection_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        space_view_id: SpaceViewId,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        re_ui::list_item::list_item_scope(ui, "map_selection_ui", |ui| {
            re_space_view::view_property_ui::<re_types::blueprint::archetypes::MapOptions>(
                ctx,
                ui,
                space_view_id,
                self,
                state,
            );
        });

        // TODO(tfoldi): this should be moved to the view_property_ui / blueprint
        let map_state = state.downcast_mut::<MapSpaceViewState>()?;

        // ui.horizontal(|ui| {
        //     ui.label("Mapbox Access Token").on_hover_text("Access token for Mapbox API. Please refer to the Mapbox documentation for more information.");
        //     ui.add( TextEdit::singleline(&mut map_state.mapbox_access_token)
        //         .hint_text("Mapbox Access Token")
        //         .password(true));
        // });

        let mut zoom_level = map_state.map_memory.zoom();
        ui.horizontal(|ui| {
            ui.label("Zoom level");
            ui.add(egui::Slider::new(&mut zoom_level, 0.0..=19.0));
            if zoom_level != map_state.map_memory.zoom() {
                let _ = map_state.map_memory.set_zoom(zoom_level);
            }
        });

        // ui.horizontal(|ui| {
        //     let mut is_following = map_state.map_memory.detached().is_none();

        //     if ui
        //         .re_checkbox(&mut is_following, "Follow positions on map")
        //         .changed()
        //     {
        //         if is_following {
        //             map_state.map_memory.follow_my_position();
        //         } else {
        //             // Detach the map from the current position
        //             // TODO(tfoldi): should be added to the map_memory API
        //         }
        //     }
        // });

        Ok(())
    }

    fn ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,

        query: &ViewQuery<'_>,
        system_output: SystemExecutionOutput,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let state = state.downcast_mut::<MapSpaceViewState>()?;
        let map_viz_system = system_output.view_systems.get::<MapVisualizerSystem>()?;

        let blueprint_db = ctx.blueprint_db();
        let view_id = query.space_view_id;
        let map_options =
            ViewProperty::from_archetype::<MapOptions>(blueprint_db, ctx.blueprint_query, view_id);
        let map_provider = map_options.component_or_fallback::<MapProvider>(ctx, self, state)?;

        // if state changed let's update it from the blueprint
        if state.selected_provider != map_provider {
            state.tiles = None;
            state.selected_provider = map_provider;
        }

        let (tiles, map_memory) = match state.ensure_and_get_mut_refs(ui.ctx()) {
            Ok(refs) => refs,
            Err(err) => return Err(err),
        };

        egui::Frame::default().show(ui, |ui| {
            let some_tiles_manager: Option<&mut dyn Tiles> = Some(tiles);
            let map_widget = ui.add(
                Map::new(
                    some_tiles_manager,
                    map_memory,
                    map_viz_system
                        .map_entries
                        .first()
                        .unwrap_or(&MapEntry::default())
                        .position,
                )
                .with_plugin(PositionsOnMap {
                    positions: map_viz_system.map_entries.clone(),
                }),
            );

            map_widget.double_clicked().then(|| {
                map_memory.follow_my_position();
            });

            let map_pos = map_widget.rect;
            let window_id = query.space_view_id.uuid().to_string();
            map_windows::zoom(ui, &window_id, &map_pos, map_memory);
            map_windows::acknowledge(ui, &window_id, &map_pos, tiles.attribution());
        });
        Ok(())
    }
}

fn get_tile_manager(
    provider: MapProvider,
    mapbox_access_token: &str,
    egui_ctx: &Context,
) -> HttpTiles {
    match provider {
        MapProvider::OpenStreetMap => {
            HttpTiles::new(walkers::sources::OpenStreetMap, egui_ctx.clone())
        }
        MapProvider::MapboxStreets => HttpTiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Streets,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        MapProvider::MapboxDark => HttpTiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Dark,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        MapProvider::MapboxSatellite => HttpTiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Satellite,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: true,
            },
            egui_ctx.clone(),
        ),
    }
}

re_viewer_context::impl_component_fallback_provider!(MapSpaceView => []);
