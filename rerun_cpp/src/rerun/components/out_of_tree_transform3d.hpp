// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/out_of_tree_transform3d.fbs".

#pragma once

#include "../datatypes/transform3d.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: An out-of-tree affine transform between two 3D spaces, represented in a given direction.
    ///
    /// "Out-of-tree" means that the transform only affects its own entity: children don't inherit from it.
    struct OutOfTreeTransform3D {
        /// Representation of the transform.
        rerun::datatypes::Transform3D repr;

      public:
        OutOfTreeTransform3D() = default;

        OutOfTreeTransform3D(rerun::datatypes::Transform3D repr_) : repr(repr_) {}

        OutOfTreeTransform3D& operator=(rerun::datatypes::Transform3D repr_) {
            repr = repr_;
            return *this;
        }

        /// Cast to the underlying Transform3D datatype
        operator rerun::datatypes::Transform3D() const {
            return repr;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(
        sizeof(rerun::datatypes::Transform3D) == sizeof(components::OutOfTreeTransform3D)
    );

    /// \private
    template <>
    struct Loggable<components::OutOfTreeTransform3D> {
        static constexpr const char Name[] = "rerun.components.OutOfTreeTransform3D";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::Transform3D>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::OutOfTreeTransform3D` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::OutOfTreeTransform3D* instances, size_t num_instances
        ) {
            return Loggable<rerun::datatypes::Transform3D>::to_arrow(
                &instances->repr,
                num_instances
            );
        }
    };
} // namespace rerun
