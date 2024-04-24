# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/space_views/spatial3d.fbs".

from __future__ import annotations

__all__ = ["Spatial3DView"]


from ... import datatypes
from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class Spatial3DView(SpaceView):
    """**Space view**: A Spatial 3D view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        background: blueprint_archetypes.Background3D
        | datatypes.Rgba32Like
        | blueprint_components.Background3DKindLike
        | None = None,
    ):
        """
        Construct a blueprint for a new Spatial3DView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the space view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.SpaceViewContents][].
        name:
            The display name of the view.
        background:
            Configuration for the background of the 3D space view.

        """

        properties: dict[str, AsComponents] = {}
        if background is not None:
            if not isinstance(background, blueprint_archetypes.Background3D):
                background = blueprint_archetypes.Background3D(background)
            properties["Background3D"] = background

        super().__init__(class_identifier="3D", origin=origin, contents=contents, name=name, properties=properties)
