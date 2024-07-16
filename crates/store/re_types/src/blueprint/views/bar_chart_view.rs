// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/views/bar_chart.fbs".

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

/// **View**: A bar chart view.
#[derive(Clone, Debug)]
pub struct BarChartView {
    /// Configures the legend of the plot.
    pub plot_legend: crate::blueprint::archetypes::PlotLegend,
}

impl ::re_types_core::SizeBytes for BarChartView {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.plot_legend.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::archetypes::PlotLegend>::is_pod()
    }
}

impl<T: Into<crate::blueprint::archetypes::PlotLegend>> From<T> for BarChartView {
    fn from(v: T) -> Self {
        Self {
            plot_legend: v.into(),
        }
    }
}

impl std::borrow::Borrow<crate::blueprint::archetypes::PlotLegend> for BarChartView {
    #[inline]
    fn borrow(&self) -> &crate::blueprint::archetypes::PlotLegend {
        &self.plot_legend
    }
}

impl std::ops::Deref for BarChartView {
    type Target = crate::blueprint::archetypes::PlotLegend;

    #[inline]
    fn deref(&self) -> &crate::blueprint::archetypes::PlotLegend {
        &self.plot_legend
    }
}

impl std::ops::DerefMut for BarChartView {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::blueprint::archetypes::PlotLegend {
        &mut self.plot_legend
    }
}

impl ::re_types_core::View for BarChartView {
    #[inline]
    fn identifier() -> ::re_types_core::SpaceViewClassIdentifier {
        "BarChart".into()
    }
}
