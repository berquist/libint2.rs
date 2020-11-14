#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("libint2/include/wrapper.h");
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
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
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
        ffi::libint2_finalize();
    }
}
