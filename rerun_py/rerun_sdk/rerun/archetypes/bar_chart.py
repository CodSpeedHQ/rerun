# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/bar_chart.fbs".

# You can extend this class by creating a "BarChartExt" class in "bar_chart_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components, datatypes
from .._baseclasses import Archetype
from ..error_utils import catch_and_log_exceptions
from .bar_chart_ext import BarChartExt

__all__ = ["BarChart"]


@define(str=False, repr=False, init=False)
class BarChart(BarChartExt, Archetype):
    """
    **Archetype**: A bar chart.

    The x values will be the indices of the array, and the bar heights will be the provided values.

    Example
    -------
    ### `bar_chart`:
    ```python

    import rerun as rr

    rr.init("rerun_example_bar_chart", spawn=True)
    rr.log("bar_chart", rr.BarChart([8, 4, 0, 9, 1, 4, 1, 6, 9, 0]))
    ```
    """

    def __init__(self: Any, values: datatypes.TensorDataLike):
        """
        Create a new instance of the BarChart archetype.

        Parameters
        ----------
        values:
             The values. Should always be a rank-1 tensor.
        """

        # You can define your own __init__ function as a member of BarChartExt in bar_chart_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(values=values)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            values=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> BarChart:
        """Produce an empty BarChart, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    values: components.TensorDataBatch = field(
        metadata={"component": "required"},
        converter=BarChartExt.values__field_converter_override,  # type: ignore[misc]
    )
    # The values. Should always be a rank-1 tensor.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
