# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/datatypes/translation_rotation_scale3d.fbs".

# You can extend this class by creating a "TranslationRotationScale3DExt" class in "translation_rotation_scale3d_ext.py".

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
)
from .translation_rotation_scale3d_ext import TranslationRotationScale3DExt

__all__ = [
    "TranslationRotationScale3D",
    "TranslationRotationScale3DArrayLike",
    "TranslationRotationScale3DBatch",
    "TranslationRotationScale3DLike",
    "TranslationRotationScale3DType",
]


def _translation_rotation_scale3d__rotation__special_field_converter_override(
    x: datatypes.Rotation3DLike | None,
) -> datatypes.Rotation3D | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Rotation3D):
        return x
    else:
        return datatypes.Rotation3D(x)


def _translation_rotation_scale3d__scale__special_field_converter_override(
    x: datatypes.Scale3DLike | None,
) -> datatypes.Scale3D | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Scale3D):
        return x
    else:
        return datatypes.Scale3D(x)


@define(init=False)
class TranslationRotationScale3D(TranslationRotationScale3DExt):
    """**Datatype**: Representation of an affine transform via separate translation, rotation & scale."""

    # __init__ can be found in translation_rotation_scale3d_ext.py

    from_parent: bool = field(converter=bool)
    # If true, this transform is from the parent space to the space where the transform was logged.
    #
    # If false (default), the transform maps from this space to its parent,
    # i.e. the translation is the position in the parent space.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    translation: datatypes.Vec3D | None = field(
        default=None,
        converter=TranslationRotationScale3DExt.translation__field_converter_override,  # type: ignore[misc]
    )
    # 3D translation vector, applied last.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    rotation: datatypes.Rotation3D | None = field(
        default=None, converter=_translation_rotation_scale3d__rotation__special_field_converter_override
    )
    # 3D rotation, applied second.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    scale: datatypes.Scale3D | None = field(
        default=None, converter=_translation_rotation_scale3d__scale__special_field_converter_override
    )
    # 3D scale, applied first.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


TranslationRotationScale3DLike = TranslationRotationScale3D
TranslationRotationScale3DArrayLike = Union[
    TranslationRotationScale3D,
    Sequence[TranslationRotationScale3DLike],
]


class TranslationRotationScale3DType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.TranslationRotationScale3D"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct([
                pa.field(
                    "translation",
                    pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                    nullable=True,
                    metadata={},
                ),
                pa.field(
                    "rotation",
                    pa.dense_union([
                        pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                        pa.field(
                            "Quaternion",
                            pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 4),
                            nullable=False,
                            metadata={},
                        ),
                        pa.field(
                            "AxisAngle",
                            pa.struct([
                                pa.field(
                                    "axis",
                                    pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                                    nullable=False,
                                    metadata={},
                                ),
                                pa.field(
                                    "angle",
                                    pa.dense_union([
                                        pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                                        pa.field("Radians", pa.float32(), nullable=False, metadata={}),
                                        pa.field("Degrees", pa.float32(), nullable=False, metadata={}),
                                    ]),
                                    nullable=False,
                                    metadata={},
                                ),
                            ]),
                            nullable=False,
                            metadata={},
                        ),
                    ]),
                    nullable=True,
                    metadata={},
                ),
                pa.field(
                    "scale",
                    pa.dense_union([
                        pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                        pa.field(
                            "ThreeD",
                            pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                            nullable=False,
                            metadata={},
                        ),
                        pa.field("Uniform", pa.float32(), nullable=False, metadata={}),
                    ]),
                    nullable=True,
                    metadata={},
                ),
                pa.field("from_parent", pa.bool_(), nullable=False, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class TranslationRotationScale3DBatch(BaseBatch[TranslationRotationScale3DArrayLike]):
    _ARROW_TYPE = TranslationRotationScale3DType()

    @staticmethod
    def _native_to_pa_array(data: TranslationRotationScale3DArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError(
            "Arrow serialization of TranslationRotationScale3D not implemented: We lack codegen for arrow-serialization of general structs"
        )  # You need to implement native_to_pa_array_override in translation_rotation_scale3d_ext.py
