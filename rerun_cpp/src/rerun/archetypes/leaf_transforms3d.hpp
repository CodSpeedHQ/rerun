// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/leaf_transforms3d.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/leaf_rotation_axis_angle.hpp"
#include "../components/leaf_rotation_quat.hpp"
#include "../components/leaf_scale3d.hpp"
#include "../components/leaf_transform_mat3x3.hpp"
#include "../components/leaf_translation3d.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: One or more transforms between the parent and the current entity which are *not* propagated in the transform hierarchy.
    ///
    /// For transforms that are propagated in the transform hierarchy, see `archetypes::Transform3D`.
    ///
    /// If both `archetypes::LeafTransforms3D` and `archetypes::Transform3D` are present,
    /// first the tree propagating `archetypes::Transform3D` is applied, then `archetypes::LeafTransforms3D`.
    ///
    /// Currently, most visualizers support only a single leaf transform per entity.
    /// Check archetype documentations for details - if not otherwise specified, onlyt the first leaf transform is applied.
    ///
    /// From the point of view of the entity's coordinate system,
    /// all components are applied in the inverse order they are listed here.
    /// E.g. if both a translation and a max3x3 transform are present,
    /// the 3x3 matrix is applied first, followed by the translation.
    ///
    /// ## Example
    ///
    /// ### Regular & leaf transform in tandom
    /// ![image](https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    /// #include <rerun/demo_utils.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_asset3d_out_of_tree");
    ///     rec.set_time_sequence("frame", 0);
    ///
    ///     // Log a box and points further down in the hierarchy.
    ///     rec.log("world/box", rerun::Boxes3D::from_half_sizes({{1.0, 1.0, 1.0}}));
    ///     rec.log(
    ///         "world/box/points",
    ///         rerun::Points3D(rerun::demo::grid3d<rerun::Position3D, float>(-10.0f, 10.0f, 10))
    ///     );
    ///
    ///     for (int i = 1; i <20; ++i) {
    ///         rec.set_time_sequence("frame", i);
    ///
    ///         // Log a regular transform which affects both the box and the points.
    ///         rec.log(
    ///             "world/box",
    ///             rerun::Transform3D::from_rotation(rerun::RotationAxisAngle{
    ///                 {0.0f, 0.0f, 1.0f},
    ///                 {rerun::Angle::degrees(static_cast<float>(i) * 2.0f)}
    ///             })
    ///         );
    ///
    ///         // Log an leaf transform which affects only the box.
    ///         float translation[] = {0, 0, fabs(static_cast<float>(i) * 0.1f - 5.0f) - 5.0f};
    ///         rec.log("world/box", rerun::LeafTransforms3D().with_translations({translation}));
    ///     }
    /// }
    /// ```
    struct LeafTransforms3D {
        /// Translation vectors.
        std::optional<Collection<rerun::components::LeafTranslation3D>> translations;

        /// Rotations via axis + angle.
        std::optional<Collection<rerun::components::LeafRotationAxisAngle>> rotation_axis_angles;

        /// Rotations via quaternion.
        std::optional<Collection<rerun::components::LeafRotationQuat>> quaternions;

        /// Scaling factors.
        std::optional<Collection<rerun::components::LeafScale3D>> scales;

        /// 3x3 transformation matrices.
        std::optional<Collection<rerun::components::LeafTransformMat3x3>> mat3x3;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.LeafTransforms3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        LeafTransforms3D() = default;
        LeafTransforms3D(LeafTransforms3D&& other) = default;

        /// Translation vectors.
        LeafTransforms3D with_translations(
            Collection<rerun::components::LeafTranslation3D> _translations
        ) && {
            translations = std::move(_translations);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotations via axis + angle.
        LeafTransforms3D with_rotation_axis_angles(
            Collection<rerun::components::LeafRotationAxisAngle> _rotation_axis_angles
        ) && {
            rotation_axis_angles = std::move(_rotation_axis_angles);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotations via quaternion.
        LeafTransforms3D with_quaternions(
            Collection<rerun::components::LeafRotationQuat> _quaternions
        ) && {
            quaternions = std::move(_quaternions);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Scaling factors.
        LeafTransforms3D with_scales(Collection<rerun::components::LeafScale3D> _scales) && {
            scales = std::move(_scales);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// 3x3 transformation matrices.
        LeafTransforms3D with_mat3x3(Collection<rerun::components::LeafTransformMat3x3> _mat3x3
        ) && {
            mat3x3 = std::move(_mat3x3);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::LeafTransforms3D> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::LeafTransforms3D& archetype
        );
    };
} // namespace rerun
