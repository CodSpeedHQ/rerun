# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/archetypes/visual_bounds2d.fbs".

# You can extend this class by creating a "VisualBounds2DExt" class in "visual_bounds2d_ext.py".

from __future__ import annotations

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from .visual_bounds2d_ext import VisualBounds2DExt

__all__ = ["VisualBounds2D"]


@define(str=False, repr=False, init=False)
class VisualBounds2D(VisualBounds2DExt, Archetype):
    """
    **Archetype**: Controls the visual bounds of a 2D view.

    Everything within these bounds are guaranteed to be visible.
    Somethings outside of these bounds may also be visible due to letterboxing.

    If no visual bounds are set, it will be determined automatically,
    based on the bounding-box of the data or other camera information present in the view.
    """

    # __init__ can be found in visual_bounds2d_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            range=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> VisualBounds2D:
        """Produce an empty VisualBounds2D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    range: blueprint_components.VisualBounds2DBatch = field(
        metadata={"component": "required"},
        converter=blueprint_components.VisualBounds2DBatch._required,  # type: ignore[misc]
    )
    # Controls the visible range of a 2D view.
    #
    # Use this to control pan & zoom of the view.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
