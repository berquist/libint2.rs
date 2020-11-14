#include <vector>
#include <libint2.hpp>
#include "cxx.h"

void libint2_init();
void libint2_finalize();
void libint2_test_c_api(int am1, int am2, int am3, int am4, double alpha1, double alpha2, double alpha3, double alpha4, const rust::Vec<double> &A, const rust::Vec<double> &B, const rust::Vec<double> &C, const rust::Vec<double> &D);
