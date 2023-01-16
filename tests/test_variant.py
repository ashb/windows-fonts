import collections.abc

import pytest

from windows_fonts import FontCollection, FontVariant, Style, Weight


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


@pytest.fixture
def family(collection):
    return collection['Arial']


@pytest.fixture
def variant(family):
    return family[0]


def test_name(variant):
    assert variant.name == "Regular"


def test_repr(variant):
    rep = repr(variant)
    assert rep.startswith('<FontVariant name=Regular, family=<FontFamily name="Arial">,')


def test_filename(variant):
    variant.filename.lower().endswith("ARIAL.TTF")


def test_style(variant):
    assert isinstance(variant.style, Style)


def test_weight(variant):
    assert isinstance(variant.weight, Weight)


def test_information(variant: FontVariant):
    info = variant.information

    assert "copyright" in info
    assert 1 in info  # The constant for copyright, as defined by the DirectWrite enums
    assert 1.0 not in info

    assert "copyright" in info.keys()
    assert info["copyright"] in info.values()
    assert ("copyright", info["copyright"] in info.items())
    assert iter(info)
    assert len(info) > 1

    # Test the iterator works
    assert "copyright" in [*iter(info)]

    with pytest.raises(KeyError):
        info[0]

    with pytest.raises(KeyError):
        info['madeup']
