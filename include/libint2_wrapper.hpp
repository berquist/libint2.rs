#ifndef _LIBINT2_WRAPPER_HPP
#define _LIBINT2_WRAPPER_HPP

#include <memory>
#include <libint2/engine.h>
#include "libint2/src/lib.rs.h"
#include "rust/cxx.h"

void libint2_init();
void libint2_finalize();
void libint2_test_c_api(int am1, int am2, int am3, int am4, double alpha1, double alpha2, double alpha3, double alpha4, const rust::Vec<double> &A, const rust::Vec<double> &B, const rust::Vec<double> &C, const rust::Vec<double> &D);

// FIXME
// using LibintEngine = libint2::Engine;
// std::unique_ptr<LibintEngine> libint2_create_engine(LibintOperator op_rust, size_t max_nprim, int max_l);

#endif // _LIBINT2_WRAPPER_HPP
