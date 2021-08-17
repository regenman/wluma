use chrono::{Local, Timelike};
use std::collections::HashMap;
use std::error::Error;

pub struct Als {
    hour_to_lux: HashMap<u32, u32>,
}

impl Als {
    pub fn new(hour_to_lux: &HashMap<String, u32>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            hour_to_lux: (0..24)
                .into_iter()
                .fold(Vec::<(u32, u32)>::new(), |mut acc, hour| {
                    let lux = hour_to_lux
                        .get(&hour.to_string())
                        .map(|&v| v)
                        .unwrap_or(acc.last().map(|&v| v.1).unwrap_or(0));
                    acc.push((hour, lux));
                    acc
                })
                .into_iter()
                .collect(),
        })
    }
}

impl super::Als for Als {
    fn get_raw(&self) -> Result<f64, Box<dyn Error>> {
        Ok(*self
            .hour_to_lux
            .get(&Local::now().hour())
            .ok_or("Unable to find ALS value for the current hour")? as f64)
    }
}
