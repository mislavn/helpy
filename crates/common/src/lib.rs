pub mod error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculatorInput {
    pub array1: Vec<f32>,
    pub array2: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculatorResponse {
    pub array: Vec<f32>,
    pub operation_counter: usize,
}

#[derive(Clone, Debug)]
pub struct Calculator {
    pub operation_counter: Arc<RwLock<usize>>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            operation_counter: Arc::new(RwLock::new(0)),
        }
    }
    pub fn counter(&self) -> Result<usize, error::CommonError> {
        Ok(*self.operation_counter.read()?)
    }

    pub fn sum_array(
        &self,
        input: CalculatorInput,
    ) -> Result<CalculatorResponse, error::CommonError> {
        if input.array1.len() != input.array2.len() {
            return Err(error::CommonError::Invalid {
                msg: "The length of input arrays don't match".to_string(),
                loc: snafu::location!(),
            });
        }

        let res = input
            .array1
            .iter()
            .zip(&input.array2)
            .map(|(a, b)| a + b)
            .collect();

        let operation_counter = {
            let mut counter = self.operation_counter.write()?;
            *counter += 1;
            *counter
        };

        Ok(CalculatorResponse {
            array: res,
            operation_counter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_counter() {
        let calculator = Calculator::new();

        let operation_counter = calculator.counter().unwrap();
        assert_eq!(operation_counter, 0);
    }

    #[test]
    fn test_calculator_sum_array() {
        let calculator = Calculator::new();

        let operation_counter = calculator.counter().unwrap();
        assert_eq!(operation_counter, 0);

        let response = calculator
            .sum_array(CalculatorInput {
                array1: vec![2.0, 3.0],
                array2: vec![4.0, 5.0],
            })
            .unwrap();
        assert_eq!(response.operation_counter, 1);
    }

    #[test]
    fn test_calculator_invalid_sum_array() {
        let calculator = Calculator::new();

        let operation_counter = calculator.counter().unwrap();
        assert_eq!(operation_counter, 0);

        let response = calculator.sum_array(CalculatorInput {
            array1: vec![2.0, 3.0],
            array2: vec![4.0],
        });
        assert!(response.is_err());
    }
}
