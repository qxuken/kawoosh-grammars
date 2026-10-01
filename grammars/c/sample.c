#include <stdio.h>
#include <stdlib.h>

#define LIMIT 10

typedef struct point {
    double x;
    double y;
} point_t;

enum colour { RED, GREEN, BLUE };

static int parse(const char *text, int *out) {
    int total = 0;
    for (const char *p = text; *p != '\0'; p++) {
        if (*p < '0' || *p > '9') {
            return -1;
        }
        total = total * 10 + (*p - '0');
    }
    *out = total;
    return 0;
}

int main(int argc, char **argv) {
    point_t p = { .x = 3.0, .y = 4.0 };
    int n = 0;
    /* A comment. */
    if (argc > 1 && parse(argv[1], &n) == 0) {
        printf("%d %f\n", n, p.x * p.y);
    }
    switch (n % 3) {
    case RED:
        break;
    default:
        n += LIMIT;
    }
    return n > 0 ? EXIT_SUCCESS : EXIT_FAILURE;
}
