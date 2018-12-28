pub struct Config;

const DEFAULT_MULTITHREADING_FACTORS: MultithreadingFactors = MultithreadingFactors {
    linear_horizontal_lifting: 128 * 128,
    linear_vertical_lifting: 128 * 128,
    cubic_horizontal_lifting: 64 * 64,
    cubic_vertical_lifting: 64 * 64,
    quantization: 96 * 96,
    compress: 128 * 128,
};

pub struct MultithreadingFactors {
    pub linear_horizontal_lifting: usize,
    pub linear_vertical_lifting: usize,
    pub cubic_horizontal_lifting: usize,
    pub cubic_vertical_lifting: usize,
    pub quantization: usize,
    pub compress: usize,
}

impl Config {
    pub fn multithreading_factors() -> &'static MultithreadingFactors {
        &DEFAULT_MULTITHREADING_FACTORS
    }
}
