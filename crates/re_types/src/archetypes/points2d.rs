// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points2d.fbs".

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

/// **Archetype**:  A 2D point cloud with positions and optional colors, radii, labels, etc.
///
/// ## Examples
///
/// ### `point2d_simple`:
/// ```ignore
/// //! Log some very simple points.
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) = rerun::RecordingStreamBuilder::new("rerun_example_points2d").memory()?;
///
///     rec.log("points", &rerun::Points2D::new([(0.0, 0.0), (1.0, 1.0)]))?;
///
///     // Log an extra rect to set the view bounds
///     rec.log("bounds", &rerun::Boxes2D::from_half_sizes([(2.0, 1.5)]))?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point2d_simple/a8e801958bce5aa4e080659c033630f86ce95f71/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point2d_simple/a8e801958bce5aa4e080659c033630f86ce95f71/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point2d_simple/a8e801958bce5aa4e080659c033630f86ce95f71/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point2d_simple/a8e801958bce5aa4e080659c033630f86ce95f71/1200w.png">
///   <img src="https://static.rerun.io/point2d_simple/a8e801958bce5aa4e080659c033630f86ce95f71/full.png" width="640">
/// </picture>
/// </center>
///
/// ### `point2d_random`:
/// ```ignore
/// //! Log some random points with color and radii.
///
/// use rand::{distributions::Uniform, Rng as _};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) = rerun::RecordingStreamBuilder::new("rerun_example_points2d").memory()?;
///
///     let mut rng = rand::thread_rng();
///     let dist = Uniform::new(-3., 3.);
///
///     rec.log(
///         "random",
///         &rerun::Points2D::new((0..10).map(|_| (rng.sample(dist), rng.sample(dist))))
///             .with_colors((0..10).map(|_| rerun::Color::from_rgb(rng.gen(), rng.gen(), rng.gen())))
///             .with_radii((0..10).map(|_| rng.gen::<f32>())),
///     )?;
///
///     // Log an extra rect to set the view bounds
///     rec.log("bounds", &rerun::Boxes2D::from_half_sizes([(4., 3.)]))?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/1200w.png">
///   <img src="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Points2D {
    /// All the 2D positions at which the point cloud shows points.
    pub positions: Vec<crate::components::Position2D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the points.
    pub labels: Option<Vec<crate::components::Text>>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    pub keypoint_ids: Option<Vec<crate::components::KeypointId>>,

    /// Unique identifiers for each individual point in the batch.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Position2D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Color".into(),
            "rerun.components.Points2DIndicator".into(),
            "rerun.components.Radius".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.ClassId".into(),
            "rerun.components.DrawOrder".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.KeypointId".into(),
            "rerun.components.Text".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Position2D".into(),
            "rerun.components.Color".into(),
            "rerun.components.Points2DIndicator".into(),
            "rerun.components.Radius".into(),
            "rerun.components.ClassId".into(),
            "rerun.components.DrawOrder".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.KeypointId".into(),
            "rerun.components.Text".into(),
        ]
    });

impl Points2D {
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Points2D`] [`crate::Archetype`]
pub type Points2DIndicator = crate::GenericIndicatorComponent<Points2D>;

impl crate::Archetype for Points2D {
    type Indicator = Points2DIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.Points2D".into()
    }

    #[inline]
    fn indicator() -> crate::MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Points2DIndicator = Points2DIndicator::DEFAULT;
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
        let positions = {
            let array = arrays_by_name
                .get("rerun.components.Position2D")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.Points2D#positions")?;
            <crate::components::Position2D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points2D#positions")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Points2D#positions")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#labels")?
            })
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            Some({
                <crate::components::DrawOrder>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#draw_order")?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(crate::DeserializationError::missing_data)
                    .with_context("rerun.archetypes.Points2D#draw_order")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#class_ids")?
            })
        } else {
            None
        };
        let keypoint_ids = if let Some(array) = arrays_by_name.get("rerun.components.KeypointId") {
            Some({
                <crate::components::KeypointId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
            })
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("rerun.components.InstanceKey")
        {
            Some({
                <crate::components::InstanceKey>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#instance_keys")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#instance_keys")?
            })
        } else {
            None
        };
        Ok(Self {
            positions,
            radii,
            colors,
            labels,
            draw_order,
            class_ids,
            keypoint_ids,
            instance_keys,
        })
    }
}

impl crate::AsComponents for Points2D {
    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.positions as &dyn crate::ComponentBatch).into()),
            self.radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.draw_order
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.keypoint_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.instance_keys
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.positions.len()
    }
}

impl Points2D {
    pub fn new(
        positions: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        Self {
            positions: positions.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            draw_order: None,
            class_ids: None,
            keypoint_ids: None,
            instance_keys: None,
        }
    }

    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }

    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_keypoint_ids(
        mut self,
        keypoint_ids: impl IntoIterator<Item = impl Into<crate::components::KeypointId>>,
    ) -> Self {
        self.keypoint_ids = Some(keypoint_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
