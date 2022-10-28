import pytest

from windows_fonts import FontCollection, py_sum


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


def test_len(collection: FontCollection):
    assert len(collection) > 0


def test_get_index_key_same(collection: FontCollection):
    by_idx = collection[0]
    assert by_idx is not None

    by_key = collection[by_idx.name]
    assert by_key is not None

    assert by_idx == by_key
