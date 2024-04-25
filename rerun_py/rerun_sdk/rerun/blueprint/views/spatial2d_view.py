# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/spatial2d.fbs".

from __future__ import annotations

__all__ = ["Spatial2DView"]


from ... import datatypes
from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class Spatial2DView(SpaceView):
    """**View**: A Spatial 2D view."""

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
        background:
            Configuration for the background of the space view.

        """

        properties: dict[str, AsComponents] = {}
        if background is not None:
            if not isinstance(background, blueprint_archetypes.Background):
                background = blueprint_archetypes.Background(background)
            properties["Background"] = background

        super().__init__(
            class_identifier="2D", origin=origin, contents=contents, name=name, visible=visible, properties=properties
        )
