use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec; 
#[derive(Debug)]  // attribute on the following struct to auto-generate suitable implementation of the debug trait

struct Dataset{
    bedrooms: f64,
    bathrooms: f64,
    sqft_living: f64,
    sqft_lot: f64,
    floors: f64,
    condition: f64,
    grade: f64,
    sqft_above: f64,
    sqft_basement: f64,
    price: f64
}

impl Dataset {
   pub fn new(raw_vector:Vec<&str>)->Dataset{
        let unwrapp_data: Vec<f64> =raw_vector.iter().map(|r| r.parse().unwrap()).collect(); 
        Dataset{bedrooms: unwrapped_text[3], 
            bathrooms: unwrapped_text[4], 
            sqft_living: unwrapped_text[5],
            sqft_lot: unwrapped_text[6],
            floors: unwrapped_text[7],
            condition: unwrapped_text[10],
            grade: unwrapped_text[11],
            sqft_above: unwrapped_text[12],
            sqft_basement: unwrapped_text[13],
            price: unwrapped_text[2],
           } 
    }
    fn features(&self) -> Vec<f64> {
        vec![self.bedrooms,
        self.bathrooms,
        self.sqft_living,
        self.sqft_lot,
        self.floors,
        self.condition,
        self.grade,
        self.sqft_above,
        self.sqft_basement]
    }
    fn target(&self)->f64{
        self.price
    }
}

pub fn read_row(row:string) -> Dataset{
    let raw_vector: Vec<&str> = row.split_whitespace().collect(); 
    let dataset_vector: Dataset = Dataset::new(raw_vector); 
    dataset_vector
}

pub fn read_csv(filename: impl AsRef<Path>) -> Vec<Dataset> {
    let file = File::open(filename).expect("open file fail");
    // notice how to read a file in rust. The .expect is used in case of error 
    let reader = BufReader::new(file);
    reader.lines().enumerate()
            .map(|row| read_row(row))
            .collect()
    
}