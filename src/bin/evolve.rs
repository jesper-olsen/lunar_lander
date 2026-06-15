use lunar_lander::{Lander, MAX_THRUST, Outcome};
use rand::RngExt;

struct TrajectoryResult {
    outcome: Outcome,
    final_speed: f64,
    fuel_remaining: f64,
    flight_duration: f64,
}

const SEQUENCE_LENGTH: usize = 30; // Max expected seconds of flight
const POPULATION_SIZE: usize = 1000;
const GENERATIONS: usize = 50;

// Evaluates a specific sequence of burns and returns the result
fn simulate_genome(genome: &[u8]) -> TrajectoryResult {
    let mut lander = Lander::new();
    let mut time = 0;

    while !lander.is_landed() {
        let intended_burn = *genome.get(time).unwrap_or(&0);
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

// Higher fitness score is better
fn calculate_fitness(result: &TrajectoryResult) -> f64 {
    match result.outcome {
        Outcome::Perfect => {
            let base_score = 10000.0;
            // Heavily reward leftover fuel (multiplier makes it a strong evolutionary pressure)
            let fuel_bonus = result.fuel_remaining * 10.0;
            // Penalize longer flight durations
            let time_penalty = result.flight_duration;

            base_score + fuel_bonus - time_penalty
        }

        Outcome::Hard => 5000.0 - result.final_speed,

        Outcome::Crashed => {
            let penalty = result.final_speed.abs();
            1000.0 - penalty.min(1000.0)
        }
    }
}

fn run_evolution() {
    println!("\n--- INITIATING EVOLUTIONARY AUTOPILOT ---");
    let mut rng = rand::rng();

    // 1. Initialize random population
    let mut population: Vec<Vec<u8>> = (0..POPULATION_SIZE)
        .map(|_| {
            (0..SEQUENCE_LENGTH)
                .map(|_| rng.random_range(0..=MAX_THRUST))
                .collect()
        })
        .collect();

    struct ScoredGenome {
        fitness: f64,
        genome: Vec<u8>,
        result: TrajectoryResult,
    }

    for generation in 1..=GENERATIONS {
        let mut scored_population: Vec<ScoredGenome> = population
            .into_iter()
            .map(|genome| {
                let result = simulate_genome(&genome);
                let fitness = calculate_fitness(&result);
                ScoredGenome {
                    fitness,
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
        let mut next_generation: Vec<Vec<u8>> = scored_population
            .iter()
            .take(elite_count)
            .map(|scored| scored.genome.clone())
            .collect();

        // Mutation: Fill the rest of the population by mutating the elites
        while next_generation.len() < POPULATION_SIZE {
            // Pick a random elite parent
            let parent_idx = rng.random_range(0..elite_count);
            let mut child = scored_population[parent_idx].genome.clone();

            // Mutate: Randomly change ~15% of the sequence
            for gene in child.iter_mut() {
                if rng.random_bool(0.15) {
                    *gene = rng.random_range(0..=MAX_THRUST);
                }
            }
            next_generation.push(child);
        }

        population = next_generation;
    }

    // Sort one final time in case the last mutation yielded the best result
    let mut final_population: Vec<ScoredGenome> = population
        .into_iter()
        .map(|genome| {
            let result = simulate_genome(&genome);
            let fitness = calculate_fitness(&result);
            ScoredGenome {
                fitness,
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

fn print_winning_trajectory(genome: &[u8], result: &TrajectoryResult) {
    println!("\n===============================================================");
    println!("SUCCESSFUL FLIGHT DATA RECOVERED (REPLAYING SIMULATION):");
    println!("===============================================================\n");
    println!("SEC  FEET      SPEED     FUEL     BURN  PLOT OF DISTANCE\n");

    let mut replay_lander = Lander::new();
    let mut time = 0;

    while !replay_lander.is_landed() {
        let star_col = 36 + (replay_lander.altitude / 15.0) as usize;

        let intended_burn = *genome.get(time).unwrap_or(&0);
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
