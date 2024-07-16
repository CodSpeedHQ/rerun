# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/class_id.fbs".

# You can extend this class by creating a "ClassIdExt" class in "class_id_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["ClassId", "ClassIdBatch", "ClassIdType"]


class ClassId(datatypes.ClassId, ComponentMixin):
    """**Component**: A 16-bit ID representing a type of semantic class."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of ClassIdExt in class_id_ext.py

    # Note: there are no fields here because ClassId delegates to datatypes.ClassId
    pass


class ClassIdType(datatypes.ClassIdType):
    _TYPE_NAME: str = "rerun.components.ClassId"


class ClassIdBatch(datatypes.ClassIdBatch, ComponentBatchMixin):
    _ARROW_TYPE = ClassIdType()


# This is patched in late to avoid circular dependencies.
ClassId._BATCH_TYPE = ClassIdBatch  # type: ignore[assignment]
