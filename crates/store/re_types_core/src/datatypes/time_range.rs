// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/visible_time_range.fbs".

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

use crate::external::arrow2;
use crate::ComponentName;
use crate::SerializationResult;
use crate::{ComponentBatch, MaybeOwnedComponentBatch};
use crate::{DeserializationError, DeserializationResult};

/// **Datatype**: Visible time range bounds for a specific timeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeRange {
    /// Low time boundary for sequence timeline.
    pub start: crate::datatypes::TimeRangeBoundary,

    /// High time boundary for sequence timeline.
    pub end: crate::datatypes::TimeRangeBoundary,
}

impl crate::SizeBytes for TimeRange {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.start.heap_size_bytes() + self.end.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::TimeRangeBoundary>::is_pod()
            && <crate::datatypes::TimeRangeBoundary>::is_pod()
    }
}

crate::macros::impl_into_cow!(TimeRange);

impl crate::Loggable for TimeRange {
    type Name = crate::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TimeRange".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new(
                "start",
                <crate::datatypes::TimeRangeBoundary>::arrow_datatype(),
                false,
            ),
            Field::new(
                "end",
                <crate::datatypes::TimeRangeBoundary>::arrow_datatype(),
                false,
            ),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        use crate::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, start): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.start.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let start_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = start_bitmap;
                            crate::datatypes::TimeRangeBoundary::to_arrow_opt(start)?
                        }
                    },
                    {
                        let (somes, end): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.end.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let end_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = end_bitmap;
                            crate::datatypes::TimeRangeBoundary::to_arrow_opt(end)?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use crate::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.TimeRange")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let start = {
                    if !arrays_by_name.contains_key("start") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "start",
                        ))
                        .with_context("rerun.datatypes.TimeRange");
                    }
                    let arrow_data = &**arrays_by_name["start"];
                    crate::datatypes::TimeRangeBoundary::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.TimeRange#start")?
                        .into_iter()
                };
                let end = {
                    if !arrays_by_name.contains_key("end") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "end",
                        ))
                        .with_context("rerun.datatypes.TimeRange");
                    }
                    let arrow_data = &**arrays_by_name["end"];
                    crate::datatypes::TimeRangeBoundary::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.TimeRange#end")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(start, end),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(start, end)| {
                        Ok(Self {
                            start: start
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TimeRange#start")?,
                            end: end
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TimeRange#end")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.TimeRange")?
            }
        })
    }
}
