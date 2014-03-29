
typedef struct rgba {unsigned char r,g,b,a;} rgba;
void undither(const unsigned char *image, const rgba *pal, unsigned int width, unsigned int height, rgba *out);
