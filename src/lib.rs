use std::f64::consts::PI;

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]
mod ffi_bindgen {
    include!(concat!(env!("OUT_DIR"), "/bindings_libint2.rs"));
}

#[allow(dead_code, unused_variables)]
fn compute_eri(
    am1: i32,
    am2: i32,
    am3: i32,
    am4: i32,
    alpha1: f64,
    alpha2: f64,
    alpha3: f64,
    alpha4: f64,
    ra: &[f64; 3],
    rb: &[f64; 3],
    rc: &[f64; 3],
    rd: &[f64; 3],
) -> f64 {
    let gammap = alpha1 + alpha2;
    let px = (alpha1 * ra[0] + alpha2 * rb[0]) / gammap;
    let py = (alpha1 * ra[1] + alpha2 * rb[1]) / gammap;
    let pz = (alpha1 * ra[2] + alpha2 * rb[2]) / gammap;
    let pax = px - ra[0];
    let pay = py - ra[1];
    let paz = pz - ra[2];
    let pbx = px - rb[0];
    let pby = py - rb[1];
    let pbz = pz - rb[2];
    let ab2 = (ra[0] - rb[0]) * (ra[0] - rb[0])
        + (ra[1] - rb[1]) * (ra[1] - rb[1])
        + (ra[2] - rb[2]) * (ra[2] - rb[2]);

    let gammaq = alpha3 + alpha4;
    let gammapq = gammap * gammaq / (gammap + gammaq);
    let qx = (alpha3 * rc[0] + alpha4 * rd[0]) / gammaq;
    let qy = (alpha3 * rc[1] + alpha4 * rd[1]) / gammaq;
    let qz = (alpha3 * rc[2] + alpha4 * rd[2]) / gammaq;
    let qcx = qx - rc[0];
    let qcy = qy - rc[1];
    let qcz = qz - rc[2];
    let qdx = qx - rd[0];
    let qdy = qy - rd[1];
    let qdz = qz - rd[2];
    let cd2 = (rc[0] - rd[0]) * (rc[0] - rd[0])
        + (rc[1] - rd[1]) * (rc[1] - rd[1])
        + (rc[2] - rd[2]) * (rc[2] - rd[2]);

    let pqx = px - qx;
    let pqy = py - qy;
    let pqz = pz - qz;
    let pq2 = pqx * pqx + pqy * pqy + pqz * pqz;
    let wx = (gammap * px + gammaq * qx) / (gammap + gammaq);
    let wy = (gammap * py + gammaq * qy) / (gammap + gammaq);
    let wz = (gammap * pz + gammaq * qz) / (gammap + gammaq);

    let k1 = (-alpha1 * alpha2 * ab2 / gammap).exp();
    let k2 = (-alpha3 * alpha4 * cd2 / gammaq).exp();
    let pfac = 2.0 * PI.powf(2.5) * k1 * k2 / (gammap * gammaq * (gammap + gammaq).sqrt());

    let am = am1 + am2 + am3 + am4;
    // TODO Boys

    let erieval = ffi_bindgen::Libint_t {
        PA_x: [pax],
        PA_y: [pay],
        PA_z: [paz],
        PB_x: [pbx],
        PB_y: [pby],
        PB_z: [pbz],
        AB_x: [ra[0] - rb[0]],
        AB_y: [ra[1] - rb[1]],
        AB_z: [ra[2] - rb[2]],
        oo2z: [0.5 / gammap],
        QC_x: [qcx],
        QC_y: [qcy],
        QC_z: [qcz],
        CD_x: [rc[0] - rd[0]],
        CD_y: [rc[1] - rd[1]],
        CD_z: [rc[2] - rd[2]],
        oo2e: [0.5 / gammaq],
        WP_x: [wx - px],
        WP_y: [wy - py],
        WP_z: [wz - pz],
        WQ_x: [wx - qx],
        WQ_y: [wy - qy],
        WQ_z: [wz - qz],
        oo2ze: [0.5 / (gammap + gammaq)],
        roz: [gammapq / gammap],
        roe: [gammapq / gammaq],
        contrdepth: 1,
        stack: unsafe { std::mem::zeroed() },
        vstack: unsafe { std::mem::zeroed() },
        targets: unsafe { std::mem::zeroed() },
        veclen: ffi_bindgen::LIBINT2_MAX_VECLEN as i32,
        ..Default::default()
    };

    let f = unsafe { ffi_bindgen::libint2_build_eri }[am1 as usize][am2 as usize][am3 as usize]
        [am4 as usize]
        .unwrap();
    let erieval_ref = &erieval;
    let f2 = unsafe { f(erieval_ref) };

    0.0
}

