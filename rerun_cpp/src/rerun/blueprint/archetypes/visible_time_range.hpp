// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/visible_time_range.fbs".

#pragma once

#include "../../blueprint/components/visible_time_range_sequence.hpp"
#include "../../blueprint/components/visible_time_range_time.hpp"
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
    /// **Archetype**: Configures what range of the timeline is shown on a view.
    ///
    /// Whenever no visual time range applies, queries are done with "latest at" semantics.
    /// This means that the view will, starting from the time cursor position,
    /// query the latest data available for each component type.
    ///
    /// The default visual time range depends on the type of view this property applies to:
    /// - For time series views, the default is to show the entire timeline.
    /// - For any other view, the default is to apply latest-at semantics.
    ///
    /// The visual time range can be overridden also individually per entity.
    struct VisibleTimeRange {
        /// The range of time to show for timelines based on sequence numbers.
        std::optional<rerun::blueprint::components::VisibleTimeRangeSequence> sequence;

        /// The range of time to show for timelines based on time.
        std::optional<rerun::blueprint::components::VisibleTimeRangeTime> time;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.VisibleTimeRangeIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        VisibleTimeRange() = default;
        VisibleTimeRange(VisibleTimeRange&& other) = default;

        /// The range of time to show for timelines based on sequence numbers.
        VisibleTimeRange with_sequence(
            rerun::blueprint::components::VisibleTimeRangeSequence _sequence
        ) && {
            sequence = std::move(_sequence);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// The range of time to show for timelines based on time.
        VisibleTimeRange with_time(rerun::blueprint::components::VisibleTimeRangeTime _time) && {
            time = std::move(_time);
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
    struct AsComponents<blueprint::archetypes::VisibleTimeRange> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(
            const blueprint::archetypes::VisibleTimeRange& archetype
        );
    };
} // namespace rerun