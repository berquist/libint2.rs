#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("libint2/include/wrapper.h");
        fn libint2_init();
        fn libint2_finalize();
        // fn create_simple_primitive_shell(alpha: f64, am: i64, px: f64, py: f64, pz: f64) -> Shell;
        fn do_thing();
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
    // use std::cmp;

    #[test]
    fn it_works() {
        ffi::libint2_init();
        let am1 = 1;
        let am2 = 1;
        let am3 = 1;
        let am4 = 1;
        let alpha1 = 1.1;
        let alpha2 = 2.3;
        let alpha3 = 3.4;
        let alpha4 = 4.8;
        let A = [0.0, 1.0, 2.0];
        let B = [1.0, 2.0, 0.0];
        let C = [2.0, 0.0, 1.0];
        let D = [0.0, 1.0, 2.0];
        // let max_am = cmp::max(cmp::max(am1, am2), cmp::max(am3, am4));
        // let mut erieval
        ffi::do_thing();
        ffi::libint2_finalize();
    }
}
