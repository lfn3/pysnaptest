from ._lib_name import assert_json_snapshot as _assert_json_snapshot
from ._lib_name import assert_snapshot as _assert_snapshot
import os
import pathlib
from typing import Callable, Any, overload, Tuple
from functools import partial, wraps


def extract_snapshot_path(test_path: str) -> str:
    test_path_file = pathlib.Path(test_path)
    snapshot_dir = (
        test_path_file.resolve().parent
        if test_path_file.is_file()
        else pathlib.Path(test_path.split("/")[-1]).resolve().parent
    )
    return str(snapshot_dir)


def extract_from_pytest_env(
    snapshot_path: str | None = None, snapshot_name: str | None = None
) -> Tuple[str, str]:
    current_test = os.environ.get("PYTEST_CURRENT_TEST")
    (test_path, test_name) = current_test.split("::")
    if snapshot_path is None:
        snapshot_path = extract_snapshot_path(test_path)
    if snapshot_name is None:
        snapshot_name = (
            f"{test_path.split('/')[-1].replace('.py', '')}_{test_name.split(' ')[0]}"
        )
    return snapshot_path, snapshot_name


def assert_json_snapshot(
    result: Callable, snapshot_path: str | None = None, snapshot_name: str | None = None
):
    (snapshot_path, snapshot_name) = extract_from_pytest_env(
        snapshot_path, snapshot_name
    )
    _assert_json_snapshot(snapshot_path, snapshot_name, result)


def assert_snapshot(
    result: Callable, snapshot_path: str | None = None, snapshot_name: str | None = None
):
    (snapshot_path, snapshot_name) = extract_from_pytest_env(
        snapshot_path, snapshot_name
    )
    _assert_snapshot(snapshot_path, snapshot_name, result)


def insta_snapshot(
    result: Callable, snapshot_path: str | None = None, snapshot_name: str | None = None
):
    if isinstance(result, dict) or isinstance(result, list):
        assert_json_snapshot(result, snapshot_path, snapshot_name)
    else:
        assert_snapshot(result, snapshot_path, snapshot_name)


@overload
def snapshot(func: Callable) -> Callable: ...


@overload
def snapshot(
    *, filename: str | None = None, folder_path: str | None = None
) -> Callable:  # noqa: F811
    ...


def snapshot(  # noqa: F811
    func: Callable | None = None,
    *,
    snapshot_path: str | None = None,
    snapshot_name: str | None = None,
) -> Callable:
    def asserted_func(func: Callable, *args: Any, **kwargs: Any):
        result = func(*args, **kwargs)
        insta_snapshot(result, snapshot_path=snapshot_path, snapshot_name=snapshot_name)

    # Without arguments `func` is passed directly to the decorator
    if func is not None:
        if not callable(func):
            raise TypeError("Not a callable. Did you use a non-keyword argument?")
        return wraps(func)(partial(asserted_func, func))

    # With arguments, we need to return a function that accepts the function
    def decorator(func: Callable) -> Callable:
        return wraps(func)(partial(asserted_func, func))

    return decorator
