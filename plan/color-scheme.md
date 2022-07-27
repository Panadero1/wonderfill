Ideas for coloring my game

All sprites right now are black & white with an inversed version for nighttime

^Daytime sprites have more white than black typically (more specifically, the fill color is usually white)

This was initially a design choice but I now realize that this is ugly and kinda flashbangy

With the graphics library, I can choose to tint a sprite on drawing

The darker the color on the sprite, the darker the tinted color is

^Therefore, all palette colors should be bright and darkness is applied on the sprite-side

_The sprites are basically a layer mask_

I plan to use this feature to make it seem like an intentional design choice :)

This document will detail the color palette I plan to use as default, and, eventually alternate color palettes for variety and colorblind accessibility

Each palette's colors need to work well together

## Palette ideas

* Default
* Pastel
* Neon bright
* Red-green colorblind
  * Deuteranomaly (green->red)
  * Protanomaly (red->green)
  * Protanopia/deuteranopia (red:green)
* Blue-yellow colorblind
  * Tritanomaly (blue:green) (yellow:red)
  * Tritanopia (blue:green) (purple:red) (yellow:pink)
* Complete colorblind (greyscale)

---
## Default
Item|Color|Color value
----|-----|-----------
Main character|White|<span style="color:black;background-color:#FFFFFF;">FFFFFF</span>
**Tiles**|-|-
Decoration|Grey|<span style="color:black;background-color:#808080;">808080</span>
Interact|Yellow|<span style="color:black;background-color:#FFFF40;">FFFF40</span>
**Entities**|-|-
Enemy|Red|<span style="color:black;background-color:#FF4040;">FF4040</span>
Friendly|Green|<span style="color:black;background-color:#40FF40;">40FF40</span>
Expand as needed

## Pastel
## Neon bright
## Deuteranomaly
## Protanomaly
## Protanopia/deuteranopia
## Tritanomaly
## Tritanopia
## Complete colorblind (greyscale)

---
## Eventual / If I feel like it
Thinking of writing shader code for custom dynamic coloring

i.e. alternating or randomly-generated colors