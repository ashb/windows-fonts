# windows-fonts

Enumerate and discover fonts on Windows

## Why this module exists

Most (all?) python modules that render text to an image (matplotlib, PIL/Pillow etc) need to take a _filename_
on Windows, but happily take a font name on other platforms, which is a) annoying from a cross-platform
standpoint, and b) requires a bit of "faff" for the user to discover the font file for a given font.

## Synopsis

```python console
>>> from windows_fonts import FontCollection, Weight
>>> fonts = FontCollection()

>>> # Get the first variant (light/regular/bold etc) for a named family
>>> family = fonts['Arial']

>>> variant = family[0]
>>> variant
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.REGULAR>
>>> print(variant.filename, variant.weight, variant.style)
C:\WINDOWS\FONTS\ARIAL.TTF Weight.REGULAR Style.NORMAL

# Find the "closest" variant for a given family
>>> variant = family.get_best_variant(weight=Weight.BOLD)  # Or `style=Style.ITALIC, or both
>>> variant
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>

# Or to find all "matching" variants in priority order:
>>> for variant  in family.get_matching_variants(weight=Weight.BOLD):
...    variant
...
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BLACK>
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
```

## Requirements

Python >= 3.7
Windows Vista and up
