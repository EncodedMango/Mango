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

        let mut ind = 0;
        for file in WalkDir::new(fp).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                let path: &str = file.path().to_str().unwrap();
                let split: Vec<&str> = path.split("/").collect();
                let class: usize = split[split.len() - 2].parse::<usize>().unwrap();

                let img: Image = raster::open(path).unwrap();

                for i in 0..img.bytes.len() {
                    x[[ind, i]] = img.bytes[i] as f64;
                }

                y[[ind]] = class;
                ind += 1;
            }
        }

        //shuffle data
        let mut vec: Vec<usize> = (0..x.shape()[0]).collect();
        vec.shuffle(&mut thread_rng());

        for (i, j) in zip(vec, 0..x.shape() [0]) {
            for k in 0..784 {
                x[[j, k]] = x[[i, k]]
            }
            y[[j]] = y [[i]]
        }

        return Dataset {x, y}
    }
}