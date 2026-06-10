use std::io::{self, Write};

// ── constants matching the BASIC original ────────────────────────────────────
const INIT_HEIGHT: f64 = 1000.0;
const INIT_VELOCITY: f64 = 50.0;
const INIT_FUEL: f64 = 150.0;
const GRAVITY: f64 = 5.0; // ft/s added to downward velocity each second
const MAX_BURN: f64 = 30.0; // max fuel units per second

// ── I/O helpers ───────────────────────────────────────────────────────────────

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    buf.trim().to_string()
}

/// Format a number as BASIC PRINT would: integer when whole, else trim zeros.
fn fmt(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e12 {
        format!("{}", x as i64)
    } else {
        let s = format!("{:.4}", x);
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        s.to_string()
    }
}

// ── plot lines ────────────────────────────────────────────────────────────────
//
// BASIC uses TAB(col) which positions the cursor at an absolute column.
// We replicate this by writing characters into a Vec<char> at the right index.

fn plot_line(nums: &str, marker_col: usize, star_col: usize) {
    // No clamping: * can pass left of I as height drops below 525 ft,
    // just as TAB(H/15) < TAB(35) does in the original BASIC.
    let len = nums.len().max(marker_col + 1).max(star_col + 1);
    let mut line: Vec<char> = vec![' '; len];
    for (i, c) in nums.chars().enumerate() {
        line[i] = c;
    }
    line[marker_col] = 'I';
    line[star_col] = '*';
    let s: String = line.iter().collect();
    println!("{}", s.trim_end());
}

/// Normal status line — BASIC line 490.
/// Columns: TAB(35)→'I',  TAB(H/15)→'*'
fn print_status(t: f64, h: f64, v: f64, f: f64) {
    let nums = format!("{:<5} {:<9} {:<9} {:<8}", fmt(t), fmt(h), fmt(v), fmt(f));
    plot_line(&nums, 35, (h / 15.0).round() as usize);
}

/// Out-of-fuel status line — BASIC line 640.
/// Columns: TAB(29)→'I',  TAB(H/12+29)→'*'
fn print_status_nofuel(t: f64, h: f64, v: f64, f: f64) {
    let nums = format!("{:<4} {:<8} {:<8} {:<8}", fmt(t), fmt(h), fmt(v), fmt(f));
    plot_line(&nums, 29, (h / 12.0 + 29.0).round() as usize);
}

// ── landing outcome ──────────────────────────────────────────────────────────

fn print_landing(t: f64, v_land: f64, fuel: f64) {
    println!("***** CONTACT *****");
    println!("TOUCHDOWN AT {:.4} SECONDS.", t);
    println!("LANDING VELOCITY= {} FEET/SEC.", fmt(v_land));
    println!("{} UNITS OF FUEL REMAINING.", fmt(fuel.max(0.0)));
    println!();
    if v_land == 0.0 {
        println!("CONGRATULATIONS! A PERFECT LANDING!!");
        println!("YOUR LICENSE WILL BE RENEWED.......LATER.");
    } else if v_land.abs() >= 2.0 {
        println!("***** SORRY, BUT YOU BLEW IT!!!!");
        println!("APPROPRIATE CONDOLENCES WILL BE SENT TO YOUR NEXT OF KIN.");
    }
    // 0 < |v| < 2 → safe but imperfect; original prints nothing extra
}

// ── game loop ─────────────────────────────────────────────────────────────────
//
// Faithful translation of the BASIC GOTOs into structured Rust.
//
// BASIC flow each iteration:
//   490  print status (normal)          ← only when f > 0
//   500  INPUT B
//   510  if B<0 → B=0
//   520  cap B at 30
//   530  cap B at F
//   ---- (physics) ----
//   540  V1 = V - B + 5
//   560  F  = F - B
//   570  H  = H - 0.5*(V+V1)
//   580  if H<=0 → landing
//   590  T  = T + 1
//   600  V  = V1
//   610  if F>0 → back to 490
//   615  if B=0 → 640            (already coasting; skip "out of fuel" msg)
//   620  print "**** OUT OF FUEL ****"
//   640  print nofuel status      ← only when f just hit 0 OR coasting
//   650  B = 0
//   660  → 540                   (re-run physics, no new input)
//
// The key insight: once fuel is exhausted the loop body becomes:
//   [no print, no input] → physics → (maybe land) → advance T → print nofuel → repeat
// So we print the nofuel status ONCE per tick, at the END of the tick,
// not at the top.  When fuel is plentiful we print at the TOP.

