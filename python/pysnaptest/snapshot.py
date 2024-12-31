from ._lib_name import assert_json_snapshot, assert_snapshot
import os
import pathlib
from typing import Callable, Any, overload
from functools import partial, wraps

def insta_snapshot(result: Callable, filename: str | None = None, folder_path: str | None = None):

    current_test = os.environ.get('PYTEST_CURRENT_TEST')
    (test_path, test_name) = current_test.split("::")
    if folder_path is None:
        test_path_file = pathlib.Path(test_path)
        if test_path_file.is_file():
            folder_path = str(test_path_file.resolve().parent)
        else:
            folder_path = str(pathlib.Path(test_path.split("/")[-1]).resolve().parent)
    if filename is None:
        filename = f"{test_path.split('/')[-1].replace('.py', '')}_{test_name.split(' ')[0]}"

    if isinstance(result, dict) or isinstance(result, list):
        assert_json_snapshot(folder_path, filename, result)
    else:
        assert_snapshot(folder_path, filename, result)

@overload
def snapshot(func: Callable) -> Callable:
    ...


@overload
def snapshot(*, filename: str | None = None, folder_path: str | None = None) -> Callable:  # noqa: F811
    ...


def snapshot(  # noqa: F811
    func: Callable | None = None, *, filename: str | None = None, folder_path: str | None = None
) -> Callable:

    def asserted_func(func: Callable, *args: Any, **kwargs: Any):
        result = func(*args, **kwargs)
        insta_snapshot(result, filename=filename, folder_path=folder_path)

    # Without arguments `func` is passed directly to the decorator
    if func is not None:
        if not callable(func):
            raise TypeError("Not a callable. Did you use a non-keyword argument?")
        return wraps(func)(partial(asserted_func, func))

    # With arguments, we need to return a function that accepts the function
    def decorator(func: Callable) -> Callable:
        return wraps(func)(partial(asserted_func, func))
    return decorator