// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/rotation_axis_angle.fbs".

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

/// **Datatype**: 3D rotation represented by a rotation around a given axis.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct RotationAxisAngle {
    /// Axis to rotate around.
    ///
    /// This is not required to be normalized.
    /// If normalization fails (typically because the vector is length zero), the rotation is silently
    /// ignored.
    pub axis: crate::datatypes::Vec3D,

    /// How much to rotate around the axis.
    pub angle: crate::datatypes::Angle,
}

impl ::re_types_core::SizeBytes for RotationAxisAngle {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.axis.heap_size_bytes() + self.angle.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Vec3D>::is_pod() && <crate::datatypes::Angle>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(RotationAxisAngle);

impl ::re_types_core::Loggable for RotationAxisAngle {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.RotationAxisAngle".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new("axis", <crate::datatypes::Vec3D>::arrow_datatype(), false),
            Field::new("angle", <crate::datatypes::Angle>::arrow_datatype(), false),
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
                        let (somes, axis): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.axis.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let axis_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let axis_inner_data: Vec<_> = axis
                                .into_iter()
                                .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                .flatten()
                                .collect();
                            let axis_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                                axis_bitmap.as_ref().map(|bitmap| {
                                    bitmap
                                        .iter()
                                        .map(|b| std::iter::repeat(b).take(3usize))
                                        .flatten()
                                        .collect::<Vec<_>>()
                                        .into()
                                });
                            FixedSizeListArray::new(
                                DataType::FixedSizeList(
                                    std::sync::Arc::new(Field::new(
                                        "item",
                                        DataType::Float32,
                                        false,
                                    )),
                                    3usize,
                                ),
                                PrimitiveArray::new(
                                    DataType::Float32,
                                    axis_inner_data.into_iter().collect(),
                                    axis_inner_bitmap,
                                )
                                .boxed(),
                                axis_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, angle): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.angle.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let angle_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = angle_bitmap;
                            crate::datatypes::Angle::to_arrow_opt(angle)?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.RotationAxisAngle")?;
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
                let axis = {
                    if !arrays_by_name.contains_key("axis") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "axis",
                        ))
                        .with_context("rerun.datatypes.RotationAxisAngle");
                    }
                    let arrow_data = &**arrays_by_name["axis"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                let expected = DataType::FixedSizeList(
                                    std::sync::Arc::new(Field::new(
                                        "item",
                                        DataType::Float32,
                                        false,
                                    )),
                                    3usize,
                                );
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.datatypes.RotationAxisAngle#axis")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(3usize)
                                .zip((3usize..).step_by(3usize).take(arrow_data.len()));
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<Float32Array>()
                                    .ok_or_else(|| {
                                        let expected = DataType::Float32;
                                        let actual = arrow_data_inner.data_type().clone();
                                        DeserializationError::datatype_mismatch(expected, actual)
                                    })
                                    .with_context("rerun.datatypes.RotationAxisAngle#axis")?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end)| {
                                    debug_assert!(end - start == 3usize);
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);

                                    // NOTE: Unwrapping cannot fail: the length must be correct.
                                    #[allow(clippy::unwrap_used)]
                                    Ok(array_init::from_iter(data).unwrap())
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt.map(|res_or_opt| {
                                    res_or_opt.map(|v| crate::datatypes::Vec3D(v))
                                })
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let angle = {
                    if !arrays_by_name.contains_key("angle") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "angle",
                        ))
                        .with_context("rerun.datatypes.RotationAxisAngle");
                    }
                    let arrow_data = &**arrays_by_name["angle"];
                    crate::datatypes::Angle::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.RotationAxisAngle#angle")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(axis, angle),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(axis, angle)| {
                        Ok(Self {
                            axis: axis
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.RotationAxisAngle#axis")?,
                            angle: angle
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.RotationAxisAngle#angle")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.RotationAxisAngle")?
            }
        })
    }
}
