// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/transform3d.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/axis_length.hpp"
#include "../components/rotation_axis_angle.hpp"
#include "../components/rotation_quat.hpp"
#include "../components/scale3d.hpp"
#include "../components/transform3d.hpp"
#include "../components/transform_mat3x3.hpp"
#include "../components/translation3d.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../rerun_sdk_export.hpp"
#include "../result.hpp"
#include "../rotation3d.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A transform between two 3D spaces, i.e. a pose.
    ///
    /// All components are applied in the inverse order they are listed here.
    /// E.g. if both a 4x4 matrix with a translation and a translation vector are present,
    /// the translation is applied first, followed by the matrix.
    ///
    /// Each transform component can be listed multiple times, but transform tree propagation is only possible
    /// if there's only one instance for each transform component.
    /// TODO(#6831): write more about the exact interaction with the to be written `OutOfTreeTransform` component.
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
    ///     rec.log("base/translated", rerun::Transform3D::from_translation({1.0f, 0.0f, 0.0f}));
    ///     rec.log("base/translated", arrow);
    ///
    ///     rec.log(
    ///         "base/rotated_scaled",
    ///         rerun::Transform3D::from_rotation_scale(
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
    ///             rerun::Transform3D::from_translation_rotation(
    ///                 {std::sin(r_planet) * d_planet, std::cos(r_planet) * d_planet, 0.0f},
    ///                 rerun::RotationAxisAngle{
    ///                     {1.0, 0.0f, 0.0f},
    ///                     rerun::Angle::degrees(20.0f),
    ///                 }
    ///             )
    ///         );
    ///         rec.log(
    ///             "sun/planet/moon",
    ///             rerun::Transform3D::from_translation(
    ///                 {std::cos(r_moon) * d_moon, std::sin(r_moon) * d_moon, 0.0f}
    ///             )
    ///                 .with_from_parent(true)
    ///         );
    ///     }
    /// }
    /// ```
    struct Transform3D {
        /// The transform
        rerun::components::Transform3D transform;

        /// Translation vectors.
        std::optional<Collection<rerun::components::Translation3D>> translation;

        /// Rotation via axis + angle.
        std::optional<Collection<rerun::components::RotationAxisAngle>> rotation_axis_angle;

        /// Rotation via quaternion.
        std::optional<Collection<rerun::components::RotationQuat>> quaternion;

        /// Scaling factor.
        std::optional<Collection<rerun::components::Scale3D>> scale;

        /// 3x3 transformation matrices.
        std::optional<Collection<rerun::components::TransformMat3x3>> mat3x3;

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

        /// Creates a new 3D transform from translation and matrix provided as 3 columns.
        ///
        /// \param translation_ \çopydoc Transform3D::translation
        /// \param columns Column vectors of 3x3 matrix.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This overload is necessary, otherwise the array may be
        /// interpreted as bool and call the wrong overload.
        Transform3D(
            const components::Translation3D& translation_, const datatypes::Vec3D (&columns)[3],
            bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)),
              mat3x3(Collection<components::TransformMat3x3>::take_ownership(
                  components::TransformMat3x3(columns)
              )) {}

        /// Creates a new 3D transform from translation/matrix.
        ///
        /// \param translation_ \çopydoc Transform3D::translation
        /// \param mat3x3_ \copydoc Transform3D::mat3x3
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const components::Translation3D& translation_,
            const components::TransformMat3x3& mat3x3_, bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)),
              mat3x3(Collection<components::TransformMat3x3>::take_ownership(mat3x3_)) {}

        /// From a translation applied after a 3x3 matrix.
        ///
        /// \param translation \çopydoc Transform3D::translation
        /// \param mat3x3 \copydoc Transform3D::mat3x3
        static Transform3D from_translation_mat3x3(
            const components::Translation3D& translation, const components::TransformMat3x3& mat3x3
        ) {
            return Transform3D(translation, mat3x3, false);
        }

        /// From a translation applied after a 3x3 matrix provided as 3 columns.
        ///
        /// \param translation \çopydoc Transform3D::translation
        /// \param columns Column vectors of 3x3 matrix.
        static Transform3D from_translation_mat3x3(
            const components::Translation3D& translation, const datatypes::Vec3D (&columns)[3]
        ) {
            return Transform3D::from_translation_mat3x3(
                translation,
                components::TransformMat3x3(columns)
            );
        }

        /// From translation only.
        ///
        /// \param translation_ \çopydoc Transform3D::translation
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const components::Translation3D& translation_, bool from_parent = false)
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)) {}

        /// From a translation.
        ///
        /// \param translation \çopydoc Transform3D::translation
        static Transform3D from_translation(const components::Translation3D& translation) {
            return Transform3D(translation, false);
        }

        /// From 3x3 matrix only.
        ///
        /// \param mat3x3_ \copydoc Transform3D::mat3x3
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const components::TransformMat3x3& mat3x3_, bool from_parent = false)
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              mat3x3(Collection<components::TransformMat3x3>::take_ownership(mat3x3_)) {}

        /// From 3x3 matrix only.
        ///
        /// \param mat3x3 \copydoc Transform3D::mat3x3
        static Transform3D from_mat3x3(const components::TransformMat3x3& mat3x3) {
            return Transform3D(mat3x3, false);
        }

        /// From 3x3 matrix provided as 3 columns only.
        ///
        /// \param columns Column vectors of 3x3 matrix.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const datatypes::Vec3D (&columns)[3], bool from_parent = false)
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              mat3x3(Collection<components::TransformMat3x3>::take_ownership(
                  components::TransformMat3x3(columns)
              )) {}

        /// From 3x3 matrix provided as 3 columns only.
        ///
        /// \param columns Column vectors of 3x3 matrix.
        static Transform3D from_mat3x3(const datatypes::Vec3D (&columns)[3]) {
            return Transform3D::from_mat3x3(components::TransformMat3x3(columns));
        }

        /// Creates a new 3D transform from translation/rotation/scale.
        ///
        /// \param translation_ \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param scale_ \copydoc Transform3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const components::Translation3D& translation_, const Rotation3D& rotation,
            const components::Scale3D& scale_, bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)),
              scale(Collection<components::Scale3D>::take_ownership(scale_)) {
            set_rotation(rotation);
        }

        /// Creates a new 3D transform from translation/rotation/uniform-scale.
        ///
        /// \param translation_ \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(
            const components::Translation3D& translation_, const Rotation3D& rotation,
            float uniform_scale, bool from_parent = false
        )
            : Transform3D(translation_, rotation, components::Scale3D(uniform_scale), from_parent) {
        }

        /// From a translation, applied after a rotation & scale, known as an affine transformation.
        ///
        /// \param translation \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param scale \copydoc Transform3D::scale
        static Transform3D from_translation_rotation_scale(
            const components::Translation3D& translation, const Rotation3D& rotation,
            const components::Scale3D& scale
        ) {
            return Transform3D(translation, rotation, scale, false);
        }

        /// From a translation, applied after a rotation & scale, known as an affine transformation.
        ///
        /// \param translation \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        static Transform3D from_translation_rotation_scale(
            const components::Translation3D& translation, const Rotation3D& rotation,
            float uniform_scale
        ) {
            return Transform3D(translation, rotation, components::Scale3D(uniform_scale), false);
        }

        /// Creates a new rigid transform (translation & rotation only).
        ///
        /// \param translation_ \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const components::Translation3D& translation_, const Rotation3D& rotation,
            bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)) {
            set_rotation(rotation);
        }

        /// From a rotation & scale.
        ///
        /// \param translation \copydoc Transform3D::translation
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        static Transform3D from_translation_rotation(
            const components::Translation3D& translation, const Rotation3D& rotation
        ) {
            return Transform3D(translation, rotation, false);
        }

        /// From translation & scale only.
        ///
        /// \param translation_ \copydoc Transform3D::translation
        /// \param scale_ Transform3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const components::Translation3D& translation_, const components::Scale3D& scale_,
            bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              translation(Collection<components::Translation3D>::take_ownership(translation_)),
              scale(Collection<components::Scale3D>::take_ownership(scale_)) {}

        /// From a translation applied after a scale.
        ///
        /// \param translation \copydoc Transform3D::translation
        /// \param scale Transform3D::scale
        static Transform3D from_translation_scale(
            const components::Translation3D& translation, const components::Scale3D& scale
        ) {
            return Transform3D(translation, scale, false);
        }

        /// From translation & uniform scale only.
        ///
        /// \param translation_ \copydoc Transform3D::translation
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(
            const components::Translation3D& translation_, float uniform_scale,
            bool from_parent = false
        )
            : Transform3D(translation_, components::Scale3D(uniform_scale), from_parent) {}

        /// From rotation & scale.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param scale_ Transform3D::scale
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(
            const Rotation3D& rotation, const components::Scale3D& scale_, bool from_parent = false
        )
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              scale(Collection<components::Scale3D>::take_ownership(scale_)) {
            set_rotation(rotation);
        }

        /// From rotation & uniform scale.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        ///
        /// _Implementation note:_ This explicit overload prevents interpretation of the float as
        /// bool, leading to a call to the wrong overload.
        Transform3D(const Rotation3D& rotation, float uniform_scale, bool from_parent = false)
            : Transform3D(rotation, components::Scale3D(uniform_scale), from_parent) {}

        /// From a rotation & scale.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param scale Transform3D::scale
        static Transform3D from_rotation_scale(
            const Rotation3D& rotation, const components::Scale3D& scale
        ) {
            return Transform3D(rotation, scale, false);
        }

        /// From a rotation & uniform scale.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        static Transform3D from_rotation_scale(const Rotation3D& rotation, float uniform_scale) {
            return Transform3D(rotation, components::Scale3D(uniform_scale), false);
        }

        /// From rotation only.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        /// \param from_parent \copydoc datatypes::TranslationRotationScale3D::from_parent
        Transform3D(const Rotation3D& rotation, bool from_parent = false)
            : transform(datatypes::TranslationRotationScale3D(from_parent)) {
            set_rotation(rotation);
        }

        /// From rotation only.
        ///
        /// \param rotation \copydoc Rotation represented either as a quaternion or axis + angle rotation.
        static Transform3D from_rotation(const Rotation3D& rotation) {
            return Transform3D(rotation, false);
        }

        /// From scale only.
        ///
        /// \param scale_ \copydoc datatypes::TranslationRotationScale3D::from_parent
        /// \param from_parent \copydoc Transform3D::scale
        Transform3D(const components::Scale3D& scale_, bool from_parent = false)
            : transform(datatypes::TranslationRotationScale3D(from_parent)),
              scale(Collection<components::Scale3D>::take_ownership(scale_)) {}

        /// From scale only.
        ///
        /// \param scale Transform3D::scale
        static Transform3D from_scale(const components::Scale3D& scale) {
            return Transform3D(scale, false);
        }

        /// From scale only.
        ///
        /// \param uniform_scale Uniform scale factor that is applied to all axis equally.
        static Transform3D from_scale(float uniform_scale) {
            return Transform3D(components::Scale3D(uniform_scale), false);
        }

        /// TODO(#6831): Should be autogenerated once from_parent component is introduced
        Transform3D with_from_parent(bool from_parent) && {
            auto translation_rotation_scale = transform.repr.get_translation_rotation_scale();
            if (translation_rotation_scale != nullptr) {
                auto cpy = *translation_rotation_scale;
                cpy.from_parent = from_parent;
                transform = cpy;
            }
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

      private:
        /// Set the rotation component of the transform using the `rerun::Rotation3D` utility.
        void set_rotation(const Rotation3D& rotation) {
            if (rotation.axis_angle.has_value()) {
                rotation_axis_angle = Collection<components::RotationAxisAngle>::take_ownership(
                    rotation.axis_angle.value()
                );
            }
            if (rotation.quaternion.has_value()) {
                quaternion =
                    Collection<components::RotationQuat>::take_ownership(rotation.quaternion.value()
                    );
            }
        }

      public:
        Transform3D() = default;
        Transform3D(Transform3D&& other) = default;

        explicit Transform3D(rerun::components::Transform3D _transform)
            : transform(std::move(_transform)) {}

        /// Translation vectors.
        Transform3D with_translation(Collection<rerun::components::Translation3D> _translation) && {
            translation = std::move(_translation);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotation via axis + angle.
        Transform3D with_rotation_axis_angle(
            Collection<rerun::components::RotationAxisAngle> _rotation_axis_angle
        ) && {
            rotation_axis_angle = std::move(_rotation_axis_angle);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotation via quaternion.
        Transform3D with_quaternion(Collection<rerun::components::RotationQuat> _quaternion) && {
            quaternion = std::move(_quaternion);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Scaling factor.
        Transform3D with_scale(Collection<rerun::components::Scale3D> _scale) && {
            scale = std::move(_scale);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// 3x3 transformation matrices.
        Transform3D with_mat3x3(Collection<rerun::components::TransformMat3x3> _mat3x3) && {
            mat3x3 = std::move(_mat3x3);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

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
