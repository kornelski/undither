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

inline static bool is_closest(const unsigned char c1, const unsigned char c2, const rgba *pal, char *similarity) {
    const unsigned int pos = c1<c2 ? (c1<<8)|c2 : (c2<<8)|c1;
    const char res = similarity[pos];
    if (res >= 0) return res;

    const rgba p1 = pal[c1], p2 = pal[c2], avg = {
        .r = (p1.r + p2.r) / 2,
        .g = (p1.g + p2.g) / 2,
        .b = (p1.b + p2.b) / 2,
        .a = (p1.a + p2.a) / 2,
    };

    unsigned int min_diff = pal_diff(avg, p1);
    for(int i=0; i < 256; i++) {
        unsigned int diff = pal_diff(avg, pal[i]);
        if (diff < min_diff && i != c1 && i != c2) {
            return similarity[pos] = 0;
        }
    }
    return similarity[pos] = 1;
}

inline static void add_to_acc(rgba_sum *acc, const unsigned char center, const unsigned char idx, const rgba *pal, char *similarity, unsigned char w) {
    if (is_closest(center, idx, pal, similarity)) {
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

    lodepng_state_init(&state);
    state.decoder.color_convert = 0;
    state.info_raw.colortype = LCT_PALETTE;
    state.info_raw.bitdepth = 8;
    lodepng_load_file(&png, &pngsize, argv[1]);
    error = lodepng_decode(&image, &width, &height, &state, png, pngsize);
    free(png);
    if (error) {
        fprintf(stderr, "error %u: %s\n", error, lodepng_error_text(error));
        return 1;
    }

    if (state.info_raw.bitdepth != 8) {
        fprintf(stderr, "Fail! %d\n", state.info_raw.bitdepth);
        return 1;
    }

    const rgba *pal = (rgba *)state.info_raw.palette;
    if (!pal || state.info_raw.colortype != LCT_PALETTE) {
        fprintf(stderr, "No pal?\n");
        return 1;
    }
    rgba *out = malloc(width*height*4);
    char *similarity = malloc(256*256);
    memset(similarity, -1, 256*256);

    for(int y=1; y < height-2; y++) {
        for(int x=1; x < width-2; x++) {
            const int center = image[x+y*width];
            const rgba cpal = pal[center];
            rgba_sum acc = {
                .r = cpal.r * 2,
                .g = cpal.g * 2,
                .b = cpal.b * 2,
                .a = cpal.a * 2,
                .count = 2,
            };

            add_to_acc(&acc, center, image[(x-1)+(y-1)*width], pal, similarity, 1);
            add_to_acc(&acc, center, image[(x+0)+(y-1)*width], pal, similarity, 2);
            add_to_acc(&acc, center, image[(x+1)+(y-1)*width], pal, similarity, 1);

            add_to_acc(&acc, center, image[(x-1)+(y-0)*width], pal, similarity, 2);
            add_to_acc(&acc, center, image[(x+1)+(y-0)*width], pal, similarity, 2);

            add_to_acc(&acc, center, image[(x-1)+(y+1)*width], pal, similarity, 1);
            add_to_acc(&acc, center, image[(x+0)+(y+1)*width], pal, similarity, 2);
            add_to_acc(&acc, center, image[(x+1)+(y+1)*width], pal, similarity, 1);

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

    lodepng_encode32_file("testout.png", (unsigned char*)out, width, height);

    return 0;
}
