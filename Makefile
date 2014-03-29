CFLAGS ?= -O3 -std=c99 -g
OBJS = lodepng.o undither.o main.o

undither: $(OBJS)
	$(CC) $(LDFLAGS) $^ -o $@

main.c: lodepng.h
lodepng.c: lodepng.h

lodepng.c:
	curl -L -o lodepng.c 'http://lpi.googlecode.com/svn/trunk/lodepng.cpp'

lodepng.h:
	curl -L -o lodepng.h 'http://lpi.googlecode.com/svn/trunk/lodepng.h'

clean:
	rm -rf undither lodepng.[ch] $(OBJS)
