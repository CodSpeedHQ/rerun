

use egui::Color32;
use egui::{Align2, Ui, Window};

use re_types::components::Radius;
use walkers::{sources::Attribution, Map, MapMemory, Plugin, Tiles, TilesManager};
use {
    egui::{self, ahash::HashMap, Context},
    re_entity_db::{EntityProperties},
    re_log_types::EntityPath,
    re_space_view::suggest_space_view_for_each_entity,
    re_types::SpaceViewClassIdentifier,
    re_ui,
    re_viewer_context::{
        SpaceViewClass,
        SpaceViewClassLayoutPriority, SpaceViewClassRegistryError, SpaceViewId,
        SpaceViewSpawnHeuristics, SpaceViewState, SpaceViewStateExt as _,
        SpaceViewSystemExecutionError, SpaceViewSystemRegistrator, SystemExecutionOutput,
        ViewQuery, ViewerContext,
    },
};

use crate::map_visualizer_system::{MapEntry, MapVisualizerSystem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Provider {
    #[default]
    OpenStreetMap,
    Geoportal,
    MapboxStreets,
    MapboxDark,
    MapboxSatellite,
    LocalTiles,
}

/// Sample map plugin which draws custom stuff on the map.
pub struct CustomShapes {
    positions: Vec<MapEntry>,
}

impl Plugin for CustomShapes {
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

            // Draw a circle at the position.
            let radius = f32::from(entry.radii.unwrap_or(Radius(10.)));

            painter.circle_filled(position, radius, Color32::RED);
        }
    }
}

/// Space view state for the custom space view.
///
/// This state is preserved between frames, but not across Viewer sessions.
#[derive(Default)]
pub struct MapSpaceViewState {
    // TODO(wumpf, jleibs): This should be part of the Blueprint so that it is serialized out.
    //                      but right now there is no way of doing that.
    tiles: Option<Tiles>,
    map_memory: MapMemory,
    selected_provider: Provider,
    token: String,
}

impl MapSpaceViewState {
    pub fn get_mut_refs(&mut self) -> (Option<&mut Tiles>, &mut MapMemory) {
        (self.tiles.as_mut(), &mut self.map_memory)
    }
    pub fn set_tiles(&mut self, tiles: Tiles) {
        self.tiles = Some(tiles);
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

impl SpaceViewClass for MapSpaceView {
    // State type as described above.

    fn identifier() -> SpaceViewClassIdentifier {
        "Map".into()
    }

    fn display_name(&self) -> &'static str {
        "Map"
    }

    fn icon(&self) -> &'static re_ui::Icon {
        &re_ui::icons::SPACE_VIEW_GENERIC
    }

    fn help_text(&self, _egui_ctx: &egui::Context) -> egui::WidgetText {
        "Map view".into()
    }

    /// Register all systems (contexts & parts) that the space view needs.
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
            selected_provider: Provider::OpenStreetMap,
            token: String::new(),
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

    /// Additional UI displayed when the space view is selected.
    ///
    /// In this sample we show a combo box to select the color coordinates mode.
    fn selection_ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        _state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        _space_view_id: SpaceViewId,
        _root_entity_properties: &mut EntityProperties,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let mut selected = "osm";

        ui.horizontal(|ui| {
            ui.label("Map provider");
            egui::ComboBox::from_id_source("color_coordinates_mode")
                .selected_text("OpenStreetMap")
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, "osm", "OpenStreetMap");
                    ui.selectable_value(&mut selected, "geoportal", "Geoportal");
                    ui.selectable_value(&mut selected, "mapbox", "Mapbox");
                });
        });

        Ok(())
    }

    /// The contents of the Space View window and all interaction within it.
    ///
    /// This is called with freshly created & executed context & part systems.
    fn ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _root_entity_properties: &EntityProperties,
        _query: &ViewQuery<'_>,
        system_output: SystemExecutionOutput,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let state = state.downcast_mut::<MapSpaceViewState>()?;
        let map_viz_system = system_output.view_systems.get::<MapVisualizerSystem>()?;

        // set tiles in case it is not already
        let (tiles, memory) = match state.tiles {
            Some(ref mut _tiles) => state.get_mut_refs(),
            None => {
                state.set_tiles(Tiles::new(
                    walkers::sources::Mapbox {
                        style: walkers::sources::MapboxStyle::Dark,
                        access_token: std::option_env!("MAPBOX_ACCESS_TOKEN").unwrap().to_string(),
                        high_resolution: false,
                    },
                    ui.ctx().clone(),
                ));
                state.get_mut_refs()
            }
        };

        egui::Frame::default().show(ui, |ui| {
            let tiles = &mut (*tiles.unwrap());

            let _attribution = tiles.attribution();
            let some_tiles_manager: Option<&mut dyn TilesManager> = Some(tiles);
            let _map = ui.add(
                Map::new(
                    some_tiles_manager,
                    // tiles,
                    memory,
                    // &mut MapMemory::default(),
                    map_viz_system.map_entries[0].position,
                )
                .with_plugin(CustomShapes {
                    positions: map_viz_system.map_entries.clone(),
                }),
            );
            // acknowledge(ui, attribution);
        });
        Ok(())
    }
}

pub fn acknowledge(ui: &Ui, attribution: Attribution) {
    Window::new("Acknowledge")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(Align2::LEFT_TOP, [10., 10.])
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if let Some(logo) = attribution.logo_light {
                    ui.add(egui::Image::new(logo).max_height(30.0).max_width(80.0));
                }
                ui.hyperlink_to(attribution.text, attribution.url);
            });
        });
}

fn providers(egui_ctx: Context) -> HashMap<Provider, Box<dyn TilesManager + Send>> {
    let mut providers: HashMap<Provider, Box<dyn TilesManager + Send>> = HashMap::default();

    providers.insert(
        Provider::OpenStreetMap,
        Box::new(Tiles::new(
            walkers::sources::OpenStreetMap,
            egui_ctx.to_owned(),
        )),
    );

    providers.insert(
        Provider::Geoportal,
        Box::new(Tiles::new(walkers::sources::Geoportal, egui_ctx.to_owned())),
    );

    // Pass in a mapbox access token at compile time. May or may not be what you want to do,
    // potentially loading it from application settings instead.
    let mapbox_access_token = std::option_env!("MAPBOX_ACCESS_TOKEN");

    // We only show the mapbox map if we have an access token
    if let Some(token) = mapbox_access_token {
        providers.insert(
            Provider::MapboxStreets,
            Box::new(Tiles::new(
                walkers::sources::Mapbox {
                    style: walkers::sources::MapboxStyle::Streets,
                    access_token: token.to_string(),
                    high_resolution: false,
                },
                egui_ctx.to_owned(),
            )),
        );
        providers.insert(
            Provider::MapboxSatellite,
            Box::new(Tiles::new(
                walkers::sources::Mapbox {
                    style: walkers::sources::MapboxStyle::Satellite,
                    access_token: token.to_string(),
                    high_resolution: true,
                },
                egui_ctx.to_owned(),
            )),
        );
    }

    providers
}
