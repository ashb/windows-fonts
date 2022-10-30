# windows-fonts

Enumerate and discover fonts on Windows

## Why this module exists

Most (all?) python modules that render text to an image (matplotlib, PIL/Pillow etc) need to take a _filename_
on Windows, but happily take a font name on other platforms, which is a) annoying from a cross-platform
standpoint, and b) requires a bit of "faff" for the user to discover the font file for a given font.

## Example

```python
from windows_fonts import FontCollection

fonts = FontCollection()

# TODO: Add a "get_best_match" to find the font name and variant
family = fonts['Arial']
variant = family[0]
print(variant.filename)
```

## Requirements

Python >= 3.7
Windows Vista and up
