#include <cstdlib>
#include <cstdio>
#include "libint2_c.h"

int main() {
    size_t dim = 5;
    double * F = (double*)malloc(dim * sizeof(double));
    printf("before:\n");
    for (size_t i = 0; i < dim; i++) {
        printf("%lf\n", F[i]);
    }
    unsigned am = 2;
    double T = 0.5;
    calc_f(F, T, am);
    printf("after:\n");
    for (size_t i = 0; i < dim; i++) {
        printf("%lf\n", F[i]);
    }
    free(F);
    return 0;
}
