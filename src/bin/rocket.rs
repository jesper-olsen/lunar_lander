use std::io;
use std::io::Write;
//use std::range::RangeInclusive;
use std::ops::RangeInclusive;

fn instructions() {
    println!();
    println!("YOU ARE LANDING ON THE MOON AND AND HAVE TAKEN OVER MANUAL");
    println!("CONTROL 1000 FEET ABOVE A GOOD LANDING SPOT. YOU HAVE A DOWN-");
    println!("WARD VELOCITY OF 50 FEET/SEC. 150 UNITS OF FUEL REMAIN.");
    println!();
    println!("HERE ARE THE RULES THAT GOVERN YOUR APOLLO SPACE-CRAFT:");
    println!();
    println!("(1) AFTER EACH SECOND THE HEIGHT, VELOCITY, AND REMAINING FUEL");
    println!("    WILL BE REPORTED VIA DIGBY YOUR ON-BOARD COMPUTER.");
    println!("(2) AFTER THE REPORT A '?' WILL APPEAR. ENTER THE NUMBER");
    println!("    OF UNITS OF FUEL YOU WISH TO BURN DURING THE NEXT");
    println!("    SECOND. EACH UNIT OF FUEL WILL SLOW YOUR DESCENT BY");
    println!("    1 FOOT/SEC.");
    println!("(3) THE MAXIMUM THRUST OF YOUR ENGINE IS 30 FEET/SEC/SEC");
    println!("    OR 30 UNITS OF FUEL PER SECOND.");
    println!("(4) WHEN YOU CONTACT THE LUNAR SURFACE. YOUR DESCENT ENGINE");
    println!("    WILL AUTOMATICALLY SHUT DOWN AND YOU WILL BE GIVEN A");
    println!("    REPORT OF YOUR LANDING SPEED AND REMAINING FUEL.");
    println!("(5) IF YOU RUN OUT OF FUEL THE '?' WILL NO LONGER APPEAR");
    println!("    BUT YOUR SECOND BY SECOND REPORT WILL CONTINUE UNTIL");
    println!("    YOU CONTACT THE LUNAR SURFACE.");
    println!();
}

fn game_loop() {
    println!("BEGINNING LANDING PROCEDURE..........");
    println!();
    println!("G O O D  L U C K ! ! !");
    println!();
    println!();
    println!("SEC  FEET      SPEED     FUEL     PLOT OF DISTANCE");
    println!();

    let mut h: f64 = 1000.0;
    let mut v: f64 = 50.0;
    let mut f: f64 = 150.0;
    let mut b;
    let mut t: usize = 0;
    let mut v1;
    loop {
        b = if f > 0.0 {
            let star_col = 36 + (h / 15.0) as usize;
            println!(
                "{:<6}{:<10}{:<10}{:<9}I{:>pad$}*",
                t,
                h,
                v,
                f,
                "",
                pad = star_col.saturating_sub(36)
            );
            let ulimit = 30u8.min(f as u8);
            get_int::<u8>("?", 0..=ulimit)
        } else {
            0u8
        };

        v1 = v - b as f64 + 5.0;
        f -= b as f64;
        h -= 0.5 * (v + v1);
        if h <= 0.0 {
            break; //contact 670
        }
        t += 1;
        v = v1;
        if f > 0.0 {
            continue;
        }
        if b == 0 {
            let star_col = 36 + (h / 12.0 + 29.0) as usize;
            let pad = star_col.saturating_sub(30);
            println!("{t:<4}{h:<8}{v:<8}{f:<9}I{:>pad$}*", "");
        } else {
            println!("**** OUT OF FUEL ****")
        }
    }
    println!("***** CONTACT *****");
    h += 0.5 * (v1 + v);
    let d = if b == 5 {
        h / v
    } else {
        (-v + (v * v + h * (10.0 - 2.0 * b as f64)).sqrt()) / (5.0 - b as f64)
    };
    println!("TOUCHDOWN AT {} SECONDS", t as f64 + d);
    v1 = v + (5.0 - b as f64) * d;

    println!("LANDING VELOCITY={v1} FEET/SEC.");
    println!("{f} UNITS OF FUEL REMAINING.");
    if v1 == 0.0 {
        println!("CONGRATULATIONS! A PERFECT LANDING!!");
        println!("YOUR LICENSE WILL BE RENEWED.......LATER.");
    } else if v1.abs() >= 2.0 {
        println!("***** SORRY, BUT YOU BLEW IT!!!!");
        println!("APPROPRIATE CONDOLENCES WILL BE SENT TO YOUR NEXT OF KIN.");
        println!("\n\n");
    }
}

fn main() {
    println!(
        "{}{}",
        " ".repeat(15),
        "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY"
    );
    println!();
    println!();
    println!();
    println!("LUNAR LANDING SIMULATION");
    println!("----- ------- ----------\n");

    if yes("DO YOU WANT INSTRUCTIONS (YES OR NO)?", "", "") {
        instructions()
    }
    loop {
        game_loop();
        if !yes("ANOTHER MISSION?", "", "CONTROL OUT.\n") {
            return;
        }
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    String::from(s.trim())
}

fn yes(q: &str, y: &str, n: &str) -> bool {
    loop {
        println!("{q}\n** ");
        let _ = io::stdout().flush();
        if let Some(c) = get_input().chars().next() {
            match c {
                'Y' | 'y' => {
                    println!("{y}");
                    return true;
                }
                'N' | 'n' => {
                    println!("{n}");
                    return false;
                }
                _ => println!(" Please answer Yes or No.\n"),
            }
        }
    }
}

fn rl() -> String {
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_uppercase()
}

fn get_int<T>(prompt: &str, range: RangeInclusive<T>) -> T
where
    T: std::str::FromStr,
    T: PartialOrd,
{
    loop {
        print!("  {prompt}: ");
        let _ = io::stdout().flush();

        if let Ok(n) = rl().parse::<T>()
            && range.contains(&n)
        {
            return n;
        }

        println!("  INVALID.");
    }
}
