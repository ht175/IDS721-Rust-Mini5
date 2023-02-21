// usual imports
use rusty_machine;
use rusty_machine::analysis::score::neg_mean_squared_error;
use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::BaseMatrix;
use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec;
use std::vec::Vec;

// #[derive(Debug)]  // attribute on the following struct to auto-generate suitable implementation of the debug trait

pub struct Dataset {
    bedrooms: f64,
    bathrooms: f64,
    sqft_living: f64,
    sqft_lot: f64,
    floors: f64,
    condition: f64,
    grade: f64,
    sqft_above: f64,
    sqft_basement: f64,
    price: f64,
}
impl Dataset {
    pub fn new(raw_vector: Vec<&str>) -> Dataset {
        let unwrapp_data: Vec<f64> = raw_vector.iter().map(|r| r.parse().unwrap()).collect();
        Dataset {
            bedrooms: unwrapp_data[0],
            bathrooms: unwrapp_data[1],
            sqft_living: unwrapp_data[2],
            sqft_lot: unwrapp_data[3],
            floors: unwrapp_data[4],
            condition: unwrapp_data[5],
            grade: unwrapp_data[6],
            sqft_above: unwrapp_data[7],
            sqft_basement: unwrapp_data[8],
            price: unwrapp_data[9],
        }
    }
    fn features(&self) -> Vec<f64> {
        vec![
            self.bedrooms,
            self.bathrooms,
            self.sqft_living,
            self.sqft_lot,
            self.floors,
            self.condition,
            self.grade,
            self.sqft_above,
            self.sqft_basement,
        ]
    }
    fn target(&self) -> f64 {
        self.price
    }
}

fn read_row(row: String) -> Dataset {
    let  raw_vector: Vec<&str> = row.split_terminator(',').collect();
    println!("{row}");
    let data_vec= vec![raw_vector[3],raw_vector[4],raw_vector[5],raw_vector[6],raw_vector[7],raw_vector[10],raw_vector[11],raw_vector[12],raw_vector[13],raw_vector[2]];
    let dataset_vector: Dataset = Dataset::new(data_vec);
    dataset_vector
}

pub fn read_csv(filename: impl AsRef<Path>) -> Vec<Dataset> {
    let file = File::open(filename).expect("open file fail");
    // notice how to read a file in rust. The .expect is used in case of error
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .map(|(numb, line)| line.expect(&format!("Impossible to read line number {}", numb)))
        .map(|row| read_row(row))
        .collect()
}
pub fn regression_analysis() -> f64 {
    let filepath = "house_price.csv";
    let mut raw_data = read_csv(&filepath);
    let test_size: f64 = raw_data.len() as f64 * 0.4;
    let test_size = test_size.round() as usize;
    //spilt data
    let (test, train) = raw_data.split_at(test_size);
    let train_size = train.len();
    let test_size = test.len();
    let x_train: Vec<f64> = train.iter().flat_map(|row| row.features()).collect();
    let y_train: Vec<f64> = train.iter().map(|row| row.target()).collect();
    let x_test: Vec<f64> = test.iter().flat_map(|row| row.features()).collect();
    let y_test: Vec<f64> = test.iter().map(|row| row.target()).collect();
    //transfer train to matrix for model trainning
    let x_train_matrix = Matrix::new(train_size, 9, x_train);
    let y_train_vector = Vector::new(y_train);
    let x_test_matrix = Matrix::new(test_size, 9, x_test);
    //begin to train model

    let mut linearRegression = LinRegressor::default();
    println!("data trainning begin");
    linearRegression.train(&x_train_matrix, &y_train_vector);
    //do prediction
    println!("prediction begin");
    let prediction = linearRegression.predict(&x_test_matrix).unwrap();
    //to calcute  error rate
    //first need to tranfer prediction value and real value to matrix
    let prediction_matrix = Matrix::new(test_size, 1, prediction);
    let real_matrix = Matrix::new(test_size, 1, y_test);
    let mean_sqr_erro = neg_mean_squared_error(&prediction_matrix, &real_matrix);
    println!("Final mean squared error is  {:?}", mean_sqr_erro);
    mean_sqr_erro
}
