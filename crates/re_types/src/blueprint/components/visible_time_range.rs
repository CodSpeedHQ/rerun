// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/visible_time_range.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: The range of values on a given timeline that will be included in a view's query.
///
/// Refer to `VisibleTimeRanges` archetype for more information.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct VisibleTimeRange(pub crate::datatypes::VisibleTimeRange);

impl ::re_types_core::SizeBytes for VisibleTimeRange {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::VisibleTimeRange>::is_pod()
    }
}

impl<T: Into<crate::datatypes::VisibleTimeRange>> From<T> for VisibleTimeRange {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::VisibleTimeRange> for VisibleTimeRange {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::VisibleTimeRange {
        &self.0
    }
}

impl std::ops::Deref for VisibleTimeRange {
    type Target = crate::datatypes::VisibleTimeRange;

    #[inline]
    fn deref(&self) -> &crate::datatypes::VisibleTimeRange {
        &self.0
    }
}

::re_types_core::macros::impl_into_cow!(VisibleTimeRange);

impl ::re_types_core::Loggable for VisibleTimeRange {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.components.VisibleTimeRange".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new(
                "timeline",
                <crate::datatypes::Utf8>::arrow_datatype(),
                false,
            ),
            Field::new(
                "range",
                <crate::datatypes::TimeRange>::arrow_datatype(),
                false,
            ),
        ]))
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                _ = data0_bitmap;
                crate::datatypes::VisibleTimeRange::to_arrow_opt(data0)?
            }
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(
            crate::datatypes::VisibleTimeRange::from_arrow_opt(arrow_data)
                .with_context("rerun.blueprint.components.VisibleTimeRange#value")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .map(|res| res.map(|v| Some(Self(v))))
                .collect::<DeserializationResult<Vec<Option<_>>>>()
                .with_context("rerun.blueprint.components.VisibleTimeRange#value")
                .with_context("rerun.blueprint.components.VisibleTimeRange")?,
        )
    }
}
