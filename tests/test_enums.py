import pytest

from windows_fonts import Style, Weight


def test_style():
    style = Style.NORMAL
    assert repr(style) == "Style.NORMAL"
