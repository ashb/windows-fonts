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
<FontVariant name=Regular, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.REGULAR>
>>> print(variant.filename, variant.weight, variant.style)
C:\WINDOWS\FONTS\ARIAL.TTF Weight.REGULAR Style.NORMAL

>>> # Find the "closest" variant for a given family
>>> variant = family.get_best_variant(weight=Weight.BOLD)  # Or `style=Style.ITALIC, or both
>>> variant
<FontVariant name=Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>

>>> # Or to find all "matching" variants in priority order:
>>> for variant  in family.get_matching_variants(weight=Weight.BOLD):
...    variant
...
<FontVariant name=Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant name=Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant name=Black, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BLACK>
<FontVariant name=Narrow Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant name=Narrow Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>
<FontVariant name=Bold Italic, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant name=Italic Bold, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant name=Narrow Bold Italic, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
<FontVariant name=Narrow Italic Bold, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>
```

### Find a specific Font Variant

Some font families are aggregated, for instance Arial and Arial Narrow both are placed under the "Arial" family by the Win32 APIs. But if you want to be able to directly get to the Arial Narrow font files you need to use the top-level `get_matching_variants` function:

```python console
>>> get_matching_variants(win32_family_names="Arial Narrow")
[<FontVariant name=Narrow, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.REGULAR>,
 <FontVariant name=Narrow Italic, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.REGULAR>,
 <FontVariant name=Narrow Bold, family=<FontFamily name="Arial">, style=Style.NORMAL weight=Weight.BOLD>,
 <FontVariant name=Narrow Bold Italic, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>]
```

Or if you know the full name already:

```python console
>>> get_matching_variants(full_name="Arial Narrow Bold Italic")
[<FontVariant name=Narrow Bold Italic, family=<FontFamily name="Arial">, style=Style.ITALIIC weight=Weight.BOLD>]
```

### Get information about a Font Variant

`.information` is a dict-like object. The some keys will not be available on every font.

```python console
>>> info = variant.information
>>> import pprint
>>> pprint.pprint(dict(info))
{'copyright': '© 2008 The Monotype Corporation. All Rights Reserved.',
 'description': ...,
 'designer': 'Robin Nicholas, Patricia Saunders',
 'full_name': 'Arial Narrow Italic',
 'license_description': ...,
 'manufacturer': 'The Monotype Corporation',
 'postscript_name': 'ArialNarrow-Italic',
 'preferred_family_names': 'Arial',
 'preferred_subfamily_names': 'Narrow Italic',
 'trademark': 'Arial is a trademark of The Monotype Corporation in the United '
              'States and/or other countries.',
 'typographic_subfamily_names': 'Narrow Italic',
 'typographic_family_names': 'Arial',
 'versions': 'Version 2.40',
 'win32_family_names': 'Arial Narrow',
 'win32_subfamily_names': 'Italic'}
```

## Requirements

Python >= 3.7<br />
Windows Vista and up<br />
Some functions or methods need Windows 10 (`get_matching_variants` top-level function, and `FontFamily.get_matching_variants` when called with `width`, `slant`, `optical_size`, or `italic` parameters).
