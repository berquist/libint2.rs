#include <libint2/boys.h>

#ifdef __cplusplus
extern "C" {
#endif

void calc_f(double *F, double T, unsigned int max_m) {
    libint2::FmEval_Chebyshev7<double>::instance(max_m)->eval(F, T, max_m);
}

#ifdef __cplusplus
}
#endif
