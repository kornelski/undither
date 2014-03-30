#include "undither.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef struct rgba_sum {unsigned int r,g,b,a,count;} rgba_sum;
typedef struct prgba {unsigned short r,g,b,a;} prgba; // premultiplied rgba

inline static unsigned int pal_diff(prgba p1, prgba p2) {
    return (p1.r - p2.r) * (p1.r - p2.r) +
           (p1.g - p2.g) * (p1.g - p2.g) +
           (p1.b - p2.b) * (p1.b - p2.b) +
           (p1.a - p2.a) * (p1.a - p2.a);
}

inline static unsigned char similarity(const unsigned char c1, const unsigned char c2, const prgba *pal, char *simcache) {
    const unsigned int pos = c1<c2 ? (c1<<8)|c2 : (c2<<8)|c1;
    const char res = simcache[pos];
    if (res >= 0) return res;

    const prgba p1 = pal[c1], p2 = pal[c2], avg = {
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

inline static void add_to_acc(rgba_sum *acc, const unsigned char center, const unsigned char idx, const prgba *pal, char *simcache, unsigned char w) {
    unsigned char sim = similarity(center, idx, pal, simcache);
    if (sim) {
        w *= sim;
        w *= pal[idx].a;
        prgba c = pal[idx];
        acc->r += c.r * w;
        acc->g += c.g * w;
        acc->b += c.b * w;
        acc->a += c.a * w;
        acc->count += w;
    }
}

void undither(const unsigned char *image, const rgba *rgba_pal, const unsigned int width, const unsigned int height, rgba *out) {
    char *simcache = malloc(256*256);
    memset(simcache, -1, 256*256);

    prgba pal[256];
    for(int i=0; i < 256; i++) {
        pal[i] = (prgba){
            .r = rgba_pal[i].r * rgba_pal[i].a,
            .g = rgba_pal[i].g * rgba_pal[i].a,
            .b = rgba_pal[i].b * rgba_pal[i].a,
            .a = rgba_pal[i].a,
        };
    }

    for(int y=0; y < height; y++) {
        for(int x=0; x < width; x++) {
            const int center = image[x+y*width];
            const prgba cpal = pal[center];
            const int center_w = 8 * cpal.a;
            rgba_sum acc = {
                .r = cpal.r * center_w,
                .g = cpal.g * center_w,
                .b = cpal.b * center_w,
                .a = cpal.a * center_w,
                .count = center_w,
            };

            if (y > 0) {
                if (x > 0) add_to_acc(&acc, center, image[(x-1)+(y-1)*width], pal, simcache, 1);
                add_to_acc(&acc, center, image[(x+0)+(y-1)*width], pal, simcache, 2);
                if (x < width-1) add_to_acc(&acc, center, image[(x+1)+(y-1)*width], pal, simcache, 1);
            }

            if (x > 0) add_to_acc(&acc, center, image[(x-1)+(y-0)*width], pal, simcache, 2);
            if (x < width-1) add_to_acc(&acc, center, image[(x+1)+(y-0)*width], pal, simcache, 2);

            if (y < height-1) {
                if (x > 0) add_to_acc(&acc, center, image[(x-1)+(y+1)*width], pal, simcache, 1);
                add_to_acc(&acc, center, image[(x+0)+(y+1)*width], pal, simcache, 2);
                if (x < width-1) add_to_acc(&acc, center, image[(x+1)+(y+1)*width], pal, simcache, 1);
            }

            if (acc.a) {
                out[x+y*width] = (rgba){
                    .r = acc.r / acc.a,
                    .g = acc.g / acc.a,
                    .b = acc.b / acc.a,
                    .a = acc.a / acc.count,
                };
            } else {
                out[x+y*width] = (rgba){0};
            }
        }
    }

    free(simcache);
}
