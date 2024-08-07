# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/auto_layout.fbs".

# You can extend this class by creating a "AutoLayoutExt" class in "auto_layout_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["AutoLayout", "AutoLayoutBatch", "AutoLayoutType"]


class AutoLayout(datatypes.Bool, ComponentMixin):
    """**Component**: Whether the viewport layout is determined automatically."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of AutoLayoutExt in auto_layout_ext.py

    # Note: there are no fields here because AutoLayout delegates to datatypes.Bool
    pass


class AutoLayoutType(datatypes.BoolType):
    _TYPE_NAME: str = "rerun.blueprint.components.AutoLayout"


class AutoLayoutBatch(datatypes.BoolBatch, ComponentBatchMixin):
    _ARROW_TYPE = AutoLayoutType()


# This is patched in late to avoid circular dependencies.
AutoLayout._BATCH_TYPE = AutoLayoutBatch  # type: ignore[assignment]
