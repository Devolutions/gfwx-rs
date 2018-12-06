use std::{env, error::Error, fmt};

fn main() -> Result<(), Box<dyn Error>> {
    let (file1, file2) = if env::args().count() == 3 {
        (env::args().nth(1).unwrap(), env::args().nth(2).unwrap())
    } else {
        panic!("Usage: {} image1 image2", env::args().nth(0).unwrap());
    };

    let image1 = image::open(&file1)?.raw_pixels();
    let image2 = image::open(&file2)?.raw_pixels();

    if image1.len() == image2.len() && image1 == image2 {
        Ok(())
    } else {
        Err(Box::new(DifferentImages(file1, file2)))
    }
}

#[derive(Debug)]
struct DifferentImages(String, String);

impl fmt::Display for DifferentImages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Images {} and {} are different", self.0, self.1)
    }
}

impl Error for DifferentImages {
    fn description(&self) -> &str {
        "Passed images has different content"
    }
}
