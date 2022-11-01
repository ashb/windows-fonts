import pytest

from windows_fonts import FontCollection, FontFamily, Style, Weight


@pytest.fixture(scope="module")
def collection():
    return FontCollection()


@pytest.fixture
def family(collection: FontCollection):
    return collection['Arial']


@pytest.mark.parametrize(
    ["weight", "italic", "expected_props"],
    [
        pytest.param(None, None, {"weight": Weight.REGULAR, "style": Style.NORMAL}, id="None,None"),
        pytest.param(700, None, {"weight": Weight.BOLD, "style": Style.NORMAL}, id="700,None"),
        pytest.param(Weight.BOLD, None, {"weight": Weight.BOLD, "style": Style.NORMAL}, id="BOLD,None"),
        pytest.param(Weight.BOLD, True, {"weight": Weight.BOLD, "style": Style.ITALIIC}, id="BOLD,True"),
    ],
)
def test_get_best_match(weight, italic, expected_props, family: FontFamily):
    var = family.get_best_variant(weight=weight, italic=italic)

    for (name, val) in expected_props.items():
        assert getattr(var, name) == val