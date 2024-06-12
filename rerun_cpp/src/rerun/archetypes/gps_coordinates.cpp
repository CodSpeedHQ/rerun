// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/gps_coordinates.fbs".

#include "gps_coordinates.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::GpsCoordinates>::serialize(
        const archetypes::GpsCoordinates& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(4);

        {
            auto result = DataCell::from_loggable(archetype.coordinates);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.radii.has_value()) {
            auto result = DataCell::from_loggable(archetype.radii.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.colors.has_value()) {
            auto result = DataCell::from_loggable(archetype.colors.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = GpsCoordinates::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
