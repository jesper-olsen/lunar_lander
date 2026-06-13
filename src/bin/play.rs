use std::io::{self, Write};
use lunar_lander::Lander;
use lunar_lander::Outcome;
use lunar_lander::ask_yes_no;
use lunar_lander::get_int;
use lunar_lander::MAX_THRUST;

fn instructions() {
    println!("\nYOU ARE LANDING ON THE MOON AND HAVE TAKEN OVER MANUAL");
    println!("CONTROL 1000 FEET ABOVE A GOOD LANDING SPOT. YOU HAVE A DOWN-");
    println!("WARD VELOCITY OF 50 FEET/SEC. 150 UNITS OF FUEL REMAIN.\n");
    println!("HERE ARE THE RULES THAT GOVERN YOUR APOLLO SPACE-CRAFT:\n");
    println!("(1) AFTER EACH SECOND THE HEIGHT, VELOCITY, AND REMAINING FUEL");
    println!("    WILL BE REPORTED VIA DIGBY YOUR ON-BOARD COMPUTER.");
    println!("(2) AFTER THE REPORT A '?' WILL APPEAR. ENTER THE NUMBER");
    println!("    OF UNITS OF FUEL YOU WISH TO BURN DURING THE NEXT");
    println!("    SECOND. EACH UNIT OF FUEL WILL SLOW YOUR DESCENT BY");
    println!("    1 FOOT/SEC.");
    println!("(3) THE MAXIMUM THRUST OF YOUR ENGINE IS 30 FEET/SEC/SEC");
    println!("    OR 30 UNITS OF FUEL PER SECOND.");
    println!("(4) WHEN YOU CONTACT THE LUNAR SURFACE, YOUR DESCENT ENGINE");
    println!("    WILL AUTOMATICALLY SHUT DOWN AND YOU WILL BE GIVEN A");
    println!("    REPORT OF YOUR LANDING SPEED AND REMAINING FUEL.");
    println!("(5) IF YOU RUN OUT OF FUEL THE '?' WILL NO LONGER APPEAR");
    println!("    BUT YOUR SECOND BY SECOND REPORT WILL CONTINUE UNTIL");
    println!("    YOU CONTACT THE LUNAR SURFACE.\n");
}

fn play_game() {
    println!("BEGINNING LANDING PROCEDURE..........\n");
    println!("G O O D  L U C K ! ! !\n\n");
    println!("SEC  FEET      SPEED     FUEL     PLOT OF DISTANCE\n");

    let mut lander = Lander::new();

    while !lander.is_landed() {
        let star_col = 36 + (lander.altitude / 15.0) as usize;

        print!(
            "{:<5}{:<10.2}{:<10.2}{:<9.1}I{:>pad$}*",
            lander.elapsed_time,
            lander.altitude,
            lander.velocity,
            lander.fuel,
            "",
            pad = star_col.saturating_sub(36)
        );
        io::stdout().flush().unwrap();

        let burn_rate = if lander.fuel > 0.0 {
            let upper_limit = MAX_THRUST.min(lander.fuel as u8);
            get_int::<u8>("\n?", 0..=upper_limit)
        } else {
            println!("\n**** OUT OF FUEL ****");
            0
        };

        lander.step(burn_rate);
    }

    println!("***** CONTACT *****");

    let total_time = lander
        .total_time
        .expect("Simulation ended without touchdown time");
    let impact_velocity = lander
        .impact_velocity
        .expect("Simulation ended without impact velocity");
    let outcome = lander.get_outcome().unwrap();

    println!("TOUCHDOWN AT {total_time:.2} SECONDS");
    println!("LANDING VELOCITY = {impact_velocity:.2} FEET/SEC.");
    println!("{:.1} UNITS OF FUEL REMAINING.", lander.fuel.max(0.0));

    match outcome {
        Outcome::Perfect => {
            println!("CONGRATULATIONS! A PERFECT LANDING!!");
            println!("YOUR LICENSE WILL BE RENEWED.......LATER.");
        }
        Outcome::Hard => {
            println!("HARD LANDING. YOU SURVIVED, BUT THE SHIP TOOK DAMAGE.");
        }
        Outcome::Crashed => {
            println!("***** SORRY, BUT YOU BLEW IT!!!!");
            println!("APPROPRIATE CONDOLENCES WILL BE SENT TO YOUR NEXT OF KIN.\n\n");
        }
    }
}

fn main() {
    println!("{:>57}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n");
    println!("LUNAR LANDING SIMULATION");
    println!("----- ------- ----------\n");

    if ask_yes_no("DO YOU WANT INSTRUCTIONS (YES OR NO)?") {
        instructions();
    }

    loop {
        play_game();
        if !ask_yes_no("ANOTHER MISSION?") {
            println!("CONTROL OUT.\n");
            break;
        }
    }
}
