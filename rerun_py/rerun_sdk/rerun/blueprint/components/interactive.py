# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/interactive.fbs".

# You can extend this class by creating a "InteractiveExt" class in "interactive_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import ComponentBatchMixin

__all__ = ["Interactive", "InteractiveBatch", "InteractiveType"]


class Interactive(datatypes.Bool):
    """
    **Component**: Whether the entity can be interacted with.

    Non interactive components are still visible, but mouse iteractions in the view are disabled.
    """

    # You can define your own __init__ function as a member of InteractiveExt in interactive_ext.py

    # Note: there are no fields here because Interactive delegates to datatypes.Bool
    pass


class InteractiveType(datatypes.BoolType):
    _TYPE_NAME: str = "rerun.blueprint.components.Interactive"


class InteractiveBatch(datatypes.BoolBatch, ComponentBatchMixin):
    _ARROW_TYPE = InteractiveType()
