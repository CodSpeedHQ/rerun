from __future__ import annotations

from enum import IntEnum
from typing import TYPE_CHECKING, Any, Iterable, cast

import numpy as np
import numpy.typing as npt
import pyarrow as pa

if TYPE_CHECKING:
    from .._log import ComponentBatchLike
    from . import ViewCoordinates, ViewCoordinatesArrayLike


class ViewCoordinatesExt:
    """Extension for [ViewCoordinates][rerun.components.ViewCoordinates]."""

    class ViewDir(IntEnum):
        Unused = 0
        Up = 1
        Down = 2
        Right = 3
        Left = 4
        Forward = 5
        Back = 6

    @staticmethod
    def coordinates__field_converter_override(data: npt.ArrayLike) -> npt.NDArray[np.uint8]:
        coordinates = np.asarray(data, dtype=np.uint8)
        if coordinates.shape != (3,):
            raise ValueError(f"ViewCoordinates must be a 3-element array. Got: {coordinates.shape}")
        return coordinates

    @staticmethod
    def native_to_pa_array_override(data: ViewCoordinatesArrayLike, data_type: pa.DataType) -> pa.Array:
        from . import ViewCoordinates, ViewCoordinatesLike

        if isinstance(data, ViewCoordinates):
            # ViewCoordinates
            data = [data.coordinates]
        elif hasattr(data, "__len__") and len(data) > 0 and isinstance(data[0], ViewCoordinates):  # type: ignore[arg-type, index]
            # [ViewCoordinates]
            data = [d.coordinates for d in data]  # type: ignore[union-attr]
        else:
            data = cast(ViewCoordinatesLike, data)
            try:
                # [x, y, z]
                data = [ViewCoordinates(data).coordinates]
            except ValueError:
                # [[x, y, z], ...]
                data = [ViewCoordinates(d).coordinates for d in data]  # type: ignore[union-attr]

        data = np.asarray(data, dtype=np.uint8)

        if len(data.shape) != 2 or data.shape[1] != 3:
            raise ValueError(f"ViewCoordinates must be a 3-element array. Got: {data.shape}")

        data = data.flatten()

        for value in data:
            # TODO(jleibs): Enforce this validation based on ViewDir
            if value not in range(1, 7):
                raise ValueError("ViewCoordinates must contain only values in the range [1,6].")

        return pa.FixedSizeListArray.from_arrays(data, type=data_type)

    # Implement the AsComponents protocol
    def as_component_batches(self) -> Iterable[ComponentBatchLike]:
        from ..archetypes import ViewCoordinates
        from ..components import ViewCoordinates as ViewCoordinatesComponent

        return ViewCoordinates(cast(ViewCoordinatesComponent, self)).as_component_batches()

    def num_instances(self) -> int:
        # Always a mono-component
        return 1

    # <BEGIN_GENERATED:declarations>
    # This section is generated by running `scripts/generate_view_coordinate_defs.py --python`
    # The following declarations are replaced in `deferred_patch_class`.
    ULF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Left, Z=Forward"""

    UFL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Forward, Z=Left"""

    LUF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Up, Z=Forward"""

    LFU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Forward, Z=Up"""

    FUL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Up, Z=Left"""

    FLU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Left, Z=Up"""

    ULB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Left, Z=Back"""

    UBL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Back, Z=Left"""

    LUB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Up, Z=Back"""

    LBU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Back, Z=Up"""

    BUL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Up, Z=Left"""

    BLU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Left, Z=Up"""

    URF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Right, Z=Forward"""

    UFR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Forward, Z=Right"""

    RUF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Up, Z=Forward"""

    RFU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Forward, Z=Up"""

    FUR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Up, Z=Right"""

    FRU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Right, Z=Up"""

    URB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Right, Z=Back"""

    UBR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Back, Z=Right"""

    RUB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Up, Z=Back"""

    RBU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Back, Z=Up"""

    BUR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Up, Z=Right"""

    BRU: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Right, Z=Up"""

    DLF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Left, Z=Forward"""

    DFL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Forward, Z=Left"""

    LDF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Down, Z=Forward"""

    LFD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Forward, Z=Down"""

    FDL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Down, Z=Left"""

    FLD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Left, Z=Down"""

    DLB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Left, Z=Back"""

    DBL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Back, Z=Left"""

    LDB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Down, Z=Back"""

    LBD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Left, Y=Back, Z=Down"""

    BDL: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Down, Z=Left"""

    BLD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Left, Z=Down"""

    DRF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Right, Z=Forward"""

    DFR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Forward, Z=Right"""

    RDF: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Down, Z=Forward"""

    RFD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Forward, Z=Down"""

    FDR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Down, Z=Right"""

    FRD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Forward, Y=Right, Z=Down"""

    DRB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Right, Z=Back"""

    DBR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Back, Z=Right"""

    RDB: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Down, Z=Back"""

    RBD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Back, Z=Down"""

    BDR: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Down, Z=Right"""

    BRD: ViewCoordinates = None  # type: ignore[assignment]
    """X=Back, Y=Right, Z=Down"""

    RIGHT_HAND_X_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Right, Z=Forward"""

    RIGHT_HAND_X_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Right, Z=Back"""

    RIGHT_HAND_Y_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Up, Z=Back"""

    RIGHT_HAND_Y_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Down, Z=Forward"""

    RIGHT_HAND_Z_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Forward, Z=Up"""

    RIGHT_HAND_Z_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Back, Z=Down"""

    LEFT_HAND_X_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Up, Y=Right, Z=Back"""

    LEFT_HAND_X_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Down, Y=Right, Z=Forward"""

    LEFT_HAND_Y_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Up, Z=Forward"""

    LEFT_HAND_Y_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Down, Z=Back"""

    LEFT_HAND_Z_UP: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Back, Z=Up"""

    LEFT_HAND_Z_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    """X=Right, Y=Forward, Z=Down"""

    # <END_GENERATED:declarations>

    @staticmethod
    def deferred_patch_class(cls: Any) -> None:
        # <BEGIN_GENERATED:definitions>
        # This section is generated by running `scripts/generate_view_coordinate_defs.py --python`
        cls.ULF = cls([cls.ViewDir.Up, cls.ViewDir.Left, cls.ViewDir.Forward])
        cls.UFL = cls([cls.ViewDir.Up, cls.ViewDir.Forward, cls.ViewDir.Left])
        cls.LUF = cls([cls.ViewDir.Left, cls.ViewDir.Up, cls.ViewDir.Forward])
        cls.LFU = cls([cls.ViewDir.Left, cls.ViewDir.Forward, cls.ViewDir.Up])
        cls.FUL = cls([cls.ViewDir.Forward, cls.ViewDir.Up, cls.ViewDir.Left])
        cls.FLU = cls([cls.ViewDir.Forward, cls.ViewDir.Left, cls.ViewDir.Up])
        cls.ULB = cls([cls.ViewDir.Up, cls.ViewDir.Left, cls.ViewDir.Back])
        cls.UBL = cls([cls.ViewDir.Up, cls.ViewDir.Back, cls.ViewDir.Left])
        cls.LUB = cls([cls.ViewDir.Left, cls.ViewDir.Up, cls.ViewDir.Back])
        cls.LBU = cls([cls.ViewDir.Left, cls.ViewDir.Back, cls.ViewDir.Up])
        cls.BUL = cls([cls.ViewDir.Back, cls.ViewDir.Up, cls.ViewDir.Left])
        cls.BLU = cls([cls.ViewDir.Back, cls.ViewDir.Left, cls.ViewDir.Up])
        cls.URF = cls([cls.ViewDir.Up, cls.ViewDir.Right, cls.ViewDir.Forward])
        cls.UFR = cls([cls.ViewDir.Up, cls.ViewDir.Forward, cls.ViewDir.Right])
        cls.RUF = cls([cls.ViewDir.Right, cls.ViewDir.Up, cls.ViewDir.Forward])
        cls.RFU = cls([cls.ViewDir.Right, cls.ViewDir.Forward, cls.ViewDir.Up])
        cls.FUR = cls([cls.ViewDir.Forward, cls.ViewDir.Up, cls.ViewDir.Right])
        cls.FRU = cls([cls.ViewDir.Forward, cls.ViewDir.Right, cls.ViewDir.Up])
        cls.URB = cls([cls.ViewDir.Up, cls.ViewDir.Right, cls.ViewDir.Back])
        cls.UBR = cls([cls.ViewDir.Up, cls.ViewDir.Back, cls.ViewDir.Right])
        cls.RUB = cls([cls.ViewDir.Right, cls.ViewDir.Up, cls.ViewDir.Back])
        cls.RBU = cls([cls.ViewDir.Right, cls.ViewDir.Back, cls.ViewDir.Up])
        cls.BUR = cls([cls.ViewDir.Back, cls.ViewDir.Up, cls.ViewDir.Right])
        cls.BRU = cls([cls.ViewDir.Back, cls.ViewDir.Right, cls.ViewDir.Up])
        cls.DLF = cls([cls.ViewDir.Down, cls.ViewDir.Left, cls.ViewDir.Forward])
        cls.DFL = cls([cls.ViewDir.Down, cls.ViewDir.Forward, cls.ViewDir.Left])
        cls.LDF = cls([cls.ViewDir.Left, cls.ViewDir.Down, cls.ViewDir.Forward])
        cls.LFD = cls([cls.ViewDir.Left, cls.ViewDir.Forward, cls.ViewDir.Down])
        cls.FDL = cls([cls.ViewDir.Forward, cls.ViewDir.Down, cls.ViewDir.Left])
        cls.FLD = cls([cls.ViewDir.Forward, cls.ViewDir.Left, cls.ViewDir.Down])
        cls.DLB = cls([cls.ViewDir.Down, cls.ViewDir.Left, cls.ViewDir.Back])
        cls.DBL = cls([cls.ViewDir.Down, cls.ViewDir.Back, cls.ViewDir.Left])
        cls.LDB = cls([cls.ViewDir.Left, cls.ViewDir.Down, cls.ViewDir.Back])
        cls.LBD = cls([cls.ViewDir.Left, cls.ViewDir.Back, cls.ViewDir.Down])
        cls.BDL = cls([cls.ViewDir.Back, cls.ViewDir.Down, cls.ViewDir.Left])
        cls.BLD = cls([cls.ViewDir.Back, cls.ViewDir.Left, cls.ViewDir.Down])
        cls.DRF = cls([cls.ViewDir.Down, cls.ViewDir.Right, cls.ViewDir.Forward])
        cls.DFR = cls([cls.ViewDir.Down, cls.ViewDir.Forward, cls.ViewDir.Right])
        cls.RDF = cls([cls.ViewDir.Right, cls.ViewDir.Down, cls.ViewDir.Forward])
        cls.RFD = cls([cls.ViewDir.Right, cls.ViewDir.Forward, cls.ViewDir.Down])
        cls.FDR = cls([cls.ViewDir.Forward, cls.ViewDir.Down, cls.ViewDir.Right])
        cls.FRD = cls([cls.ViewDir.Forward, cls.ViewDir.Right, cls.ViewDir.Down])
        cls.DRB = cls([cls.ViewDir.Down, cls.ViewDir.Right, cls.ViewDir.Back])
        cls.DBR = cls([cls.ViewDir.Down, cls.ViewDir.Back, cls.ViewDir.Right])
        cls.RDB = cls([cls.ViewDir.Right, cls.ViewDir.Down, cls.ViewDir.Back])
        cls.RBD = cls([cls.ViewDir.Right, cls.ViewDir.Back, cls.ViewDir.Down])
        cls.BDR = cls([cls.ViewDir.Back, cls.ViewDir.Down, cls.ViewDir.Right])
        cls.BRD = cls([cls.ViewDir.Back, cls.ViewDir.Right, cls.ViewDir.Down])
        cls.RIGHT_HAND_X_UP = cls([cls.ViewDir.Up, cls.ViewDir.Right, cls.ViewDir.Forward])
        cls.RIGHT_HAND_X_DOWN = cls([cls.ViewDir.Down, cls.ViewDir.Right, cls.ViewDir.Back])
        cls.RIGHT_HAND_Y_UP = cls([cls.ViewDir.Right, cls.ViewDir.Up, cls.ViewDir.Back])
        cls.RIGHT_HAND_Y_DOWN = cls([cls.ViewDir.Right, cls.ViewDir.Down, cls.ViewDir.Forward])
        cls.RIGHT_HAND_Z_UP = cls([cls.ViewDir.Right, cls.ViewDir.Forward, cls.ViewDir.Up])
        cls.RIGHT_HAND_Z_DOWN = cls([cls.ViewDir.Right, cls.ViewDir.Back, cls.ViewDir.Down])
        cls.LEFT_HAND_X_UP = cls([cls.ViewDir.Up, cls.ViewDir.Right, cls.ViewDir.Back])
        cls.LEFT_HAND_X_DOWN = cls([cls.ViewDir.Down, cls.ViewDir.Right, cls.ViewDir.Forward])
        cls.LEFT_HAND_Y_UP = cls([cls.ViewDir.Right, cls.ViewDir.Up, cls.ViewDir.Forward])
        cls.LEFT_HAND_Y_DOWN = cls([cls.ViewDir.Right, cls.ViewDir.Down, cls.ViewDir.Back])
        cls.LEFT_HAND_Z_UP = cls([cls.ViewDir.Right, cls.ViewDir.Back, cls.ViewDir.Up])
        cls.LEFT_HAND_Z_DOWN = cls([cls.ViewDir.Right, cls.ViewDir.Forward, cls.ViewDir.Down])
        # <END_GENERATED:definitions>
