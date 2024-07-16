// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/rotation3d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A 3D rotation, represented either by a quaternion or a rotation around axis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rotation3D(
    /// Representation of the rotation.
    pub crate::datatypes::Rotation3D,
);

impl ::re_types_core::SizeBytes for Rotation3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Rotation3D>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Rotation3D>> From<T> for Rotation3D {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Rotation3D> for Rotation3D {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Rotation3D {
        &self.0
    }
}

impl std::ops::Deref for Rotation3D {
    type Target = crate::datatypes::Rotation3D;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Rotation3D {
        &self.0
    }
}

impl std::ops::DerefMut for Rotation3D {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Rotation3D {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(Rotation3D);

impl ::re_types_core::Loggable for Rotation3D {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Rotation3D".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::Rotation3D::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Rotation3D::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::Rotation3D::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}
