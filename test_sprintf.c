#include <stdio.h>

int main() {
    char buf[64];
    float f = 12.0;
    sprintf(buf, "%f", (double)f);
    printf("Result: %s\n", buf);
    return 0;
}
