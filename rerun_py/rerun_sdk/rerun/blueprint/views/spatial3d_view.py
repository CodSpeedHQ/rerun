# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/spatial3d.fbs".

from __future__ import annotations

from typing import Sequence

__all__ = ["Spatial3DView"]


from ... import datatypes
from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class Spatial3DView(SpaceView):
    """
    **View**: A Spatial 3D view.

    Example
    -------
    ### Use a blueprint to customize a Spatial3DView:
    ```python
    import rerun as rr
    import rerun.blueprint as rrb
    from numpy.random import default_rng

    rr.init("rerun_example_spatial_3d", spawn=True)

    # Create some random points
    rng = default_rng(12345)
    positions = rng.uniform(-5, 5, size=[10, 3])
    colors = rng.uniform(0, 255, size=[10, 3])
    radii = rng.uniform(0, 1, size=[10])

    rr.log("points", rr.Points3D(positions, colors=colors, radii=radii))

    # Create a Spatial3D View
    blueprint = rrb.Blueprint(
        rrb.Spatial3DView(
            origin="/points",
            background=[80, 80, 80],
        )
    )

    rr.send_blueprint(blueprint)
    ```

    """

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: blueprint_components.VisibleLike | None = None,
        background: blueprint_archetypes.Background
        | datatypes.Rgba32Like
        | blueprint_components.BackgroundKindLike
        | None = None,
        time_ranges: blueprint_archetypes.VisibleTimeRanges
        | blueprint_components.VisibleTimeRangeLike
        | Sequence[blueprint_components.VisibleTimeRangeLike]
        | None = None,
    ) -> None:
        """
        Construct a blueprint for a new Spatial3DView view.

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
        background:
            Configuration for the background of the space view.
        time_ranges:
            Configures which range on each timeline is shown by this view (unless specified differently per entity).

        """

        properties: dict[str, AsComponents] = {}
        if background is not None:
            if not isinstance(background, blueprint_archetypes.Background):
                background = blueprint_archetypes.Background(background)
            properties["Background"] = background

        if time_ranges is not None:
            if not isinstance(time_ranges, blueprint_archetypes.VisibleTimeRanges):
                time_ranges = blueprint_archetypes.VisibleTimeRanges(time_ranges)
            properties["VisibleTimeRanges"] = time_ranges

        super().__init__(
            class_identifier="3D", origin=origin, contents=contents, name=name, visible=visible, properties=properties
        )
