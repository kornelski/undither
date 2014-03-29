#include "lodepng.h"
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

typedef struct rgba {unsigned char r,g,b,a;} rgba;
typedef struct rgba_sum {unsigned int r,g,b,a,count;} rgba_sum;

inline static unsigned int pal_diff(rgba p1, rgba p2) {
    return (p1.r - p2.r) * (p1.r - p2.r) +
           (p1.g - p2.g) * (p1.g - p2.g) +
           (p1.b - p2.b) * (p1.b - p2.b) +
           (p1.a - p2.a) * (p1.a - p2.a);
}

inline static unsigned char similarity(const unsigned char c1, const unsigned char c2, const rgba *pal, char *simcache) {
    const unsigned int pos = c1<c2 ? (c1<<8)|c2 : (c2<<8)|c1;
    const char res = simcache[pos];
    if (res >= 0) return res;

    const rgba p1 = pal[c1], p2 = pal[c2], avg = {
        .r = (p1.r + p2.r) / 2,
        .g = (p1.g + p2.g) / 2,
        .b = (p1.b + p2.b) / 2,
        .a = (p1.a + p2.a) / 2,
    };

    const unsigned int allowed_diff = pal_diff(avg, p1);
    unsigned int min_diff = 1<<31;
    for(int i=0; i < 256; i++) {
        if (i == c1 || i == c2) continue;
        unsigned int diff = pal_diff(avg, pal[i]);
        if (diff < min_diff) {
            min_diff = diff;
        }
    }

    if (min_diff >= allowed_diff*2) {
        return simcache[pos] = 5;
    }
    if (min_diff >= allowed_diff) {
        return simcache[pos] = 4;
    }
    if (min_diff*2 >= allowed_diff) {
        return simcache[pos] = 1;
    }
    return simcache[pos] = 0;
}

inline static void add_to_acc(rgba_sum *acc, const unsigned char center, const unsigned char idx, const rgba *pal, char *simcache, unsigned char w) {
    unsigned char sim = similarity(center, idx, pal, simcache);
    if (sim) {
        w *= sim;
        rgba c = pal[idx];
        acc->r += c.r * w;
        acc->g += c.g * w;
        acc->b += c.b * w;
        acc->a += c.a * w;
        acc->count += w;
    }
}

int main(int argc, char **argv) {
    unsigned error;
    unsigned char* image;
    unsigned width, height;
    unsigned char* png;
    size_t pngsize;
    LodePNGState state;

    if (argc != 3) {
        fprintf(stderr, "Usage: %s input-8bit.png output-32bit.png\n\nVersion 0.1, © 2014 Kornel Lesiński <kornel@geekhood.net>\nhttps://github.com/pornel/undither\n\n", argv[0]);
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
    char *simcache = malloc(256*256);
    memset(simcache, -1, 256*256);

    for(int y=1; y < height-2; y++) {
        for(int x=1; x < width-2; x++) {
            const int center = image[x+y*width];
            const rgba cpal = pal[center];
            rgba_sum acc = {
                .r = cpal.r * 8,
                .g = cpal.g * 8,
                .b = cpal.b * 8,
                .a = cpal.a * 8,
                .count = 8,
            };

            add_to_acc(&acc, center, image[(x-1)+(y-1)*width], pal, simcache, 1);
            add_to_acc(&acc, center, image[(x+0)+(y-1)*width], pal, simcache, 2);
            add_to_acc(&acc, center, image[(x+1)+(y-1)*width], pal, simcache, 1);

            add_to_acc(&acc, center, image[(x-1)+(y-0)*width], pal, simcache, 2);
            add_to_acc(&acc, center, image[(x+1)+(y-0)*width], pal, simcache, 2);

            add_to_acc(&acc, center, image[(x-1)+(y+1)*width], pal, simcache, 1);
            add_to_acc(&acc, center, image[(x+0)+(y+1)*width], pal, simcache, 2);
            add_to_acc(&acc, center, image[(x+1)+(y+1)*width], pal, simcache, 1);

            out[x+y*width] = (rgba){
                .r = acc.r/acc.count,
                .g = acc.g/acc.count,
                .b = acc.b/acc.count,
                .a = acc.a/acc.count,
            };
        }
    }

    lodepng_state_cleanup(&state);
    free(image);

    error = lodepng_encode32_file(argv[2], (unsigned char*)out, width, height);
    if (error) {
        fprintf(stderr, "error when saving '%s': %s\n", argv[2], lodepng_error_text(error));
        return error;
    }

    return 0;
}
