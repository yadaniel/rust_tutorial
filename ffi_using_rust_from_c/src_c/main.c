#include <stdio.h>
#include "main.h"

int main() {
    printf("1 + 2 = %i\n", called_by_c(1,2));

    struct X x = {.x0=0, .x1=1, .x2=2, .x3=3};
    print_x(&x);

    return 0;
}

