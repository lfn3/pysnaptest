from pysnaptest import snapshot, assert_json_snapshot, assert_dataframe_snapshot
import pandas as pd
import polars as pl
import pytest


@snapshot
def test_snapshot_number() -> int:
    return 5


@snapshot
def test_snapshot_dict_result() -> dict[str, str]:
    return {"test": 2}


@snapshot
def test_snapshot_list_result() -> list[str]:
    return [1, 2, 4]


def test_assert_json_snapshot():
    assert_json_snapshot({"assert_json_snapshot": "expected_result"})


def test_assert_snapshot():
    assert_json_snapshot("expected_result")


def test_assert_pandas_dataframe_snapshot():
    df = pd.DataFrame({"name": ["foo", "bar"], "id": [1, 2]})
    assert_dataframe_snapshot(df, index=False)


@snapshot
def test_assert_polars_dataframe_snapshot() -> pl.DataFrame:
    return pl.DataFrame(
        {
            "foo": [1, 2, 3, 4, 5],
            "bar": [6, 7, 8, 9, 10],
            "ham": ["a", "b", "c", "d", "e"],
        }
    )


@pytest.mark.asyncio
@snapshot
async def test_snapshot_async() -> int:
    return 5


def test_assert_snapshot_multiple():
    snapshot_name_prefix = "test_snapshots_test_assert_snapshot_multiple"
    assert_json_snapshot("expected_result_1", snapshot_name=f"{snapshot_name_prefix}_1")
    assert_json_snapshot("expected_result_2", snapshot_name=f"{snapshot_name_prefix}_2")
