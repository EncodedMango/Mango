mod dataset;

use raster::Color;
use raster::Image;
use ndarray::*;
use crate::dataset::Dataset;


fn main() {
    //let dense1: LayerDense = LayerDense::new(2, 3);
    let mnist = Dataset::load_dataset("Fashion_MNIST/train");
    println!("{}", mnist.x.mean().unwrap());

    let mut img: Image = Image::blank(512, 512);
    raster::editor::fill(&mut img, Color::white()).expect("oof, that didn't work");

    //raster::save(&img, "out.png").expect("can't save");
}
