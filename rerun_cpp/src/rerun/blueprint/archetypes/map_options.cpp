// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/map_options.fbs".

#include "map_options.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<blueprint::archetypes::MapOptions>::serialize(
        const blueprint::archetypes::MapOptions& archetype
    ) {
        using namespace blueprint::archetypes;
        std::vector<DataCell> cells;
        cells.reserve(4);

        {
            auto result = DataCell::from_loggable(archetype.provider);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto result = DataCell::from_loggable(archetype.zoom);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto result = DataCell::from_loggable(archetype.access_token);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = MapOptions::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
