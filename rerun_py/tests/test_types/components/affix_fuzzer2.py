# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer2Ext" class in "affix_fuzzer2_ext.py".

from __future__ import annotations

from rerun._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

from .. import datatypes

__all__ = ["AffixFuzzer2", "AffixFuzzer2Batch", "AffixFuzzer2Type"]


class AffixFuzzer2(datatypes.AffixFuzzer1, ComponentMixin):
    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of AffixFuzzer2Ext in affix_fuzzer2_ext.py

    # Note: there are no fields here because AffixFuzzer2 delegates to datatypes.AffixFuzzer1
    pass


class AffixFuzzer2Type(datatypes.AffixFuzzer1Type):
    _TYPE_NAME: str = "rerun.testing.components.AffixFuzzer2"


class AffixFuzzer2Batch(datatypes.AffixFuzzer1Batch, ComponentBatchMixin):
    _ARROW_TYPE = AffixFuzzer2Type()


# This is patched in late to avoid circular dependencies.
AffixFuzzer2._BATCH_TYPE = AffixFuzzer2Batch  # type: ignore[assignment]
