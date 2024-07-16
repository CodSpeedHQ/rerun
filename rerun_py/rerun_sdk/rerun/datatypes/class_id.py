# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/class_id.fbs".

# You can extend this class by creating a "ClassIdExt" class in "class_id_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
)

__all__ = ["ClassId", "ClassIdArrayLike", "ClassIdBatch", "ClassIdLike", "ClassIdType"]


@define(init=False)
class ClassId:
    """**Datatype**: A 16-bit ID representing a type of semantic class."""

    def __init__(self: Any, id: ClassIdLike):
        """Create a new instance of the ClassId datatype."""

        # You can define your own __init__ function as a member of ClassIdExt in class_id_ext.py
        self.__attrs_init__(id=id)

    id: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of ClassIdExt in class_id_ext.py
        return np.asarray(self.id, dtype=dtype)

    def __int__(self) -> int:
        return int(self.id)

    def __hash__(self) -> int:
        return hash(self.id)


if TYPE_CHECKING:
    ClassIdLike = Union[ClassId, int]
else:
    ClassIdLike = Any

ClassIdArrayLike = Union[ClassId, Sequence[ClassIdLike], int, npt.ArrayLike]


class ClassIdType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.ClassId"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.uint16(), self._TYPE_NAME)


class ClassIdBatch(BaseBatch[ClassIdArrayLike]):
    _ARROW_TYPE = ClassIdType()

    @staticmethod
    def _native_to_pa_array(data: ClassIdArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.uint16).flatten()
        return pa.array(array, type=data_type)
