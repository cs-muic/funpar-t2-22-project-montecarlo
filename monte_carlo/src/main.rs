extern crate csv;
extern crate rand;

use std::time::Instant;
use std::{error::Error, time::Duration};
use rayon::prelude::*;
use rand_distr::{Normal, Distribution};
use rand::{thread_rng, Rng};

pub struct OptionPricing {
    strike_price: f64,
    risk_free_rate: f64,        // percentage expressed as a decimal (0 < r < 1)
    time_to_maturity: f64,
    volatility: f64,            // percentage as decimal (0 < v < 1)
}

pub fn read_csv(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true) // Skip first line
        .from_path(file_path)?;
    
    let mut ending_values = Vec::new();
    for result in reader.records() {
        let record = result?;
        let value: f64 = record[1].parse()?;
        ending_values.push(value);
    }
    
    Ok(ending_values)
}

pub fn mtcl_simulator(ending_values: &str, op: OptionPricing, num_sim: i32, num_threads: i32) -> Result<f64, Box<dyn Error>> {
    let ending_values = read_csv(ending_values)?;
    let n = ending_values.len();
    let OptionPricing { strike_price, risk_free_rate, time_to_maturity, volatility } = op;
    let simulations_per_thread = num_sim / num_threads;

    // create a channel for sending payoffs from each thread to the main thread
    let total_payoff: f64 = (0..num_threads)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let normal = Normal::new(0.0, 1.0).unwrap();
            let mut payoff = 0.0;

            for _ in 0..simulations_per_thread {
                let rnd_n = rng.gen_range(0..n);
                let mut price = ending_values[rnd_n];
                let drift = (risk_free_rate - 0.5 * volatility * volatility) * time_to_maturity;
                let diffusion = volatility * normal.sample(&mut rng) * time_to_maturity.sqrt();
                price *= (drift + diffusion).exp();
                let payoff_sim = (price - strike_price).max(0.0);
                payoff += payoff_sim;
            }
            println!("payoff =  {:?}", &payoff);
            payoff
        })
        .sum();

    let option_price = (total_payoff / num_sim as f64) * (1.0 / (1.0 + risk_free_rate* time_to_maturity));

    println!("\nTotal payoff =  {:?} | Option price = {:?}", &total_payoff, &option_price);

    Ok(option_price)
}

pub fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
}

pub fn run_sim_default(ending_values: &str)
{
    // For datasets with ~30,000,000 values
    let n_sim = 30_000_000usize;
    let n_thr = 1000;

    let (res, t) = timed(|| 
        mtcl_simulator(ending_values, 
            OptionPricing { strike_price: 100.0, risk_free_rate: 0.025, time_to_maturity: 1.0, volatility: 0.2 }, 
            n_sim as i32, n_thr));
            
    println!("mtcl_sim: res = {:?}, t={}s", res, t.as_secs_f64());
}

pub fn run_sim_fine_tuned(ending_values: &str, n_sim: usize)
{
    let n_thr: i32 = 1000;

    let (res, t) = timed(|| 
        mtcl_simulator(ending_values, 
            OptionPricing { strike_price: 100.0, risk_free_rate: 0.025, time_to_maturity: 1.0, volatility: 0.2 }, 
            n_sim as i32, n_thr));
            
    println!("mtcl_sim: res = {:?}, t={}s", res, t.as_secs_f64());
}

fn main() {
    // let testing_2 = "/Users/TX3014/Downloads/funpar-t2-22-project-montecarlo/test_cases/test2.csv";
    // let testing_3 = "/Users/TX3014/Downloads/funpar-t2-22-project-montecarlo/test_cases/test3.csv";
    // let testing_4 = "/Users/TX3014/Downloads/funpar-t2-22-project-montecarlo/test_cases/test4.csv";
    let testing_5 = "/Users/TX3014/Downloads/funpar-t2-22-project-montecarlo/test_cases/test5.csv";

    // run_sim_fine_tuned(&testing_2, 1000);
    // run_sim_fine_tuned(&testing_3, 1_000);
    // run_sim_fine_tuned(&testing_4, 900_000);
    run_sim_fine_tuned(&testing_5, 1_000);
}