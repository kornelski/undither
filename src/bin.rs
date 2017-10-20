#include "lodepng.h"
#include "undither.h"
#include <stdlib.h>
#include <stdio.h>

int main(int argc, char **argv) {
    unsigned error;
    unsigned char* image;
    unsigned width, height;
    unsigned char* png;
    size_t pngsize;
    LodePNGState state;

    if (argc != 3) {
        fprintf(stderr, "Usage: %s input-8bit.png output-32bit.png\n\nVersion 0.2, © 2014 Kornel Lesiński <kornel@geekhood.net>\nhttps://github.com/pornel/undither\n\n", argv[0]);
        return 1;
    }

    lodepng_state_init(&state);
    state.decoder.color_convert = 0;
    state.info_raw.colortype = LCT_PALETTE;
    state.info_raw.bitdepth = 8;
    error = lodepng_load_file(&png, &pngsize, argv[1]);
    if (!error) {
        error = lodepng_decode(&image, &width, &height, &state, png, pngsize);
    }
    free(png);
    if (error) {
        fprintf(stderr, "error when loading '%s': %s\n", argv[1], lodepng_error_text(error));
        return error;
    }

    if (state.info_raw.bitdepth != 8) {
        fprintf(stderr, "Only 256-color images are supported\n");
        return 1;
    }

    const rgba *pal = (rgba *)state.info_raw.palette;
    if (!pal || state.info_raw.colortype != LCT_PALETTE) {
        fprintf(stderr, "No pal?\n");
        return 1;
    }
    rgba *out = malloc(width*height*4);

    undither(image, pal, width, height, out);

    lodepng_state_cleanup(&state);
    free(image);

    error = lodepng_encode32_file(argv[2], (unsigned char*)out, width, height);
    if (error) {
        fprintf(stderr, "error when saving '%s': %s\n", argv[2], lodepng_error_text(error));
        return error;
    }

    return 0;
}
