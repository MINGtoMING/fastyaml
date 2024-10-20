from typing import Any

from . import fastyaml as _fastyaml

__doc__ = _fastyaml.__doc__
if hasattr(_fastyaml, "__all__"):
    __all__ = _fastyaml.__all__


def load(fp) -> Any:
    return _fastyaml.loads(fp.read())


def loads(s: str) -> Any:
    return _fastyaml.loads(s)


def dump(obj: Any, fp) -> None:
    fp.write(_fastyaml.dumps(obj))


def dumps(obj: Any) -> str:
    return _fastyaml.dumps(obj)
