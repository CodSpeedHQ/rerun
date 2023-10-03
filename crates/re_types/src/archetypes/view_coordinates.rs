// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// **Archetype**:  How we interpret the coordinate system of an entity/space.
///
/// For instance: What is "up"? What does the Z axis mean? Is this right-handed or left-handed?
///
/// The three coordinates are always ordered as [x, y, z].
///
/// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
/// down, and the Z axis points forward.
///
/// ## Example
///
/// ### `view_coordinates_simple`:
/// ```ignore
/// //! Change the view coordinates for the scene.
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) =
///         rerun::RecordingStreamBuilder::new("rerun_example_view_coordinates").memory()?;
///
///     rec.log_timeless("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?; // Set an up-axis
///     rec.log(
///         "world/xyz",
///         &rerun::Arrows3D::from_vectors(
///             [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], //
///         )
///         .with_colors([[255, 0, 0], [0, 255, 0], [0, 0, 255]]),
///     )?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct ViewCoordinates {
    pub xyz: crate::components::ViewCoordinates,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.ViewCoordinates".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.ViewCoordinatesIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.InstanceKey".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.ViewCoordinates".into(),
            "rerun.components.ViewCoordinatesIndicator".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

impl ViewCoordinates {
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`ViewCoordinates`] [`crate::Archetype`]
pub type ViewCoordinatesIndicator = crate::GenericIndicatorComponent<ViewCoordinates>;

impl crate::Archetype for ViewCoordinates {
    type Indicator = ViewCoordinatesIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.ViewCoordinates".into()
    }

    #[inline]
    fn indicator() -> crate::MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ViewCoordinatesIndicator = ViewCoordinatesIndicator::DEFAULT;
        crate::MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow(
        arrow_data: impl IntoIterator<
            Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>),
        >,
    ) -> crate::DeserializationResult<Self> {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let xyz = {
            let array = arrays_by_name
                .get("rerun.components.ViewCoordinates")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.ViewCoordinates#xyz")?;
            <crate::components::ViewCoordinates>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.ViewCoordinates#xyz")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.ViewCoordinates#xyz")?
        };
        Ok(Self { xyz })
    }
}

impl crate::AsComponents for ViewCoordinates {
    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.xyz as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }
}

impl ViewCoordinates {
    pub fn new(xyz: impl Into<crate::components::ViewCoordinates>) -> Self {
        Self { xyz: xyz.into() }
    }
}
