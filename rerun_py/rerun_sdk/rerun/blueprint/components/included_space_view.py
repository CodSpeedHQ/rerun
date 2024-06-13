# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/included_space_view.fbs".

# You can extend this class by creating a "IncludedSpaceViewExt" class in "included_space_view_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["IncludedSpaceView", "IncludedSpaceViewBatch", "IncludedSpaceViewType"]


class IncludedSpaceView(datatypes.Uuid, ComponentMixin):
    """**Component**: The unique id of a space view, used to refer to views in containers."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of IncludedSpaceViewExt in included_space_view_ext.py

    # Note: there are no fields here because IncludedSpaceView delegates to datatypes.Uuid
    pass


class IncludedSpaceViewType(datatypes.UuidType):
    _TYPE_NAME: str = "rerun.blueprint.components.IncludedSpaceView"


class IncludedSpaceViewBatch(datatypes.UuidBatch, ComponentBatchMixin):
    _ARROW_TYPE = IncludedSpaceViewType()


# This is patched in late to avoid circular dependencies.
IncludedSpaceView._BATCH_TYPE = IncludedSpaceViewBatch  # type: ignore[assignment]
