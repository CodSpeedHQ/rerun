# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/material.fbs".

# You can extend this class by creating a "MaterialExt" class in "material_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Material", "MaterialBatch", "MaterialType"]


class Material(datatypes.Material, ComponentMixin):
    """**Component**: Material properties of a mesh."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of MaterialExt in material_ext.py

    # Note: there are no fields here because Material delegates to datatypes.Material
    pass


class MaterialType(datatypes.MaterialType):
    _TYPE_NAME: str = "rerun.components.Material"


class MaterialBatch(datatypes.MaterialBatch, ComponentBatchMixin):
    _ARROW_TYPE = MaterialType()


# This is patched in late to avoid circular dependencies.
Material._BATCH_TYPE = MaterialBatch  # type: ignore[assignment]