#[cxx::bridge]
mod ffi {
    struct LibintContraction {
        l: i32,
        pure: bool,
        coeff: Vec<f64>,
    }

    struct LibintShell {
        alpha: Vec<f64>,
        contr: Vec<LibintContraction>,
        // origin: [f64; 3],
        origin: Vec<f64>,
    }

    enum LibintOperator {
        Overlap,
        Kinetic,
        Nuclear,
        ErfNuclear,
        ErfcNuclear,
        EMultipole1,
        EMultipole2,
        EMultipole3,
        SphEMultipole,
        Delta,
        Coulomb,
        Cgtg,
        CgtgTimesCoulomb,
        DelCgtgSquared,
        R12,
        ErfCoulomb,
        ErfcCoulomb,
        Stg,
        StgTimesCoulomb,
    }

    unsafe extern "C++" {
        include!("/usr/include/libint2/engine.h");

        #[namespace = "libint2"]
        type Engine;

        include!("libint2/include/libint2_wrapper.hpp");

        fn libint2_init();
        fn libint2_finalize();
        fn libint2_test_c_api(
            am1: i32,
            am2: i32,
            am3: i32,
            am4: i32,
            alpha1: f64,
            alpha2: f64,
            alpha3: f64,
            alpha4: f64,
            A: &Vec<f64>,
            B: &Vec<f64>,
            C: &Vec<f64>,
            D: &Vec<f64>,
        );

        fn libint2_calc_boys_reference_single(T: f64, m: usize) -> f64;
        fn libint2_calc_boys_chebyshev7(T: f64, max_m: usize) -> UniquePtr<CxxVector<f64>>;

        fn libint2_create_engine(
            op: LibintOperator,
            max_nprim: usize,
            max_l: i32,
        ) -> UniquePtr<Engine>;
    }
}

// fn create_engine(oper: Operator, max_nprim: u64, max_l: i32) -> Engine {}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::cmp;

    #[test]
    fn test_c_api() {
        //! A partial translation of the test at
        //! https://github.com/evaleev/libint/blob/3bf3a07b58650fe2ed4cd3dc6517d741562e1249/tests/unit/test-c-api.cc#L23.
        ffi::libint2_init();
        let am1 = 1;
        let am2 = 1;
        let am3 = 1;
        let am4 = 1;
        let alpha1 = 1.1;
        let alpha2 = 2.3;
        let alpha3 = 3.4;
        let alpha4 = 4.8;
        let A = vec![0.0, 1.0, 2.0];
        let B = vec![1.0, 2.0, 0.0];
        let C = vec![2.0, 0.0, 1.0];
        let D = vec![0.0, 1.0, 2.0];
        ffi::libint2_test_c_api(
            am1, am2, am3, am4, alpha1, alpha2, alpha3, alpha4, &A, &B, &C, &D,
        );
        // let sh1 = ffi::LibintShell {
        //     alpha: vec![alpha1],
        //     contr: vec![ffi::LibintContraction {
        //         l: am1,
        //         pure: false,
        //         coeff: vec![1.0],
        //     }],
        //     origin: vec![0.0, 1.0, 2.0],
        // };
        // let sh2 = ffi::LibintShell {
        //     alpha: vec![alpha2],
        //     contr: vec![ffi::LibintContraction {
        //         l: am2,
        //         pure: false,
        //         coeff: vec![1.0],
        //     }],
        //     origin: vec![1.0, 2.0, 0.0],
        // };
        // let sh3 = ffi::LibintShell {
        //     alpha: vec![alpha3],
        //     contr: vec![ffi::LibintContraction {
        //         l: am3,
        //         pure: false,
        //         coeff: vec![1.0],
        //     }],
        //     origin: vec![2.0, 0.0, 1.0],
        // };
        // let sh4 = ffi::LibintShell {
        //     alpha: vec![alpha4],
        //     contr: vec![ffi::LibintContraction {
        //         l: am4,
        //         pure: false,
        //         coeff: vec![1.0],
        //     }],
        //     origin: vec![0.0, 1.0, 2.0],
        // };
        // let shls = vec![sh1, sh2, sh3, sh4];
        // let max_am = cmp::max(cmp::max(am1, am2), cmp::max(am3, am4));

        // let engine = create_engine(Operator::Coulomb, 1, max_am);
        // let result = engine.compute(&shls);
        ffi::libint2_finalize();
    }

    #[test]
    fn test_calc_boys() {
        println!("{}", ffi::libint2_calc_boys_reference_single(2.0, 2));
        println!(
            "{:#?}",
            ffi::libint2_calc_boys_chebyshev7(2.0, 2)
                .iter()
                .collect::<Vec<_>>()
        );
    }
}
