// #include <catch2/catch.hpp>
#include <libint2/engine.h>
#include "libint2/include/libint2_c.h"
#include "libint2/include/libint2_wrapper.hpp"

void libint2_init()
{
    libint2::initialize();
}

void libint2_finalize()
{
    libint2::finalize();
}

libint2::Operator convert_operator_rust_to_cxx(LibintOperator operator_rust) {
    libint2::Operator op;
    switch(operator_rust) {
    case LibintOperator::Overlap:
        op = libint2::Operator::overlap;
        break;
    case LibintOperator::Kinetic:
        op = libint2::Operator::kinetic;
        break;
    case LibintOperator::Nuclear:
        op = libint2::Operator::nuclear;
        break;
    case LibintOperator::ErfNuclear:
        op = libint2::Operator::erf_nuclear;
        break;
    case LibintOperator::ErfcNuclear:
        op = libint2::Operator::erfc_nuclear;
        break;
    case LibintOperator::EMultipole1:
        op = libint2::Operator::emultipole1;
        break;
    case LibintOperator::EMultipole2:
        op = libint2::Operator::emultipole2;
        break;
    case LibintOperator::EMultipole3:
        op = libint2::Operator::emultipole3;
        break;
    case LibintOperator::SphEMultipole:
        op = libint2::Operator::sphemultipole;
        break;
    case LibintOperator::Delta:
        op = libint2::Operator::delta;
        break;
    case LibintOperator::Coulomb:
        op = libint2::Operator::coulomb;
        break;
    case LibintOperator::Cgtg:
        op = libint2::Operator::cgtg;
        break;
    case LibintOperator::CgtgTimesCoulomb:
        op = libint2::Operator::cgtg_x_coulomb;
        break;
    case LibintOperator::DelCgtgSquared:
        op = libint2::Operator::delcgtg2;
        break;
    case LibintOperator::R12:
        op = libint2::Operator::r12;
        break;
    case LibintOperator::ErfCoulomb:
        op = libint2::Operator::erf_coulomb;
        break;
    case LibintOperator::ErfcCoulomb:
        op = libint2::Operator::erfc_coulomb;
        break;
    case LibintOperator::Stg:
        op = libint2::Operator::stg;
        break;
    case LibintOperator::StgTimesCoulomb:
        op = libint2::Operator::stg_x_coulomb;
        break;
    default:
        throw "operator not implemented";
    }
    return op;
}

// libint2::Contraction convert_contraction_rust_to_cxx(LibintContraction &contraction_rust) {
//     std::vector<double> coeff;
//     for (auto& it : contraction_rust.coeff) {
//         coeff.push_back(it);
//     }
//     return Contraction(contraction_rust.l, contraction_rust.pure);
// }

// libint2::Shell convert_shell_rust_to_cxx(LibintShell &shell_rust) {
//     std::vector<double> alpha;
//     std::array<double, 3> origin;
//     // return libint2::Shell
// }

Libint_t erieval;
double* F;

void init_c_api(unsigned int max_am) {
    libint2_init_eri(&erieval, max_am, 0);
    F = (double*)malloc(sizeof(double)*(4*max_am+1));
#if LIBINT_CONTRACTED_INTS
    /* if have support for contracted integrals, set the contraction length to 1 */
    erieval.contrdepth = 1;
#endif
}

void finalize_c_api() {
    free(F);
    libint2_cleanup_eri(&erieval);
}

