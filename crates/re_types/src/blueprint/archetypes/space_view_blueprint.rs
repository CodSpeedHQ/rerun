// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/space_view_blueprint.fbs".

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

/// **Archetype**: The top-level description of the Viewport.
#[derive(Clone, Debug)]
pub struct SpaceViewBlueprint {
    /// The class of the view.
    pub class_identifier: crate::blueprint::components::SpaceViewClass,

    /// The name of the view.
    pub display_name: Option<crate::components::Name>,

    /// The "anchor point" of this space view.
    ///
    /// Defaults to the root path '/' if not specified.
    ///
    /// The transform at this path forms the reference point for all scene->world transforms in this space view.
    /// I.e. the position of this entity path in space forms the origin of the coordinate system in this space view.
    /// Furthermore, this is the primary indicator for heuristics on what entities we show in this space view.
    pub space_origin: Option<crate::blueprint::components::SpaceViewOrigin>,

    /// Whether this space view is visible.
    ///
    /// Defaults to true if not specified.
    pub visible: Option<crate::blueprint::components::Visible>,
}

impl ::re_types_core::SizeBytes for SpaceViewBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.class_identifier.heap_size_bytes()
            + self.display_name.heap_size_bytes()
            + self.space_origin.heap_size_bytes()
            + self.visible.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::components::SpaceViewClass>::is_pod()
            && <Option<crate::components::Name>>::is_pod()
            && <Option<crate::blueprint::components::SpaceViewOrigin>>::is_pod()
            && <Option<crate::blueprint::components::Visible>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.SpaceViewClass".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(
        || ["rerun.blueprint.components.SpaceViewBlueprintIndicator".into()],
    );

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.SpaceViewOrigin".into(),
            "rerun.blueprint.components.Visible".into(),
            "rerun.components.Name".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.SpaceViewClass".into(),
            "rerun.blueprint.components.SpaceViewBlueprintIndicator".into(),
            "rerun.blueprint.components.SpaceViewOrigin".into(),
            "rerun.blueprint.components.Visible".into(),
            "rerun.components.Name".into(),
        ]
    });

impl SpaceViewBlueprint {
    /// The total number of components in the archetype: 1 required, 1 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`SpaceViewBlueprint`] [`::re_types_core::Archetype`]
pub type SpaceViewBlueprintIndicator =
    ::re_types_core::GenericIndicatorComponent<SpaceViewBlueprint>;

impl ::re_types_core::Archetype for SpaceViewBlueprint {
    type Indicator = SpaceViewBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.SpaceViewBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Space view blueprint"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: SpaceViewBlueprintIndicator = SpaceViewBlueprintIndicator::DEFAULT;
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
        let class_identifier = {
            let array = arrays_by_name
                .get("rerun.blueprint.components.SpaceViewClass")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#class_identifier")?;
            <crate::blueprint::components::SpaceViewClass>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#class_identifier")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#class_identifier")?
        };
        let display_name = if let Some(array) = arrays_by_name.get("rerun.components.Name") {
            <crate::components::Name>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#display_name")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let space_origin =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.SpaceViewOrigin") {
                <crate::blueprint::components::SpaceViewOrigin>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#space_origin")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let visible = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.Visible")
        {
            <crate::blueprint::components::Visible>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.SpaceViewBlueprint#visible")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            class_identifier,
            display_name,
            space_origin,
            visible,
        })
    }
}

impl ::re_types_core::AsComponents for SpaceViewBlueprint {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.class_identifier as &dyn ComponentBatch).into()),
            self.display_name
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.space_origin
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.visible
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl SpaceViewBlueprint {
    /// Create a new `SpaceViewBlueprint`.
    #[inline]
    pub fn new(class_identifier: impl Into<crate::blueprint::components::SpaceViewClass>) -> Self {
        Self {
            class_identifier: class_identifier.into(),
            display_name: None,
            space_origin: None,
            visible: None,
        }
    }

    /// The name of the view.
    #[inline]
    pub fn with_display_name(mut self, display_name: impl Into<crate::components::Name>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// The "anchor point" of this space view.
    ///
    /// Defaults to the root path '/' if not specified.
    ///
    /// The transform at this path forms the reference point for all scene->world transforms in this space view.
    /// I.e. the position of this entity path in space forms the origin of the coordinate system in this space view.
    /// Furthermore, this is the primary indicator for heuristics on what entities we show in this space view.
    #[inline]
    pub fn with_space_origin(
        mut self,
        space_origin: impl Into<crate::blueprint::components::SpaceViewOrigin>,
    ) -> Self {
        self.space_origin = Some(space_origin.into());
        self
    }

    /// Whether this space view is visible.
    ///
    /// Defaults to true if not specified.
    #[inline]
    pub fn with_visible(
        mut self,
        visible: impl Into<crate::blueprint::components::Visible>,
    ) -> Self {
        self.visible = Some(visible.into());
        self
    }
}
