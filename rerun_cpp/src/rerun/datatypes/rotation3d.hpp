// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/rotation3d.fbs".

#pragma once

#include "../rerun_sdk_export.hpp"
#include "../result.hpp"
#include "quaternion.hpp"
#include "rotation_axis_angle.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class DenseUnionBuilder;
} // namespace arrow

namespace rerun::datatypes {
    namespace detail {
        /// \private
        enum class Rotation3DTag : uint8_t {
            /// Having a special empty state makes it possible to implement move-semantics. We need to be able to leave the object in a state which we can run the destructor on.
            None = 0,
            Quaternion,
            AxisAngle,
        };

        /// \private
        union Rotation3DData {
            /// Rotation defined by a quaternion.
            rerun::datatypes::Quaternion quaternion;

            /// Rotation defined with an axis and an angle.
            rerun::datatypes::RotationAxisAngle axis_angle;

            Rotation3DData() {
                std::memset(reinterpret_cast<void*>(this), 0, sizeof(Rotation3DData));
            }

            ~Rotation3DData() {}

            void swap(Rotation3DData& other) noexcept {
                // This bitwise swap would fail for self-referential types, but we don't have any of those.
                char temp[sizeof(Rotation3DData)];
                void* otherbytes = reinterpret_cast<void*>(&other);
                void* thisbytes = reinterpret_cast<void*>(this);
                std::memcpy(temp, thisbytes, sizeof(Rotation3DData));
                std::memcpy(thisbytes, otherbytes, sizeof(Rotation3DData));
                std::memcpy(otherbytes, temp, sizeof(Rotation3DData));
            }
        };
    } // namespace detail

    /// **Datatype**: A 3D rotation.
    struct Rotation3D {
        Rotation3D() : _tag(detail::Rotation3DTag::None) {}

        /// Copy constructor
        Rotation3D(const Rotation3D& other) : _tag(other._tag) {
            const void* otherbytes = reinterpret_cast<const void*>(&other._data);
            void* thisbytes = reinterpret_cast<void*>(&this->_data);
            std::memcpy(thisbytes, otherbytes, sizeof(detail::Rotation3DData));
        }

        Rotation3D& operator=(const Rotation3D& other) noexcept {
            Rotation3D tmp(other);
            this->swap(tmp);
            return *this;
        }

        Rotation3D(Rotation3D&& other) noexcept : Rotation3D() {
            this->swap(other);
        }

        Rotation3D& operator=(Rotation3D&& other) noexcept {
            this->swap(other);
            return *this;
        }

      public: // START of extensions from rotation3d_ext.cpp:
        RERUN_SDK_EXPORT static const Rotation3D IDENTITY;

        // END of extensions from rotation3d_ext.cpp, start of generated code:

        void swap(Rotation3D& other) noexcept {
            std::swap(this->_tag, other._tag);
            this->_data.swap(other._data);
        }

        /// Rotation defined by a quaternion.
        Rotation3D(rerun::datatypes::Quaternion quaternion) : Rotation3D() {
            *this = Rotation3D::quaternion(std::move(quaternion));
        }

        /// Rotation defined with an axis and an angle.
        Rotation3D(rerun::datatypes::RotationAxisAngle axis_angle) : Rotation3D() {
            *this = Rotation3D::axis_angle(std::move(axis_angle));
        }

        /// Rotation defined by a quaternion.
        static Rotation3D quaternion(rerun::datatypes::Quaternion quaternion) {
            Rotation3D self;
            self._tag = detail::Rotation3DTag::Quaternion;
            new (&self._data.quaternion) rerun::datatypes::Quaternion(std::move(quaternion));
            return self;
        }

        /// Rotation defined with an axis and an angle.
        static Rotation3D axis_angle(rerun::datatypes::RotationAxisAngle axis_angle) {
            Rotation3D self;
            self._tag = detail::Rotation3DTag::AxisAngle;
            new (&self._data.axis_angle) rerun::datatypes::RotationAxisAngle(std::move(axis_angle));
            return self;
        }

        /// Return a pointer to quaternion if the union is in that state, otherwise `nullptr`.
        const rerun::datatypes::Quaternion* get_quaternion() const {
            if (_tag == detail::Rotation3DTag::Quaternion) {
                return &_data.quaternion;
            } else {
                return nullptr;
            }
        }

        /// Return a pointer to axis_angle if the union is in that state, otherwise `nullptr`.
        const rerun::datatypes::RotationAxisAngle* get_axis_angle() const {
            if (_tag == detail::Rotation3DTag::AxisAngle) {
                return &_data.axis_angle;
            } else {
                return nullptr;
            }
        }

        /// \private
        const detail::Rotation3DData& get_union_data() const {
            return _data;
        }

        /// \private
        detail::Rotation3DTag get_union_tag() const {
            return _tag;
        }

      private:
        detail::Rotation3DTag _tag;
        detail::Rotation3DData _data;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Rotation3D> {
        static constexpr const char Name[] = "rerun.datatypes.Rotation3D";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Rotation3D` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Rotation3D* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::DenseUnionBuilder* builder, const datatypes::Rotation3D* elements,
            size_t num_elements
        );
    };
} // namespace rerun
