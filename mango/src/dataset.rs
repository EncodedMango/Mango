use std::iter::zip;
use ndarray::*;
use raster::Image;
use walkdir::WalkDir;
use rand::*;
use rand::seq::SliceRandom;

pub struct Dataset {pub x: Array2<f64>, pub y: Array1<usize>}

impl Dataset {
    pub fn load_dataset(fp: &str) -> Dataset {
        let samples = 6000;
        let classes = 10;

        let mut x: Array2<f64> = Array2::default((classes * samples, 784));
        let mut y: Array1<usize> = Array1::default((classes * samples));

        return Dataset {x, y}
    }
}