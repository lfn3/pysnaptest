from pysnaptest import snapshot, assert_json_snapshot


@snapshot
def test_snapshot_number() -> int:
    return 5


@snapshot
def test_snapshot_dict_result() -> dict[str, str]:
    return {"test": 2}


@snapshot
def test_snapshot_list_result() -> list[str]:
    return [1, 2, 4]

def test_assert_json_snapshot() -> list[str]:
    assert_json_snapshot({"assert_json_snapshot": "expected_result"})

def test_assert_snapshot() -> list[str]:
    assert_json_snapshot("expected_result")

def test_assert_snapshot_multiple() -> list[str]:
    snapshot_name_prefix = "test_snapshots_test_assert_snapshot_multiple"
    assert_json_snapshot("expected_result_1", snapshot_name=f"{snapshot_name_prefix}_1")
    assert_json_snapshot("expected_result_2", snapshot_name=f"{snapshot_name_prefix}_2")
