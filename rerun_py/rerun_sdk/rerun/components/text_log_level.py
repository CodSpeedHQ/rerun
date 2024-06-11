# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/text_log_level.fbs".

# You can extend this class by creating a "TextLogLevelExt" class in "text_log_level_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)
from .text_log_level_ext import TextLogLevelExt

__all__ = ["TextLogLevel", "TextLogLevelBatch", "TextLogLevelType"]


class TextLogLevel(TextLogLevelExt, datatypes.Utf8, ComponentMixin):
    """
    **Component**: The severity level of a text log message.

    Recommended to be one of:
    * `"CRITICAL"`
    * `"ERROR"`
    * `"WARN"`
    * `"INFO"`
    * `"DEBUG"`
    * `"TRACE"`
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of TextLogLevelExt in text_log_level_ext.py

    # Note: there are no fields here because TextLogLevel delegates to datatypes.Utf8
    pass


class TextLogLevelType(datatypes.Utf8Type):
    _TYPE_NAME: str = "rerun.components.TextLogLevel"


class TextLogLevelBatch(datatypes.Utf8Batch, ComponentBatchMixin):
    _ARROW_TYPE = TextLogLevelType()


# This is patched in late to avoid circular dependencies.
TextLogLevel._BATCH_TYPE = TextLogLevelBatch  # type: ignore[assignment]


TextLogLevelExt.deferred_patch_class(TextLogLevel)
