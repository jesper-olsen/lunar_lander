use std::io::{self, Write};
use std::ops::RangeInclusive;

const GRAVITY: f64 = 5.0;
pub const MAX_THRUST: u8 = 30;
const DT: f64 = 1.0;

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Perfect,
    Hard,
    Crashed,
}

#[derive(Clone, Debug)]
pub struct Lander {
    pub altitude: f64,
    pub velocity: f64,
    pub fuel: f64,
    pub elapsed_time: u32,
    pub impact_velocity: Option<f64>,
    pub total_time: Option<f64>,
}

impl Default for Lander {
    fn default() -> Self {
        Self::new()
    }
}

impl Lander {
    pub fn new() -> Self {
        Self {
            altitude: 1000.0,
            velocity: 50.0,
            fuel: 150.0,
            elapsed_time: 0,
            impact_velocity: None,
            total_time: None,
        }
    }

    pub fn step(&mut self, burn_rate: u8) {
        if self.is_landed() {
            return;
        }

        let actual_burn = f64::from(burn_rate).min(self.fuel);
        let next_velocity = self.velocity + (GRAVITY - actual_burn) * DT;

        self.fuel -= actual_burn;
        self.altitude -= 0.5 * (self.velocity + next_velocity);

        // Check for touchdown
        if self.altitude <= 0.0 {
            let pre_impact_alt = self.altitude + 0.5 * (self.velocity + next_velocity);
            let a = GRAVITY - actual_burn;

            let touchdown_delta = if a.abs() < 1e-12 {
                pre_impact_alt / self.velocity
            } else {
                (-self.velocity + (self.velocity.powi(2) + pre_impact_alt * 2.0 * a).sqrt()) / a
            };

            self.impact_velocity = Some(self.velocity + a * touchdown_delta);
            self.total_time = Some(f64::from(self.elapsed_time) + touchdown_delta);
            self.altitude = 0.0;
        } else {
            self.velocity = next_velocity;
        }

        self.elapsed_time += 1;
    }

    pub fn is_landed(&self) -> bool {
        self.altitude <= 0.0
    }

    pub fn get_outcome(&self) -> Option<Outcome> {
        self.impact_velocity.map(|v| {
            if v <= 0.0 {
                Outcome::Perfect
            } else if v < 2.0 {
                Outcome::Hard
            } else {
                Outcome::Crashed
            }
        })
    }
}

// -- Utilities -------------------------------------------------------------

pub fn ask_yes_no(question: &str) -> bool {
    loop {
        print!("{question}? ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        match s.trim().to_lowercase().chars().next() {
            Some('y') => return true,
            Some('n') => return false,
            _ => println!(" Please answer Yes or No.\n"),
        }
    }
}

pub fn get_int<T>(prompt: &str, range: RangeInclusive<T>) -> T
where
    T: std::str::FromStr,
    T: PartialOrd,
{
    loop {
        print!("  {prompt}: ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        if let Ok(n) = s.trim().parse::<T>()
            && range.contains(&n)
        {
            return n;
        }
        println!("  INVALID.");
    }
}