fn run_mission() {
    println!("BEGINNING LANDING PROCEDURE..........");
    println!();
    println!("G O O D  L U C K ! ! !");
    println!();
    println!();
    println!("SEC  FEET      SPEED     FUEL     PLOT OF DISTANCE");
    println!();

    let mut t: f64 = 0.0;
    let mut h: f64 = INIT_HEIGHT;
    let mut v: f64 = INIT_VELOCITY;
    let mut f: f64 = INIT_FUEL;
    // True once we've entered the no-fuel coasting phase.
    let mut coasting = false;

    loop {
        // ── get burn amount ───────────────────────────────────────────────────
        let b: f64 = if !coasting {
            // Fuel available: show status and ask for input.
            print_status(t, h, v, f);
            print!("? ");
            io::stdout().flush().unwrap();
            let raw: f64 = read_line().parse().unwrap_or(0.0);
            // Line 510: negative → 0.  Lines 520/530: clamp to max and to fuel.
            if raw < 0.0 {
                0.0
            } else {
                raw.min(MAX_BURN).min(f)
            }
        } else {
            0.0 // forced; no print, no input
        };

        // ── physics (BASIC 540–570) ───────────────────────────────────────────
        let v1 = v - b + GRAVITY;
        let f1 = f - b;
        let h1 = h - 0.5 * (v + v1);

        // ── landing check (BASIC 580) ─────────────────────────────────────────
        if h1 <= 0.0 {
            // Solve for exact fractional second D of touchdown.
            //
            // Quadratic from H(d) = h - 0.5*(v + v+(5-b)*d)*d = 0:
            //   (5-b)/2 * d^2 + v*d - h = 0
            //
            // BASIC:
            //   general: D = (-V + sqrt(V*V + H*(10-2*B))) / (5-B)
            //   B=5:     D = H/V  (net acceleration zero; avoid ÷0)
            let d = if (b - GRAVITY).abs() < 1e-9 {
                h / v
            } else {
                let disc = v * v + h * (10.0 - 2.0 * b);
                (-v + disc.sqrt()) / (GRAVITY - b)
            };
            let v_land = v + (GRAVITY - b) * d;
            print_landing(t + d, v_land, f1);
            return;
        }

        // ── advance time (BASIC 590–600) ─────────────────────────────────────
        t += 1.0;
        v = v1;
        h = h1;
        f = (f1).max(0.0); // clamp float noise

        // ── out-of-fuel handling (BASIC 610–660) ──────────────────────────────
        // BASIC 610: if F>0, loop back to 490 (normal path — we just continue).
        if f <= 0.0 {
            if !coasting {
                // First time we've run dry this mission.
                // BASIC 615: if B was 0, skip the message (went straight to 640).
                if b > 0.0 {
                    println!("**** OUT OF FUEL ****");
                }
                coasting = true;
            }
            // BASIC 640: print nofuel status (every tick while coasting)
            print_status_nofuel(t, h, v, f);
        }
        // If f > 0, we loop back naturally; normal status will be printed at
        // the top of the next iteration.
    }
}

// ── entry point ───────────────────────────────────────────────────────────────

fn print_header() {
    println!("{:>36}", "ROCKET");
    println!("{:>51}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
    println!();
    println!();
    println!();
    println!("LUNAR LANDING SIMULATION");
    println!("----- ------- ----------");
    println!();
}

fn print_instructions() {
    println!();
    println!("YOU ARE LANDING ON THE MOON AND HAVE TAKEN OVER MANUAL");
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

fn main() {
    print_header();

    print!("DO YOU WANT INSTRUCTIONS (YES OR NO)? ");
    io::stdout().flush().unwrap();
    if read_line().to_uppercase() != "NO" {
        print_instructions();
    }

    loop {
        run_mission();

        println!();
        println!();
        println!();
        print!("ANOTHER MISSION? ");
        io::stdout().flush().unwrap();
        if read_line().to_uppercase() != "YES" {
            break;
        }
    }

    println!();
    println!("CONTROL OUT.");
    println!();
}
