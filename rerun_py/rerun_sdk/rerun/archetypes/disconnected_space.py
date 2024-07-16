# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/disconnected_space.fbs".

# You can extend this class by creating a "DisconnectedSpaceExt" class in "disconnected_space_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)
from .disconnected_space_ext import DisconnectedSpaceExt

__all__ = ["DisconnectedSpace"]


@define(str=False, repr=False, init=False)
class DisconnectedSpace(DisconnectedSpaceExt, Archetype):
    """
    **Archetype**: Spatially disconnect this entity from its parent.

    Specifies that the entity path at which this is logged is spatially disconnected from its parent,
    making it impossible to transform the entity path into its parent's space and vice versa.
    It *only* applies to space views that work with spatial transformations, i.e. 2D & 3D space views.
    This is useful for specifying that a subgraph is independent of the rest of the scene.

    Example
    -------
    ### Disconnected space:
    ```python
    import rerun as rr

    rr.init("rerun_example_disconnected_space", spawn=True)

    # These two points can be projected into the same space..
    rr.log("world/room1/point", rr.Points3D([[0, 0, 0]]))
    rr.log("world/room2/point", rr.Points3D([[1, 1, 1]]))

    # ..but this one lives in a completely separate space!
    rr.log("world/wormhole", rr.DisconnectedSpace())
    rr.log("world/wormhole/point", rr.Points3D([[2, 2, 2]]))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/1200w.png">
      <img src="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/full.png" width="640">
    </picture>
    </center>

    """

    # __init__ can be found in disconnected_space_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            disconnected_space=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> DisconnectedSpace:
        """Produce an empty DisconnectedSpace, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    disconnected_space: components.DisconnectedSpaceBatch = field(
        metadata={"component": "required"},
        converter=components.DisconnectedSpaceBatch._required,  # type: ignore[misc]
    )
    # Whether the entity path at which this is logged is disconnected from its parent.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
