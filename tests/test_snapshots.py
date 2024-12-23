from pysnaptest import snapshot

@snapshot
def test_snapshot_number() -> int:
    return 5

@snapshot
def test_snapshot_dict_result() -> dict[str, str]:
    return {"test": 2}

@snapshot
def test_snapshot_list_result() -> list[str]:
    return [1, 2, 4]
