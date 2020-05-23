//! integration_test
//!
//! test functions in this crate.

macro_rules! assert_eq_precision {
    ( $ left : expr , $ right : expr, $ precision : expr ) => {{
        match (&($left), &($right), &($precision)) {
            (left_val, right_val, precision_val) => {
                if (*left_val - *right_val > *precision_val
                    || *right_val - *left_val > *precision_val)
                {
                    panic!(
                        "assertion failed: `(|left - right| < precision)` \
                                   (left: `{:?}`, right: `{:?}`,precision: `{:?}`)",
                        *left_val, *right_val, *precision_val
                    )
                }
            }
        }
    }};
}

extern crate rust_particle as rp;

const PRECISION: f32 = 0.001;
use std::f32::consts::PI;

#[test]
fn test_space() {
    let grid_length: Vec<usize> = vec![500; 3];
    let size: Vec<f32> = vec![100.0; 3];
    let space = rp::core::space::Space::new(&grid_length, &size);

    // real space
    assert_eq!(space.spatial_resolution(), [0.4, 0.4, 0.4]);
    assert_eq!(space.spatial_range_x()[0], -100.0);
    assert_eq_precision!(space.spatial_range_x()[grid_length[0] - 1], 99.6, PRECISION);

    // frequency space
    let freq_res: Vec<f32> = (0..grid_length.len()).map(|ii| 0.5 / size[ii]).collect();
    for ii in 0..freq_res.len() {
        assert_eq_precision!(space.frequency_resolution()[ii], freq_res[ii], PRECISION);
    }

    assert_eq_precision!(
        space.frequency_range_x()[0],
        -0.5 * grid_length[0] as f32 * freq_res[0],
        PRECISION
    );
    assert_eq_precision!(
        space.frequency_range_x()[grid_length[0] - 1],
        0.5 * grid_length[0] as f32 * freq_res[0] - freq_res[0],
        PRECISION
    );
    // assert_eq_precision!(
    //     space.frequency_range_x()[grid_length[0] - 1],
    //     1.0,
    //     PRECISION
    // );

    // momentum space
    let mom_res: Vec<f32> = (0..grid_length.len()).map(|ii| PI / size[ii]).collect();
    for ii in 0..freq_res.len() {
        assert_eq_precision!(space.momentum_resolution()[ii], mom_res[ii], PRECISION);
    }

    assert_eq_precision!(
        space.momentum_range_x()[0],
        - 0.5 * grid_length[0] as f32 * mom_res[0],
        PRECISION
    );
    assert_eq_precision!(
        space.momentum_range_x()[grid_length[0] - 1],
        0.5 * grid_length[0] as f32 * mom_res[0] - mom_res[0],
        PRECISION
    );
}
