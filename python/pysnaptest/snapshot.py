from ._lib_name import assert_json_snapshot, assert_snapshot
import os
import pathlib
from typing import Any, Callable


def insta_snapshot(result: Callable[Any, Any], filename: str | None = None, folder_path: str | None = None):

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

def snapshot(filename: str | None = None, folder_path: str | None = None):
    def decorator(fn_test: Callable[Any, Any]):
        def asserted_test(*args, **kwargs):
            result = fn_test(*args, **kwargs)
            insta_snapshot(result, filename=filename, folder_path=folder_path)

        return asserted_test
    return decorator