/** This function evaluates ERI over 4 primitive Gaussian shells.
    See tests/eri/test.cc for an example of how to deal with
    contracted Gaussians.

    For simplicity, many details are omitted here, e.g. normalization.
*/
void
_compute_eri(Libint_t* erieval,
             unsigned int am1, double alpha1, const double* A,
             unsigned int am2, double alpha2, const double* B,
             unsigned int am3, double alpha3, const double* C,
             unsigned int am4, double alpha4, const double* D
    )
{
    /* I will assume that libint2_static_init() and libint2_init_eri(&erieval,max_am,0) had been called elsewhere! */

    double gammap, Px, Py, Pz, PAx, PAy, PAz, PBx, PBy, PBz, AB2;
    double gammaq, Qx, Qy, Qz, QCx, QCy, QCz, QDx, QDy, QDz, CD2;
    double gammapq, PQx, PQy, PQz, PQ2, Wx, Wy, Wz;
    double K1, K2, pfac;
    unsigned int am;

    /*
      Compute requisite data -- many of these quantities would be precomputed
      for all nonnegligible shell pairs somewhere else
    */
    gammap = alpha1 + alpha2;
    Px = (alpha1*A[0] + alpha2*B[0])/gammap;
    Py = (alpha1*A[1] + alpha2*B[1])/gammap;
    Pz = (alpha1*A[2] + alpha2*B[2])/gammap;
    PAx = Px - A[0];
    PAy = Py - A[1];
    PAz = Pz - A[2];
    PBx = Px - B[0];
    PBy = Py - B[1];
    PBz = Pz - B[2];
    AB2 = (A[0]-B[0])*(A[0]-B[0])
        + (A[1]-B[1])*(A[1]-B[1])
        + (A[2]-B[2])*(A[2]-B[2]);

    erieval->PA_x[0] = PAx;
    erieval->PA_y[0] = PAy;
    erieval->PA_z[0] = PAz;
    erieval->AB_x[0] = A[0] - B[0];
    erieval->AB_y[0] = A[1] - B[1];
    erieval->AB_z[0] = A[2] - B[2];
    erieval->oo2z[0] = 0.5/gammap;

    gammaq = alpha3 + alpha4;
    gammapq = gammap*gammaq/(gammap+gammaq);
    Qx = (alpha3*C[0] + alpha4*D[0])/gammaq;
    Qy = (alpha3*C[1] + alpha4*D[1])/gammaq;
    Qz = (alpha3*C[2] + alpha4*D[2])/gammaq;
    QCx = Qx - C[0];
    QCy = Qy - C[1];
    QCz = Qz - C[2];
    QDx = Qx - D[0];
    QDy = Qy - D[1];
    QDz = Qz - D[2];
    CD2 = (C[0]-D[0])*(C[0]-D[0])
        + (C[1]-D[1])*(C[1]-D[1])
        + (C[2]-D[2])*(C[2]-D[2]);

    erieval->QC_x[0] = QCx;
    erieval->QC_y[0] = QCy;
    erieval->QC_z[0] = QCz;
    erieval->CD_x[0] = C[0] - D[0];
    erieval->CD_y[0] = C[1] - D[1];
    erieval->CD_z[0] = C[2] - D[2];
    erieval->oo2e[0] = 0.5/gammaq;

    PQx = Px - Qx;
    PQy = Py - Qy;
    PQz = Pz - Qz;
    PQ2 = PQx*PQx + PQy*PQy + PQz*PQz;
    Wx = (gammap*Px + gammaq*Qx)/(gammap+gammaq);
    Wy = (gammap*Py + gammaq*Qy)/(gammap+gammaq);
    Wz = (gammap*Pz + gammaq*Qz)/(gammap+gammaq);

    erieval->WP_x[0] = Wx - Px;
    erieval->WP_y[0] = Wy - Py;
    erieval->WP_z[0] = Wz - Pz;
    erieval->WQ_x[0] = Wx - Qx;
    erieval->WQ_y[0] = Wy - Qy;
    erieval->WQ_z[0] = Wz - Qz;
    erieval->oo2ze[0] = 0.5/(gammap+gammaq);
    erieval->roz[0] = gammapq/gammap;
    erieval->roe[0] = gammapq/gammaq;

    K1 = exp(-alpha1*alpha2*AB2/gammap);
    K2 = exp(-alpha3*alpha4*CD2/gammaq);
    pfac = 2*pow(M_PI,2.5)*K1*K2/(gammap*gammaq*sqrt(gammap+gammaq));

    /*
      evaluate Boys function F_m for all m in [0,am]
    */
    am = am1 + am2 + am3 + am4;
    calc_f(F, PQ2*gammapq, am);

    /* (00|00)^m = pfac * F_m */
    assert(am <= 4);
    erieval->LIBINT_T_SS_EREP_SS(0)[0] = pfac*F[0];
    erieval->LIBINT_T_SS_EREP_SS(1)[0] = pfac*F[1];
    erieval->LIBINT_T_SS_EREP_SS(2)[0] = pfac*F[2];
    erieval->LIBINT_T_SS_EREP_SS(3)[0] = pfac*F[3];
    erieval->LIBINT_T_SS_EREP_SS(4)[0] = pfac*F[4];

    /* compute ERIs */
    libint2_build_eri[am1][am2][am3][am4](erieval);

}

