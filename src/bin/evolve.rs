use lunar_lander::{Lander, MAX_THRUST, Outcome};
use rand::{Rng, RngExt};

struct TrajectoryResult {
    outcome: Outcome,
    final_speed: f64,
    fuel_remaining: f64,
    flight_duration: f64,
}

impl TrajectoryResult {
    // Higher fitness score is better
    fn fitness(&self) -> f64 {
        match self.outcome {
            Outcome::Perfect => {
                let base_score = 10000.0;
                // Heavily reward leftover fuel (multiplier makes it a strong evolutionary pressure)
                let fuel_bonus = self.fuel_remaining * 10.0;
                // Penalize longer flight durations
                let time_penalty = self.flight_duration;

                base_score + fuel_bonus - time_penalty
            }

            Outcome::Hard => 5000.0 - self.final_speed,

            Outcome::Crashed => {
                let penalty = self.final_speed.abs();
                1000.0 - penalty.min(1000.0)
            }
        }
    }
}

const SEQUENCE_LENGTH: usize = 30; // Max expected seconds of flight
struct Genome([u8; SEQUENCE_LENGTH]);
const POPULATION_SIZE: usize = 1000;
const GENERATIONS: usize = 50;

impl Genome {
    fn random(rng: &mut impl Rng) -> Self {
        Genome(std::array::from_fn(|_| rng.random_range(0..MAX_THRUST)))
    }

    fn clone(&self) -> Self {
        Genome(self.0.clone())
    }

    fn mutate(mut self, rng: &mut impl Rng) -> Self {
        for gene in &mut self.0.iter_mut() {
            // let prob = 0.15;
            let prob = 2.0 / SEQUENCE_LENGTH as f64;
            if rng.random_bool(prob) {
                *gene = rng.random_range(0..=MAX_THRUST)
            }
        }
        self
    }

    fn crossover(&self, other: &Genome, rng: &mut impl Rng) -> Self {
        let point = rng.random_range(1..SEQUENCE_LENGTH);
        let genes = std::array::from_fn(|i| if i < point { self.0[i] } else { other.0[i] });
        Genome(genes)
    }

    // Evaluates a specific sequence of burns and returns the result
    fn simulate(&self) -> TrajectoryResult {
        let mut lander = Lander::new();
        let mut time = 0;

        while !lander.is_landed() {
            let intended_burn = *self.0.get(time).unwrap_or(&0);
            let max_burn = MAX_THRUST.min(lander.fuel as u8);
            let actual_burn = intended_burn.min(max_burn);

            lander.step(actual_burn);
            time += 1;
        }

        let final_speed = lander.impact_velocity.expect("No impact velocity");
        let total_time = lander.total_time.expect("No total time");
        let outcome = lander.get_outcome().unwrap();

        TrajectoryResult {
            outcome,
            final_speed,
            fuel_remaining: lander.fuel,
            flight_duration: total_time, // <-- Capture the exact time
        }
    }
}

fn run_evolution() {
    println!("\n--- INITIATING EVOLUTIONARY AUTOPILOT ---");
    let mut rng = rand::rng();

    // Initialize random population
    let mut population: Vec<Genome> = (0..POPULATION_SIZE)
        .map(|_| Genome::random(&mut rng))
        .collect();

    struct ScoredGenome {
        fitness: f64,
        genome: Genome,
        result: TrajectoryResult,
    }

    for generation in 1..=GENERATIONS {
        let mut scored_population: Vec<ScoredGenome> = population
            .into_iter()
            .map(|genome| {
                let result = genome.simulate();
                ScoredGenome {
                    fitness: result.fitness(),
                    genome,
                    result,
                }
            })
            .collect();

        // Sort descending by fitness
        scored_population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let best = &scored_population[0];

        if generation % 5 == 0 || generation == 1 {
            println!(
                "Gen {generation:02} | Best Fitness: {:.1} | Impact Speed: {:.2} ft/s | Fuel left: {:.1} | Duration: {:.1} | {:?}",
                best.fitness,
                best.result.final_speed,
                best.result.fuel_remaining,
                best.result.flight_duration,
                best.result.outcome
            );
        }

        // perfect landing - stop early
        //if *best_outcome == Outcome::Perfect {
        //  break
        //}

        // Selection: Keep the top 10% (Elitism)
        let elite_count = POPULATION_SIZE / 10;
        let mut next_generation: Vec<Genome> = scored_population
            .iter()
            .take(elite_count)
            .map(|scored| scored.genome.clone())
            .collect();

        // Mutation: Fill the rest of the population by mutating the elites
        while next_generation.len() < POPULATION_SIZE {
            // Pick a random elite parent
            let parent_idx1 = rng.random_range(0..elite_count);
            let parent_idx2 = rng.random_range(0..elite_count);
            let genome1 = &scored_population[parent_idx1].genome;
            let genome2 = &scored_population[parent_idx2].genome;
            let child = genome1.crossover(genome2, &mut rng).mutate(&mut rng);
            next_generation.push(child);
        }

        population = next_generation;
    }

    // Sort one final time in case the last mutation yielded the best result
    let mut final_population: Vec<ScoredGenome> = population
        .into_iter()
        .map(|genome| {
            let result = genome.simulate();
            ScoredGenome {
                fitness: result.fitness(),
                genome,
                result,
            }
        })
        .collect();
    final_population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let best_result = &final_population[0].result;
    let best_genome = &final_population[0].genome;

    if best_result.outcome == Outcome::Perfect {
        println!("\nEVOLUTION COMPLETE: Optimal flight plan found.");
        print_winning_trajectory(best_genome, best_result);
    } else {
        println!("\nEvolution finished, but no perfect landing was found.");
    }
}

fn print_winning_trajectory(genome: &Genome, result: &TrajectoryResult) {
    println!("\n===============================================================");
    println!("SUCCESSFUL FLIGHT DATA RECOVERED (REPLAYING SIMULATION):");
    println!("===============================================================\n");
    println!("SEC  FEET      SPEED     FUEL     BURN  PLOT OF DISTANCE\n");

    let mut replay_lander = Lander::new();
    let mut time = 0;

    while !replay_lander.is_landed() {
        let star_col = 36 + (replay_lander.altitude / 15.0) as usize;

        let intended_burn = *genome.0.get(time).unwrap_or(&0);
        let max_burn = MAX_THRUST.min(replay_lander.fuel as u8);
        let actual_burn = intended_burn.min(max_burn);

        println!(
            "{:<5}{:<10.2}{:<10.2}{:<9.1}{actual_burn:<6}I{:>pad$}*",
            replay_lander.elapsed_time,
            replay_lander.altitude,
            replay_lander.velocity,
            replay_lander.fuel,
            "",
            pad = star_col.saturating_sub(36)
        );

        replay_lander.step(actual_burn);
        time += 1;
    }

    println!("***** CONTACT *****");
    println!("LANDING VELOCITY = {:.2} FEET/SEC.", result.final_speed);
    println!("{:.1} UNITS OF FUEL REMAINING.", result.fuel_remaining);
}

fn main() {
    println!("{:>57}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n");
    println!("LUNAR LANDING SIMULATION");
    println!("----- ------- ----------\n");

    //let count = get_int::<usize>("HOW MANY GENERATIONS", 1..=1000);
    run_evolution();
}
