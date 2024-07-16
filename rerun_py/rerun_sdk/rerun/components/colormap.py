# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/colormap.fbs".

# You can extend this class by creating a "ColormapExt" class in "colormap_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = ["Colormap", "ColormapArrayLike", "ColormapBatch", "ColormapLike", "ColormapType"]


from enum import Enum


class Colormap(Enum):
    """
    **Component**: Colormap for mapping scalar values within a given range to a color.

    This provides a number of popular pre-defined colormaps.
    In the future, the Rerun Viewer will allow users to define their own colormaps,
    but currently the Viewer is limited to the types defined here.
    """

    Grayscale = 1
    """
    A simple black to white gradient.

    This is a sRGB gray gradient which is perceptually uniform.
    """

    Inferno = 2
    """
    The Inferno colormap from Matplotlib.

    This is a perceptually uniform colormap.
    It interpolates from black to red to bright yellow.
    """

    Magma = 3
    """
    The Magma colormap from Matplotlib.

    This is a perceptually uniform colormap.
    It interpolates from black to purple to white.
    """

    Plasma = 4
    """
    The Plasma colormap from Matplotlib.

    This is a perceptually uniform colormap.
    It interpolates from dark blue to purple to yellow.
    """

    Turbo = 5
    """
    Google's Turbo colormap map.

    This is a perceptually non-uniform rainbow colormap addressing many issues of
    more traditional rainbow colormaps like Jet.
    It is more perceptually uniform without sharp transitions and is more colorblind-friendly.
    Details: <https://research.google/blog/turbo-an-improved-rainbow-colormap-for-visualization/>
    """

    Viridis = 6
    """
    The Viridis colormap from Matplotlib

    This is a perceptually uniform colormap which is robust to color blindness.
    It interpolates from dark purple to green to yellow.
    """


ColormapLike = Union[Colormap, Literal["grayscale", "inferno", "magma", "plasma", "turbo", "viridis"]]
ColormapArrayLike = Union[ColormapLike, Sequence[ColormapLike]]


class ColormapType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.Colormap"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("Grayscale", pa.null(), nullable=True, metadata={}),
                pa.field("Inferno", pa.null(), nullable=True, metadata={}),
                pa.field("Magma", pa.null(), nullable=True, metadata={}),
                pa.field("Plasma", pa.null(), nullable=True, metadata={}),
                pa.field("Turbo", pa.null(), nullable=True, metadata={}),
                pa.field("Viridis", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class ColormapBatch(BaseBatch[ColormapArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = ColormapType()

    @staticmethod
    def _native_to_pa_array(data: ColormapArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (Colormap, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, Colormap):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(Colormap, value):
                    types.append(Colormap[value].value)  # fast path
                elif value.lower() == "grayscale":
                    types.append(Colormap.Grayscale.value)
                elif value.lower() == "inferno":
                    types.append(Colormap.Inferno.value)
                elif value.lower() == "magma":
                    types.append(Colormap.Magma.value)
                elif value.lower() == "plasma":
                    types.append(Colormap.Plasma.value)
                elif value.lower() == "turbo":
                    types.append(Colormap.Turbo.value)
                elif value.lower() == "viridis":
                    types.append(Colormap.Viridis.value)
                else:
                    raise ValueError(f"Unknown Colormap kind: {value}")
            else:
                raise ValueError(f"Unknown Colormap kind: {value}")

        buffers = [
            None,
            pa.array(types, type=pa.int8()).buffers()[1],
        ]
        children = (1 + 6) * [pa.nulls(len(data))]

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=buffers,
            children=children,
        )
