# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer8Ext" class in "affix_fuzzer8_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field
from rerun._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
    ComponentMixin,
)
from rerun._converters import (
    float_or_none,
)

__all__ = ["AffixFuzzer8", "AffixFuzzer8ArrayLike", "AffixFuzzer8Batch", "AffixFuzzer8Like", "AffixFuzzer8Type"]


@define(init=False)
class AffixFuzzer8(ComponentMixin):
    _BATCH_TYPE = None

    def __init__(self: Any, single_float_optional: float | None = None):
        """Create a new instance of the AffixFuzzer8 component."""

        # You can define your own __init__ function as a member of AffixFuzzer8Ext in affix_fuzzer8_ext.py
        self.__attrs_init__(single_float_optional=single_float_optional)

    single_float_optional: float | None = field(default=None, converter=float_or_none)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of AffixFuzzer8Ext in affix_fuzzer8_ext.py
        return np.asarray(self.single_float_optional, dtype=dtype)


AffixFuzzer8Like = AffixFuzzer8
AffixFuzzer8ArrayLike = Union[
    AffixFuzzer8,
    Sequence[AffixFuzzer8Like],
]


class AffixFuzzer8Type(BaseExtensionType):
    _TYPE_NAME: str = "rerun.testing.components.AffixFuzzer8"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class AffixFuzzer8Batch(BaseBatch[AffixFuzzer8ArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = AffixFuzzer8Type()

    @staticmethod
    def _native_to_pa_array(data: AffixFuzzer8ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError(
            "Arrow serialization of AffixFuzzer8 not implemented: We lack codegen for arrow-serialization of general structs"
        )  # You need to implement native_to_pa_array_override in affix_fuzzer8_ext.py
