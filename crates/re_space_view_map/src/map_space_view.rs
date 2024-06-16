use egui::Color32;
use egui::{Ui, Window};

use re_types::components::{Color, Radius};

use walkers::{sources::Attribution, Map, MapMemory, Plugin, Tiles, TilesManager};
use {
    egui::{self, Context},
    re_entity_db::EntityProperties,
    re_log_types::EntityPath,
    re_space_view::suggest_space_view_for_each_entity,
    re_types::{SpaceViewClassIdentifier, View},
    re_ui,
    re_viewer_context::{
        SpaceViewClass, SpaceViewClassLayoutPriority, SpaceViewClassRegistryError, SpaceViewId,
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
    MapboxStreets,
    MapboxDark,
    MapboxSatellite,
}

/// Sample map plugin which draws custom stuff on the map.
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

/// Space view state for the custom space view.
///
/// This state is preserved between frames, but not across Viewer sessions.
#[derive(Default)]
pub struct MapSpaceViewState {
    tiles: Option<Tiles>,
    map_memory: MapMemory,
    selected_provider: Provider,
    mapbox_access_token: String,
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

type ViewType = re_types::blueprint::views::MapView;

impl SpaceViewClass for MapSpaceView {
    // State type as described above.

    fn identifier() -> SpaceViewClassIdentifier {
        ViewType::identifier()
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
        println!("Creating new state");
        Box::<MapSpaceViewState>::new(MapSpaceViewState {
            tiles: None,
            map_memory: MapMemory::default(),
            selected_provider: Provider::OpenStreetMap,
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

    /// Additional UI displayed when the space view is selected.
    ///
    /// In this sample we show a combo box to select the color coordinates mode.
    fn selection_ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        _space_view_id: SpaceViewId,
        _root_entity_properties: &mut EntityProperties,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let map_state = state.downcast_mut::<MapSpaceViewState>()?;
        let mut selected = map_state.selected_provider;

        // TODO(tfoldi): UI looks horrible, needs to be improved
        ui.horizontal(|ui| {
            ui.label("Map provider");
            egui::ComboBox::from_id_source("map_provider")
                .selected_text(format!("{selected:?}"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, Provider::OpenStreetMap, "OpenStreetMap");
                    ui.selectable_value(
                        &mut selected,
                        Provider::MapboxStreets,
                        "Mapbox Streets (Light)",
                    );
                    ui.selectable_value(
                        &mut selected,
                        Provider::MapboxDark,
                        "Mapbox Streets (Dark)",
                    );
                    ui.selectable_value(
                        &mut selected,
                        Provider::MapboxSatellite,
                        "Mapbox Satellite",
                    );
                });
        });

        ui.horizontal(|ui| {
            ui.label("Mapbox Access Token").on_hover_text("Access token for Mapbox API. Please refer to the Mapbox documentation for more information.");
            ui.text_edit_singleline(&mut map_state.mapbox_access_token);
        });

        ui.horizontal(|ui| {
            if ui
                .button("Follow position")
                .on_hover_text("Follow the position of the entity on the map.")
                .clicked()
            {
                map_state.map_memory.follow_my_position();
            }
        });

        // If the selected provider has changed, reset the tiles.
        if selected != map_state.selected_provider {
            map_state.tiles = None;
            map_state.selected_provider = selected;
        }

        Ok(())
    }

    /// The contents of the Map Space View window and all interaction within it.
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
        let (tiles, map_memory) = if let Some(ref mut _tiles) = state.tiles {
            state.get_mut_refs()
        } else {
            state.set_tiles(get_tile_manager(
                state.selected_provider,
                &state.mapbox_access_token,
                ui.ctx(),
            ));
            state.get_mut_refs()
        };

        egui::Frame::default().show(ui, |ui| {
            // TODO(tfoldi): tiles must be always Some at this point but still would be nice to have a check here
            let tiles = &mut (*tiles.unwrap());

            let some_tiles_manager: Option<&mut dyn TilesManager> = Some(tiles);
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

            // TODO(tfoldi): move to function along with acknowledge in a separate file
            let map_pos = map_widget.rect;
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
            acknowledge(ui, &map_pos, tiles.attribution());
        });
        Ok(())
    }
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

fn get_tile_manager(provider: Provider, mapbox_access_token: &str, egui_ctx: &Context) -> Tiles {
    match provider {
        Provider::OpenStreetMap => Tiles::new(walkers::sources::OpenStreetMap, egui_ctx.clone()),
        Provider::MapboxStreets => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Streets,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        Provider::MapboxDark => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Dark,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        Provider::MapboxSatellite => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Satellite,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: true,
            },
            egui_ctx.clone(),
        ),

        #[allow(unreachable_patterns)]
        _ => unreachable!("Provider not implemented"),
    }
}
