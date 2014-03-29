Undither
========

Smart filter to remove Floyd-Steinberg dithering from paletted images.

The tool analyses image palette to find optimal blurring threshold. For any two adjacent pixels, if the palette has a color that is between colors of these two pixels, then it's assumed to be an edge.

![Dithered](https://cloud.githubusercontent.com/assets/72159/2558878/adb5e0ce-b75f-11e3-8ab4-3e78a4f32ecb.png)
→
![Undithered](https://cloud.githubusercontent.com/assets/72159/2558877/ad96f114-b75f-11e3-9768-b99f69748a90.png)

The algorithm should be useful when converting PNG8 to JPEG or anim-GIF to video (although at the moment the tool only reads PNG).

## Usage

    make
    ./undither input.png output.png

The input must be an 8-bit PNG image.
