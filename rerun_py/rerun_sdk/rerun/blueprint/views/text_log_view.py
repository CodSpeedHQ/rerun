# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/text_log.fbs".

from __future__ import annotations

__all__ = ["TextLogView"]


from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class TextLogView(SpaceView):
    """**View**: A text log view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: blueprint_components.VisibleLike | None = None,
    ) -> None:
        """
        Construct a blueprint for a new TextLogView view.

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

        """

        properties: dict[str, AsComponents] = {}
        super().__init__(
            class_identifier="TextLog",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
        )
