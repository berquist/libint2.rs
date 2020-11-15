#ifdef __cplusplus
/* #define EXTERNC extern "C" */
/* #else */
/* #define EXTERNC */
extern "C" {
#endif

void calc_f(double *F, double T, unsigned int max_m);

/* #undef EXTERNC */
#ifdef __cplusplus
}
#endif
