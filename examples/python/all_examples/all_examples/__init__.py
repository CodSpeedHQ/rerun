from __future__ import annotations

import platform
from dataclasses import field, dataclass
from pathlib import Path
from typing import Iterable, Any

import tomli
from pyproject_metadata import StandardMetadata


# def _relative(target: Path, origin: Path) -> Path:
#     """Return target path relative to the origin, allowing for walking up.
#
#     From https://stackoverflow.com/a/71874881/229511
#     Note: Path.relative_to(origin, walk_up=True) is only available in Python 3.12
#     """
#     try:
#         return Path(target).resolve().relative_to(Path(origin).resolve())
#     except ValueError as e:  # target does not start with origin
#         # recursion with origin (eventually origin is root so try will succeed)
#         return Path("..").joinpath(_relative(target, Path(origin).parent))


@dataclass
class RerunMetadata:
    """Extract Rerun example metadata from a pyproject.toml data.

    Expected format in the pyproject.toml:

        [tool.rerun-example]
        skip = true
        extra-args = "--help"  # may also be a list
    """

    skip: bool
    extra_args: list[str]

    @classmethod
    def from_pyproject(cls, pyproject_data: dict[str, Any]) -> RerunMetadata:
        rerun_data = pyproject_data.get("tool", {}).get("rerun-example", {})

        skip = rerun_data.pop("skip", False)
        extra_args = rerun_data.pop("extra-args", [])
        if isinstance(extra_args, str):
            extra_args = [extra_args]

        if not len(rerun_data) == 0:
            raise ValueError(f"Unsupported fields in the rerun-example metadata: {', '.join(rerun_data.keys())}")

        return cls(skip=skip, extra_args=extra_args)


@dataclass
class Example:
    path: Path
    name: str = field(init=False)
    standard_metadata: StandardMetadata = field(init=False)
    rerun_metadata: RerunMetadata = field(init=False)

    def __post_init__(self):
        self.name = self.path.name
        pyproject_data = tomli.loads(Path(self.path / "pyproject.toml").read_text())
        self.standard_metadata = StandardMetadata.from_pyproject(pyproject_data, self.path)
        self.rerun_metadata = RerunMetadata.from_pyproject(pyproject_data)

    def active(self) -> bool:
        """Check that this example is active given its metadata but disregarding compatibility with the current Python
        version.
        """
        return not self.rerun_metadata.skip

    def compatible(self) -> bool:
        """Check that this example is compatible with the current Python version."""
        requires_python = self.standard_metadata.requires_python
        if requires_python is not None:
            return requires_python.contains(platform.python_version())

        return True

    def environment_specifier(self) -> str:
        """Returns an environment specifier as per the dependency specification."""
        spec = ""
        if self.standard_metadata.requires_python is not None:
            spec += " ; "
            spec += " and ".join(
                f"python_version {v.operator} '{v.version}'" for v in self.standard_metadata.requires_python
            )

        return spec


def active_examples() -> Iterable[Example]:
    """Iterator over all active examples."""
    example_dir = Path(__file__).parent.parent.parent

    our_name = Path(__file__).parent.parent.name
    for example_path in example_dir.glob("*"):
        if example_path.is_dir() and (example_path / "pyproject.toml").exists() and example_path.name != our_name:
            example = Example(example_path.absolute())

            if example.active():
                yield example
