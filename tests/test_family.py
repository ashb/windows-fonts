import os

import pytest

from windows_fonts import FontCollection, FontFamily, Style, Weight


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


@pytest.fixture
def family(collection: FontCollection):
    return collection['Arial']


@pytest.mark.parametrize(
    ["weight", "style", "expected_props"],
    [
        pytest.param(None, None, {"weight": Weight.REGULAR, "style": Style.NORMAL}, id="None,None"),
        pytest.param(700, None, {"weight": Weight.BOLD, "style": Style.NORMAL}, id="700,None"),
        pytest.param(Weight.BOLD, None, {"weight": Weight.BOLD, "style": Style.NORMAL}, id="BOLD,None"),
        pytest.param(Weight.BOLD, Style.ITALIIC, {"weight": Weight.BOLD, "style": Style.ITALIIC}, id="BOLD,True"),
    ],
)
def test_get_best_match(weight, style, expected_props, family: FontFamily):
    var = family.get_best_variant(weight=weight, style=style)

    for (name, val) in expected_props.items():
        assert getattr(var, name) == val


@pytest.mark.parametrize(
    ["width", "weight", "italic", "expected_props"],
    [
        pytest.param(
            100,
            None,
            None,
            {"weight": Weight.REGULAR, "style": Style.NORMAL, "file_stem": "ARIAL.TTF"},
            id="100,None,None",
        ),
        pytest.param(
            100,
            None,
            True,
            {"weight": Weight.REGULAR, "style": Style.ITALIIC, "file_stem": "ARIALI.TTF"},
            id="100,None,False",
        ),
        pytest.param(
            100,
            700,
            True,
            {"weight": Weight.BOLD, "style": Style.ITALIIC, "file_stem": "ARIALBI.TTF"},
            id="100,700,True",
        ),
        pytest.param(
            100,
            Weight.BOLD,
            True,
            {"weight": Weight.BOLD, "style": Style.ITALIIC, "file_stem": "ARIALBI.TTF"},
            id="100,BOLD,True",
        ),
    ],
)
def test_get_best_match_dwrite3(width, weight, italic, expected_props, family: FontFamily):
    """Test the _get_dwrite3_matching_variants code path"""
    var = family.get_best_variant(weight=weight, width=width, italic=italic)

    for (name, expected) in expected_props.items():
        if name == "file_stem":
            val = os.path.basename(var.filename).upper()
        else:
            val = getattr(var, name)
        assert val == expected


def test_get_matching_variants(family: FontFamily):
    variants = family.get_matching_variants()
    assert isinstance(variants, list)
    assert len(variants) > 2


def test_repr(family: FontFamily):
    assert repr(family) == '<FontFamily name="Arial">'


def test_len(family: FontFamily):
    # Lets just check it's an in int in a plausible range
    assert 2 < len(family) < 25