double*
compute_eri(unsigned int am1, double alpha1, const double* A,
             unsigned int am2, double alpha2, const double* B,
             unsigned int am3, double alpha3, const double* C,
             unsigned int am4, double alpha4, const double* D) {
  _compute_eri(&erieval, am1, alpha1, A, am2, alpha2, B, am3, alpha3, C, am4, alpha4, D);
  return erieval.targets[0];
}



void libint2_test_c_api(int am1, int am2, int am3, int am4, double alpha1, double alpha2, double alpha3, double alpha4, const rust::Vec<double> &A, const rust::Vec<double> &B, const rust::Vec<double> &C, const rust::Vec<double> &D) {
    using std::max;
    auto max_am = max(max(am1,am2),max(am3,am4));
    init_c_api(max_am);

    auto* c_result = compute_eri(am1, alpha1, A.data(), am2, alpha2, B.data(), am3, alpha3, C.data(), am4, alpha4, D.data());

    const double* cpp_result;
    using libint2::Shell;
    Shell sh1{{alpha1}, {{am1, false, {1.0}}}, {A[0], A[1], A[2]}};
    Shell sh2{{alpha2}, {{am2, false, {1.0}}}, {B[0], B[1], B[2]}};
    Shell sh3{{alpha3}, {{am3, false, {1.0}}}, {C[0], C[1], C[2]}};
    Shell sh4{{alpha4}, {{am4, false, {1.0}}}, {D[0], D[1], D[2]}};
    libint2::Engine engine(libint2::Operator::coulomb, 1, max_am);
    engine.compute(sh1, sh2, sh3, sh4);
    cpp_result = engine.results()[0];

    unsigned int n1, n2, n3, n4;
    unsigned int a, b, c, d, abcd;
    n1 = (am1 + 1) * (am1 + 2)/2;
    n2 = (am2 + 1) * (am2 + 2)/2;
    n3 = (am3 + 1) * (am3 + 2)/2;
    n4 = (am4 + 1) * (am4 + 2)/2;
    const auto norm_factor = sh1.contr[0].coeff[0] * sh2.contr[0].coeff[0] * sh3.contr[0].coeff[0] * sh4.contr[0].coeff[0];
    for(a=0, abcd=0; a<n1; a++) {
        for(b=0; b<n2; b++) {
            for(c=0; c<n3; c++) {
                for(d=0; d<n4; d++, ++abcd) {
                    printf("a = %d b = %d c = %d d = %d (ab|cd) = %20.15lf , ref (ab|cd) = %20.15lf\n", a, b, c, d, c_result[abcd]*norm_factor, cpp_result[abcd]);
                    // REQUIRE(c_result[abcd]*norm_factor == Approx(cpp_result[abcd]));
                    // printf("a = %d b = %d c = %d d = %d ref (ab|cd) = %20.15lf\n", a, b, c, d, cpp_result[abcd]);
                }
            }
        }
    }

    std::cout << "n1: " << n1 << std::endl;
    std::cout << "n2: " << n2 << std::endl;
    std::cout << "n3: " << n3 << std::endl;
    std::cout << "n4: " << n4 << std::endl;
    std::cout << "abcd: " << abcd << std::endl;

    finalize_c_api();
}

double libint2_calc_boys_reference_single(double T, unsigned int m) {
    const auto instance = libint2::FmEval_Reference<double>();
    return instance.eval(T, m);
}

std::vector<double> libint2_calc_boys_chebyshev7(double T, unsigned int max_m) {
    std::vector<double> ret(max_m + 1);
    const auto instance = libint2::FmEval_Chebyshev7<double>::instance(max_m);
    instance->eval(ret.data(), T, max_m);
    return ret;
}

// FIXME
// std::unique_ptr<LibintEngine> libint2_create_engine(LibintOperator op_rust, size_t max_nprim, int max_l) {
//     const auto op_libint = convert_operator_rust_to_cxx(op_rust);
//     auto engine = Engine(op_libint, max_nprim, max_l, 0);
//     return std::make_unique<LibintEngine>(std::move(engine));
// }
