# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/rotation_axis_angle.fbs".

# You can extend this class by creating a "LeafRotationAxisAngleExt" class in "leaf_rotation_axis_angle_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["LeafRotationAxisAngle", "LeafRotationAxisAngleBatch", "LeafRotationAxisAngleType"]


class LeafRotationAxisAngle(datatypes.RotationAxisAngle, ComponentMixin):
    """**Component**: 3D rotation represented by a rotation around a given axis that doesn't propagate in the transform hierarchy."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of LeafRotationAxisAngleExt in leaf_rotation_axis_angle_ext.py

    # Note: there are no fields here because LeafRotationAxisAngle delegates to datatypes.RotationAxisAngle
    pass


class LeafRotationAxisAngleType(datatypes.RotationAxisAngleType):
    _TYPE_NAME: str = "rerun.components.LeafRotationAxisAngle"


class LeafRotationAxisAngleBatch(datatypes.RotationAxisAngleBatch, ComponentBatchMixin):
    _ARROW_TYPE = LeafRotationAxisAngleType()


# This is patched in late to avoid circular dependencies.
LeafRotationAxisAngle._BATCH_TYPE = LeafRotationAxisAngleBatch  # type: ignore[assignment]
