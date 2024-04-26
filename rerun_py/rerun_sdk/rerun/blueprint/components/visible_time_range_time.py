# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/visible_time_range_time.fbs".

# You can extend this class by creating a "VisibleTimeRangeTimeExt" class in "visible_time_range_time_ext.py".

from __future__ import annotations

from ..._baseclasses import ComponentBatchMixin
from ...blueprint import datatypes as blueprint_datatypes

__all__ = ["VisibleTimeRangeTime", "VisibleTimeRangeTimeBatch", "VisibleTimeRangeTimeType"]


class VisibleTimeRangeTime(blueprint_datatypes.VisibleTimeRange):
    """**Component**: The range of values on sequence timelines that will be included in a space view query."""

    # You can define your own __init__ function as a member of VisibleTimeRangeTimeExt in visible_time_range_time_ext.py

    # Note: there are no fields here because VisibleTimeRangeTime delegates to datatypes.VisibleTimeRange
    pass


class VisibleTimeRangeTimeType(blueprint_datatypes.VisibleTimeRangeType):
    _TYPE_NAME: str = "rerun.blueprint.components.VisibleTimeRangeTime"


class VisibleTimeRangeTimeBatch(blueprint_datatypes.VisibleTimeRangeBatch, ComponentBatchMixin):
    _ARROW_TYPE = VisibleTimeRangeTimeType()
