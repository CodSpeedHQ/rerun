# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/spatial2d.fbs".

from __future__ import annotations

from typing import Sequence, Union

__all__ = ["Spatial2DView"]


from ... import datatypes
from ..._baseclasses import AsComponents, ComponentBatchLike
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes, components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class Spatial2DView(SpaceView):
    """
    **View**: For viewing spatial 2D data.

    Example
    -------
    ### Use a blueprint to customize a Spatial2DView.:
    ```python
    import numpy as np
    import rerun as rr
    import rerun.blueprint as rrb

    rr.init("rerun_example_spatial_2d", spawn=True)

    # Create a spiral of points:
    n = 150
    angle = np.linspace(0, 10 * np.pi, n)
    spiral_radius = np.linspace(0.0, 3.0, n) ** 2
    positions = np.column_stack((np.cos(angle) * spiral_radius, np.sin(angle) * spiral_radius))
    colors = np.dstack((np.linspace(255, 255, n), np.linspace(255, 0, n), np.linspace(0, 255, n)))[0].astype(int)
    radii = np.linspace(0.01, 0.7, n)

    rr.log("points", rr.Points2D(positions, colors=colors, radii=radii))

    # Create a Spatial2D view to display the points.
    blueprint = rrb.Blueprint(
        rrb.Spatial2DView(
            origin="/",
            name="2D Scene",
            # Set the background color
            background=[105, 20, 105],
            # Note that this range is smaller than the range of the points,
            # so some points will not be visible.
            visual_bounds=rrb.VisualBounds2D(x_range=[-5, 5], y_range=[-5, 5]),
        ),
        collapse_panels=True,
    )

    rr.send_blueprint(blueprint)
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/Spatial2DVIew/824a075e0c50ea4110eb6ddd60257f087cb2264d/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/Spatial2DVIew/824a075e0c50ea4110eb6ddd60257f087cb2264d/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/Spatial2DVIew/824a075e0c50ea4110eb6ddd60257f087cb2264d/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/Spatial2DVIew/824a075e0c50ea4110eb6ddd60257f087cb2264d/1200w.png">
      <img src="https://static.rerun.io/Spatial2DVIew/824a075e0c50ea4110eb6ddd60257f087cb2264d/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: blueprint_components.VisibleLike | None = None,
        defaults: list[Union[AsComponents, ComponentBatchLike]] = [],
        overrides: dict[EntityPathLike, list[ComponentBatchLike]] = {},
        background: blueprint_archetypes.Background
        | datatypes.Rgba32Like
        | blueprint_components.BackgroundKindLike
        | None = None,
        visual_bounds: blueprint_archetypes.VisualBounds2D | None = None,
        time_ranges: blueprint_archetypes.VisibleTimeRanges
        | datatypes.VisibleTimeRangeLike
        | Sequence[datatypes.VisibleTimeRangeLike]
        | None = None,
    ) -> None:
        """
        Construct a blueprint for a new Spatial2DView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.SpaceViewContents][].
        name:
            The display name of the view.
        visible:
            Whether this view is visible.

            Defaults to true if not specified.
        defaults:
            List of default components or component batches to add to the space view. When an archetype
            in the view is missing a component included in this set, the value of default will be used
            instead of the normal fallback for the visualizer.
        overrides:
            Dictionary of overrides to apply to the space view. The key is the path to the entity where the override
            should be applied. The value is a list of component or component batches to apply to the entity.

            Important note: the path must be a fully qualified entity path starting at the root. The override paths
            do not yet support `$origin` relative paths or glob expressions.
            This will be addressed in: [https://github.com/rerun-io/rerun/issues/6673][].
        background:
            Configuration for the background of the view.
        visual_bounds:
            The visible parts of the scene, in the coordinate space of the scene.

            Everything within these bounds are guaranteed to be visible.
            Somethings outside of these bounds may also be visible due to letterboxing.
        time_ranges:
            Configures which range on each timeline is shown by this view (unless specified differently per entity).

            If not specified, the default is to show the latest state of each component.
            If a timeline is specified more than once, the first entry will be used.

        """

        properties: dict[str, AsComponents] = {}
        if background is not None:
            if not isinstance(background, blueprint_archetypes.Background):
                background = blueprint_archetypes.Background(background)
            properties["Background"] = background

        if visual_bounds is not None:
            if not isinstance(visual_bounds, blueprint_archetypes.VisualBounds2D):
                visual_bounds = blueprint_archetypes.VisualBounds2D(visual_bounds)
            properties["VisualBounds2D"] = visual_bounds

        if time_ranges is not None:
            if not isinstance(time_ranges, blueprint_archetypes.VisibleTimeRanges):
                time_ranges = blueprint_archetypes.VisibleTimeRanges(time_ranges)
            properties["VisibleTimeRanges"] = time_ranges

        super().__init__(
            class_identifier="2D",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
            defaults=defaults,
            overrides=overrides,
        )
