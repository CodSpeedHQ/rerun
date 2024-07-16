// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/transform3d.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/axis_length.hpp"
#include "../components/transform3d.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../rerun_sdk_export.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A transform between two 3D spaces, i.e. a pose.
    ///
    /// ## Examples
    ///
    /// ### Variety of 3D transforms
    /// ![image](https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// constexpr float TAU = 6.28318530717958647692528676655900577f;
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_transform3d");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     auto arrow =
    ///         rerun::Arrows3D::from_vectors({{0.0f, 1.0f, 0.0f}}).with_origins({{0.0f, 0.0f, 0.0f}});
    ///
    ///     rec.log("base", arrow);
    ///
    ///     rec.log("base/translated", rerun::Transform3D({1.0f, 0.0f, 0.0f}));
    ///     rec.log("base/translated", arrow);
    ///
    ///     rec.log(
    ///         "base/rotated_scaled",
    ///         rerun::Transform3D(
    ///             rerun::RotationAxisAngle({0.0f, 0.0f, 1.0f}, rerun::Angle::radians(TAU / 8.0f)),
    ///             2.0f
    ///         )
    ///     );
    ///     rec.log("base/rotated_scaled", arrow);
    /// }
    /// ```
    ///
    /// ### Transform hierarchy
    /// ![image](https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// constexpr float TAU = 6.28318530717958647692528676655900577f;
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_transform3d_hierarchy");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // TODO(#5521): log two space views as in the python example
    ///
    ///     rec.set_time_seconds("sim_time", 0.0);
    ///
    ///     // Planetary motion is typically in the XY plane.
    ///     rec.log_static("/", rerun::ViewCoordinates::RIGHT_HAND_Z_UP);
    ///
    ///     // Setup points, all are in the center of their own space:
    ///     rec.log(
    ///         "sun",
    ///         rerun::Points3D({{0.0f, 0.0f, 0.0f}})
    ///             .with_radii({1.0f})
    ///             .with_colors({rerun::Color(255, 200, 10)})
    ///     );
    ///     rec.log(
    ///         "sun/planet",
    ///         rerun::Points3D({{0.0f, 0.0f, 0.0f}})
    ///             .with_radii({0.4f})
    ///             .with_colors({rerun::Color(40, 80, 200)})
    ///     );
    ///     rec.log(
    ///         "sun/planet/moon",
    ///         rerun::Points3D({{0.0f, 0.0f, 0.0f}})
    ///             .with_radii({0.15f})
    ///             .with_colors({rerun::Color(180, 180, 180)})
    ///     );
    ///
    ///     // Draw fixed paths where the planet & moon move.
    ///     float d_planet = 6.0f;
    ///     float d_moon = 3.0f;
    ///     std::vector<std::array<float, 3>> planet_path, moon_path;
    ///     for (int i = 0; i <= 100; i++) {
    ///         float angle = static_cast<float>(i) * 0.01f * TAU;
    ///         float circle_x = std::sin(angle);
    ///         float circle_y = std::cos(angle);
    ///         planet_path.push_back({circle_x * d_planet, circle_y * d_planet, 0.0f});
    ///         moon_path.push_back({circle_x * d_moon, circle_y * d_moon, 0.0f});
    ///     }
    ///     rec.log("sun/planet_path", rerun::LineStrips3D(rerun::LineStrip3D(planet_path)));
    ///     rec.log("sun/planet/moon_path", rerun::LineStrips3D(rerun::LineStrip3D(moon_path)));
    ///
    ///     // Movement via transforms.
    ///     for (int i = 0; i <6 * 120; i++) {
    ///         float time = static_cast<float>(i) / 120.0f;
    ///         rec.set_time_seconds("sim_time", time);
    ///         float r_moon = time * 5.0f;
    ///         float r_planet = time * 2.0f;
    ///
    ///         rec.log(
    ///             "sun/planet",
    ///             rerun::Transform3D(
    ///                 {std::sin(r_planet) * d_planet, std::cos(r_planet) * d_planet, 0.0f},
    ///                 rerun::RotationAxisAngle{
    ///                     {1.0, 0.0f, 0.0f},
    ///                     rerun::Angle::degrees(20.0f),
    ///                 }
    ///             )
    ///         );
    ///         rec.log(
    ///             "sun/planet/moon",
    ///             rerun::Transform3D(
    ///                 {
    ///                     std::cos(r_moon) * d_moon,
    ///                     std::sin(r_moon) * d_moon,
    ///                     0.0f,
    ///                 },
    ///                 true
    ///             )
    ///         );
    ///     }
    /// }
    /// ```
    struct Transform3D {
        /// The transform
        rerun::components::Transform3D transform;

        /// Visual length of the 3 axes.
        ///
        /// The length is interpreted in the local coordinate system of the transform.
        /// If the transform is scaled, the axes will be scaled accordingly.
        std::optional<rerun::components::AxisLength> axis_length;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.Transform3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        // Extensions to generated type defined in 'transform3d_ext.cpp'

        /// Identity transformation.
        ///
        /// Applying this transform does not alter an entity's transformation.
        RERUN_SDK_EXPORT static const Transform3D IDENTITY;

        /// New 3D transform from translation/matrix datatype.
        ///
        /// \param translation_and_mat3x3 Combined translation/matrix.
        Transform3D(const datatypes::TranslationAndMat3x3& translation_and_mat3x3)
            : Transform3D(datatypes::Transform3D::translation_and_mat3x3(translation_and_mat3x3)) {}

        /// Creates a new 3D transform from translation and matrix provided as 3 columns.
        ///
        /// \param translation \çopydoc datatypes::TranslationAndMat3x3::translation
        /// \param columns Column vectors of 3x3 matrix.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This overload is necessary, otherwise the array may be
        /// interpreted as bool and call the wrong overload.
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Vec3D (&columns)[3],
            bool from_parent = false
        )
            : Transform3D(datatypes::TranslationAndMat3x3(translation, columns, from_parent)) {}

        /// Creates a new 3D transform from translation/matrix.
        ///
        /// \param translation \çopydoc datatypes::TranslationAndMat3x3::translation
        /// \param matrix \copydoc datatypes::TranslationAndMat3x3::mat3x3
        /// \param from_parent \copydoc datatypes::TranslationAndMat3x3::from_parent
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Mat3x3& matrix,
            bool from_parent = false
        )
            : Transform3D(datatypes::TranslationAndMat3x3(translation, matrix, from_parent)) {}

        /// From translation only.
        ///
        /// \param translation \çopydoc datatypes::TranslationRotationScale3D::translation
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const datatypes::Vec3D& translation, bool from_parent = false)
            : Transform3D(datatypes::TranslationRotationScale3D(translation, from_parent)) {}

        /// From 3x3 matrix only.
        ///
        /// \param matrix \copydoc datatypes::TranslationAndMat3x3::mat3x3
        /// \param from_parent \copydoc datatypes::TranslationAndMat3x3::from_parent
        Transform3D(const datatypes::Mat3x3& matrix, bool from_parent = false)
            : Transform3D(datatypes::TranslationAndMat3x3(matrix, from_parent)) {}

        /// From 3x3 matrix provided as 3 columns only.
        ///
        /// \param columns Column vectors of 3x3 matrix.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const datatypes::Vec3D (&columns)[3], bool from_parent = false)
            : Transform3D(datatypes::TranslationAndMat3x3(columns, from_parent)) {}

        /// New 3D transform from translation/rotation/scale datatype.
        /// \param translation_rotation_scale3d Combined translation/rotation/scale.
        Transform3D(const datatypes::TranslationRotationScale3D& translation_rotation_scale3d)
            : Transform3D(
                  datatypes::Transform3D::translation_rotation_scale(translation_rotation_scale3d)
              ) {}

        /// Creates a new 3D transform from translation/rotation/scale.
        ///
        /// \param translation \copydoc datatypes::TranslationRotationScale3D::translation
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param scale \copydoc datatypes::TranslationRotationScale3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
            const datatypes::Scale3D& scale, bool from_parent = false
        )
            : Transform3D(
                  datatypes::TranslationRotationScale3D(translation, rotation, scale, from_parent)
              ) {}

        /// Creates a new 3D transform from translation/rotation/uniform-scale.
        ///
        /// \param translation \copydoc datatypes::TranslationRotationScale3D::translation
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
            float uniform_scale, bool from_parent = false
        )
            : Transform3D(datatypes::TranslationRotationScale3D(
                  translation, rotation, uniform_scale, from_parent
              )) {}

        /// Creates a new rigid transform (translation & rotation only).
        ///
        /// \param translation \copydoc datatypes::TranslationRotationScale3D::translation
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
            bool from_parent = false
        )
            : Transform3D(datatypes::TranslationRotationScale3D(translation, rotation, from_parent)
              ) {}

        /// From translation & scale only.
        ///
        /// \param translation \copydoc datatypes::TranslationRotationScale3D::translation
        /// \param scale datatypes::TranslationRotationScale3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const datatypes::Vec3D& translation, const datatypes::Scale3D& scale,
            bool from_parent = false
        )
            : Transform3D(datatypes::TranslationRotationScale3D(translation, scale, from_parent)) {}

        /// From translation & uniform scale only.
        ///
        /// \param translation \copydoc datatypes::TranslationRotationScale3D::translation
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(
            const datatypes::Vec3D& translation, float uniform_scale, bool from_parent = false
        )
            : Transform3D(
                  datatypes::TranslationRotationScale3D(translation, uniform_scale, from_parent)
              ) {}

        /// From rotation & scale.
        ///
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param scale datatypes::TranslationRotationScale3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const datatypes::Rotation3D& rotation, const datatypes::Scale3D& scale,
            bool from_parent = false
        )
            : Transform3D(datatypes::TranslationRotationScale3D(rotation, scale, from_parent)) {}

        /// From rotation & uniform scale.
        ///
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(
            const datatypes::Rotation3D& rotation, float uniform_scale, bool from_parent = false
        )
            : Transform3D(
                  datatypes::TranslationRotationScale3D(rotation, uniform_scale, from_parent)
              ) {}

        /// From rotation only.
        ///
        /// \param rotation \copydoc datatypes::TranslationRotationScale3D::rotation
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const datatypes::Rotation3D& rotation, bool from_parent = false)
            : Transform3D(datatypes::TranslationRotationScale3D(rotation, from_parent)) {}

        /// From scale only.
        ///
        /// \param scale \copydoc datatypes::TranslationRotationScale3D::from_parent
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::scale
        Transform3D(const datatypes::Scale3D& scale, bool from_parent = false)
            : Transform3D(datatypes::TranslationRotationScale3D(scale, from_parent)) {}

      public:
        Transform3D() = default;
        Transform3D(Transform3D&& other) = default;

        explicit Transform3D(rerun::components::Transform3D _transform)
            : transform(std::move(_transform)) {}

        /// Visual length of the 3 axes.
        ///
        /// The length is interpreted in the local coordinate system of the transform.
        /// If the transform is scaled, the axes will be scaled accordingly.
        Transform3D with_axis_length(rerun::components::AxisLength _axis_length) && {
            axis_length = std::move(_axis_length);
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
    struct AsComponents<archetypes::Transform3D> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::Transform3D& archetype);
    };
} // namespace rerun
