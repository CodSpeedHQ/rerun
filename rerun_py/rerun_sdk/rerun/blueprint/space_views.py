from __future__ import annotations

from .._baseclasses import AsComponents
from ..datatypes import EntityPathLike, Rgba32Like, Utf8Like
from . import archetypes as blueprint_archetypes
from . import components as blueprint_components
from .api import SpaceView, SpaceViewContentsLike


class BarChartView(SpaceView):
    """A bar chart view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new bar chart view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="BarChart", origin=origin, contents=contents, name=name)


class Spatial2DView(SpaceView):
    """A Spatial 2D view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new spatial 2D view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="2D", origin=origin, contents=contents, name=name)


class Spatial3DView(SpaceView):
    """A Spatial 3D view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        # TODO(andreas): codegen everything that comes below:
        background: blueprint_components.Background3DKindLike
        | Rgba32Like
        | blueprint_archetypes.Background3D
        | None = None,
    ):
        """
        Construct a blueprint for a new spatial 3D view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.
        background:
            Configuration for the background of the 3D space view.

        """
        properties: dict[str, AsComponents] = {}
        # TODO(andreas): codegen creation of the properties dict
        if background is not None:
            properties["Background3D"] = (
                background
                if isinstance(background, blueprint_archetypes.Background3D)
                else blueprint_archetypes.Background3D(background)
            )

        super().__init__(class_identifier="3D", origin=origin, contents=contents, name=name, properties=properties)


class TensorView(SpaceView):
    """A tensor view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new tensor view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="Tensor", origin=origin, contents=contents, name=name)


class TextDocumentView(SpaceView):
    """A text document view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new text document view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="TextDocument", origin=origin, contents=contents, name=name)


class TextLogView(SpaceView):
    """A text log view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new text log view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="TextLog", origin=origin, contents=contents, name=name)


class TimeSeriesView(SpaceView):
    """A time series view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
    ):
        """
        Construct a blueprint for a new time series view.

        Parameters
        ----------
        origin
            The `EntityPath` to use as the origin of this view. All other entities will be transformed
            to be displayed relative to this origin.
        contents
            The contents of the space view specified as a query expression. This is either a single expression,
            or a list of multiple expressions. See [rerun.blueprint.archetypes.SpaceViewContents][].
        name
            The name of the view.

        """
        super().__init__(class_identifier="TimeSeries", origin=origin, contents=contents, name=name)
