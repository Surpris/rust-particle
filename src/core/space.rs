//! space
//!
//! a 3D universe struct providing mesh grids in the Cartesian coordinates.

use super::utils;
use num_traits::float::Float;
use std::f64::consts::PI;

/// Space
///
/// provide functions which give ranges and grids
/// in real and reciprocal space.
pub struct Space<T>
where
    T: Float,
{
    grid_length: Vec<usize>,
    space_size: Vec<T>,
}

impl<T> Space<T>
where
    T: Float,
{
    /// create a Space instance.
    pub fn new(n: &[usize], l: &[T]) -> Self {
        assert_eq!(n.len(), 3);
        assert_eq!(l.len(), 3);
        let grid_length: Vec<usize> = n.to_vec();
        let space_size: Vec<T> = l.to_vec();
        Self {
            grid_length,
            space_size,
        }
    }

    /// return spatial resolution.
    pub fn spatial_resolution(&self) -> Vec<T> {
        (0..self.grid_length.len())
            .map(|ii: usize| {
                utils::cast_t2u::<f64, T>(2.0) * self.space_size[ii]
                    / utils::cast_t2u::<usize, T>(self.grid_length[ii])
            })
            .collect()
    }

    /// return spatial range in the x axis.
    pub fn spatial_range_x(&self) -> Vec<T> {
        self.spatial_range(0)
    }

    /// return spatial range in the y axis.
    pub fn spatial_range_y(&self) -> Vec<T> {
        self.spatial_range(1)
    }

    /// return spatial range in the z axis.
    pub fn spatial_range_z(&self) -> Vec<T> {
        self.spatial_range(2)
    }

    /// return spatial range specified by an index `ii`.
    fn spatial_range(&self, ii: usize) -> Vec<T> {
        utils::xrange(
            -self.space_size[ii],
            self.space_size[ii],
            self.grid_length[ii],
        )
    }

    /// return 2D spatial grids.
    pub fn spatial_grid_2d(&self) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
        let range_x = self.spatial_range_x();
        let range_y = self.spatial_range_y();
        let xx: Vec<Vec<T>> = (0..range_y.len()).map(|_| range_x.clone()).collect();
        let yy: Vec<Vec<T>> = (0..range_y.len())
            .map(|ii| vec![range_y[ii]; range_x.len()])
            .collect();
        (xx, yy)
    }

    /// return frequency resolution.
    pub fn frequency_resolution(&self) -> Vec<T> {
        (0..self.space_size.len())
            .map(|ii: usize| utils::cast_t2u::<f64, T>(0.5) / self.space_size[ii])
            .collect()
    }

    /// return frequency range in the x axis.
    pub fn frequency_range_x(&self) -> Vec<T> {
        self.frequency_range(0)
    }

    /// return frequency range in the y axis.
    pub fn frequency_range_y(&self) -> Vec<T> {
        self.frequency_range(1)
    }

    /// return frequency range in the z axis.
    pub fn frequency_range_z(&self) -> Vec<T> {
        self.frequency_range(2)
    }

    /// return frequency range specified by an index `ii`.
    fn frequency_range(&self, ii: usize) -> Vec<T> {
        let df = self.frequency_resolution()[ii];
        let imax = utils::cast_t2u::<f64, T>(0.5 * self.grid_length[ii] as f64);
        utils::arange(-df * imax, df * imax, df)
    }

    /// return 2D grids in frequency space.
    pub fn frequency_grid_2d(&self) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
        let range_x = self.frequency_range_x();
        let range_y = self.frequency_range_y();
        let xx: Vec<Vec<T>> = (0..range_y.len()).map(|_| range_x.clone()).collect();
        let yy: Vec<Vec<T>> = (0..range_y.len())
            .map(|ii| vec![range_y[ii]; range_x.len()])
            .collect();
        (xx, yy)
    }

    /// return momentum resolution.
    pub fn momentum_resolution(&self) -> Vec<T> {
        (0..self.space_size.len())
            .map(|ii: usize| utils::cast_t2u::<f64, T>(PI) / self.space_size[ii])
            .collect()
    }

    /// return momentum range in the x axis.
    pub fn momentum_range_x(&self) -> Vec<T> {
        self.momentum_range(0)
    }

    /// return momentum range in the y axis.
    pub fn momentum_range_y(&self) -> Vec<T> {
        self.momentum_range(1)
    }

    /// return momentum range in the z axis.
    pub fn momentum_range_z(&self) -> Vec<T> {
        self.momentum_range(2)
    }

    /// return momentum range specified by an index `ii`.
    fn momentum_range(&self, ii: usize) -> Vec<T> {
        let dq = self.momentum_resolution()[ii];
        let imax = utils::cast_t2u::<f64, T>(0.5 * self.grid_length[ii] as f64);
        utils::arange(-dq * imax, dq * imax, dq)
    }

    /// return 2D grids in momentum space.
    pub fn momentum_grid_2d(&self) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
        let range_x = self.momentum_range_x();
        let range_y = self.momentum_range_y();
        let xx: Vec<Vec<T>> = (0..range_y.len()).map(|_| range_x.clone()).collect();
        let yy: Vec<Vec<T>> = (0..range_y.len())
            .map(|ii| vec![range_y[ii]; range_x.len()])
            .collect();
        (xx, yy)
    }
}
