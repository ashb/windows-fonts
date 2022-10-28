import pytest

from windows_fonts import FontCollection, Style, Weight


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


@pytest.fixture
def family(collection):
    return collection['Arial']


@pytest.fixture
def variant(family):
    return family[0]


def test_filename(variant):
    variant.filename.lower().endswith("ARIAL.TTF")


def test_style(variant):
    assert isinstance(variant.style, Style)


def test_weight(variant):
    assert isinstance(variant.weight, Weight)
