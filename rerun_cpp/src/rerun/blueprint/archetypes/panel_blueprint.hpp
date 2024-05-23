// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/panel_blueprint.fbs".

#pragma once

#include "../../blueprint/components/panel_state.hpp"
#include "../../collection.hpp"
#include "../../compiler_utils.hpp"
#include "../../data_cell.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: Shared state for the 3 collapsible panels.
    struct PanelBlueprint {
        std::optional<rerun::blueprint::components::PanelState> state;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.PanelBlueprintIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        PanelBlueprint() = default;
        PanelBlueprint(PanelBlueprint&& other) = default;

        PanelBlueprint with_state(rerun::blueprint::components::PanelState _state) && {
            state = std::move(_state);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::PanelBlueprint> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(
            const blueprint::archetypes::PanelBlueprint& archetype
        );
    };
} // namespace rerun
