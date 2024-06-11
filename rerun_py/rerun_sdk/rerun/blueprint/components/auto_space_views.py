# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/auto_space_views.fbs".

# You can extend this class by creating a "AutoSpaceViewsExt" class in "auto_space_views_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import pyarrow as pa
from attrs import define, field

from ..._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = [
    "AutoSpaceViews",
    "AutoSpaceViewsArrayLike",
    "AutoSpaceViewsBatch",
    "AutoSpaceViewsLike",
    "AutoSpaceViewsType",
]


@define(init=False)
class AutoSpaceViews(ComponentMixin):
    """**Component**: Whether or not space views should be created automatically."""

    _BATCH_TYPE = None

    def __init__(self: Any, auto_space_views: AutoSpaceViewsLike):
        """Create a new instance of the AutoSpaceViews component."""

        # You can define your own __init__ function as a member of AutoSpaceViewsExt in auto_space_views_ext.py
        self.__attrs_init__(auto_space_views=auto_space_views)

    def __bool__(self) -> bool:
        return self.auto_space_views

    auto_space_views: bool = field(converter=bool)


if TYPE_CHECKING:
    AutoSpaceViewsLike = Union[AutoSpaceViews, bool]
else:
    AutoSpaceViewsLike = Any

AutoSpaceViewsArrayLike = Union[
    AutoSpaceViews,
    Sequence[AutoSpaceViewsLike],
]


class AutoSpaceViewsType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.AutoSpaceViews"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.bool_(), self._TYPE_NAME)


class AutoSpaceViewsBatch(BaseBatch[AutoSpaceViewsArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = AutoSpaceViewsType()

    @staticmethod
    def _native_to_pa_array(data: AutoSpaceViewsArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.bool_).flatten()
        return pa.array(array, type=data_type)
