// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/tensor_view_fit.fbs".

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

/// **Archetype**: Configures how a selected tensor slice is shown on screen.
#[derive(Clone, Debug, Default)]
pub struct TensorViewFit {
    /// How the image is scaled to fit the view.
    pub scaling: Option<crate::blueprint::components::ViewFit>,
}

impl ::re_types_core::SizeBytes for TensorViewFit {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.scaling.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::ViewFit>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.TensorViewFitIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.ViewFit".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.TensorViewFitIndicator".into(),
            "rerun.blueprint.components.ViewFit".into(),
        ]
    });

impl TensorViewFit {
    /// The total number of components in the archetype: 0 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`TensorViewFit`] [`::re_types_core::Archetype`]
pub type TensorViewFitIndicator = ::re_types_core::GenericIndicatorComponent<TensorViewFit>;

impl ::re_types_core::Archetype for TensorViewFit {
    type Indicator = TensorViewFitIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.TensorViewFit".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Tensor view fit"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: TensorViewFitIndicator = TensorViewFitIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let scaling = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ViewFit")
        {
            <crate::blueprint::components::ViewFit>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.TensorViewFit#scaling")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { scaling })
    }
}

impl ::re_types_core::AsComponents for TensorViewFit {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.scaling
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for TensorViewFit {}

impl TensorViewFit {
    /// Create a new `TensorViewFit`.
    #[inline]
    pub fn new() -> Self {
        Self { scaling: None }
    }

    /// How the image is scaled to fit the view.
    #[inline]
    pub fn with_scaling(
        mut self,
        scaling: impl Into<crate::blueprint::components::ViewFit>,
    ) -> Self {
        self.scaling = Some(scaling.into());
        self
    }
}
