# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/archetypes/map_options.fbs".

# You can extend this class by creating a "MapOptionsExt" class in "map_options_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import datatypes
from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["MapOptions"]


@define(str=False, repr=False, init=False)
class MapOptions(Archetype):
    """**Archetype**: Configuration for the background of a view."""

    def __init__(
        self: Any,
        provider: blueprint_components.MapProviderLike,
        zoom: blueprint_components.ZoomLevelLike,
        access_token: datatypes.Utf8Like,
    ):
        """
        Create a new instance of the MapOptions archetype.

        Parameters
        ----------
        provider:
            Map provider and style to use.
        zoom:
            Zoom level for the map. The default is 16.
        access_token:
            Optional access token to access the map tiles.

        """

        # You can define your own __init__ function as a member of MapOptionsExt in map_options_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(provider=provider, zoom=zoom, access_token=access_token)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            provider=None,  # type: ignore[arg-type]
            zoom=None,  # type: ignore[arg-type]
            access_token=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> MapOptions:
        """Produce an empty MapOptions, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    provider: blueprint_components.MapProviderBatch = field(
        metadata={"component": "required"},
        converter=blueprint_components.MapProviderBatch._required,  # type: ignore[misc]
    )
    # Map provider and style to use.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    zoom: blueprint_components.ZoomLevelBatch = field(
        metadata={"component": "required"},
        converter=blueprint_components.ZoomLevelBatch._required,  # type: ignore[misc]
    )
    # Zoom level for the map. The default is 16.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    access_token: blueprint_components.SecretBatch = field(
        metadata={"component": "required"},
        converter=blueprint_components.SecretBatch._required,  # type: ignore[misc]
    )
    # Optional access token to access the map tiles.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
