import collections.abc

import pytest

from windows_fonts import FontCollection, FontVariant, Style, Weight, get_matching_variants


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


@pytest.fixture
def family(collection):
    return collection['Arial']


@pytest.fixture
def variant(family):
    return family[0]


def test_get_matching_variants(collection: FontCollection):
    vars = get_matching_variants(full_name="Arial Bold Italic")

    assert len(vars) == 1

    var = vars[0]
    assert var.style == Style.ITALIIC
    assert var.weight == Weight.BOLD
    assert var.name == "Bold Italic"
    assert var.family == collection["Arial"]


@pytest.mark.parametrize(
    ["kwargs", "match"],
    [
        [{"not_a_prop": "string"}, r"isn't a known font property name"],
        [{"copyright": "string"}, r"doesn't have a mapping to font property id"],
        [{"win32_family_names": 1.0}, r"'float' object cannot be converted to"],
        [{}, r"no filter conditions passed"],
    ],
)
def test_get_matching_variants_errors(kwargs, match: str, collection: FontCollection):
    with pytest.raises(TypeError, match=match):
        get_matching_variants(**kwargs)


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
