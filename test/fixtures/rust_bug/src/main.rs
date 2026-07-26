// Fixture: accumulating-state bug, debugged via CodeLLDB.
//
// running_total is meant to hold the sum of doses administered so far, but the
// += happens BEFORE the per-patient reset check, so patient B's total carries
// A's dose. The visible symptom (a wrong number) is far from the cause, which
// makes it a reasonable target for breakpoint + variable inspection.

#[derive(Debug)]
struct Patient {
    name: String,
    dose_mg: f64,
    weight_kg: f64,
}

fn dose_per_kg(total_mg: f64, weight_kg: f64) -> f64 {
    total_mg / weight_kg
}

fn accumulate(patients: &[Patient]) -> Vec<f64> {
    let mut running_total = 0.0;
    let mut results = Vec::new();

    for p in patients {
        running_total += p.dose_mg;
        let per_kg = dose_per_kg(running_total, p.weight_kg);
        results.push(per_kg);
    }

    results
}

fn main() {
    let patients = vec![
        Patient { name: "A".to_string(), dose_mg: 700.0, weight_kg: 70.0 },
        Patient { name: "B".to_string(), dose_mg: 450.0, weight_kg: 90.0 },
    ];

    let results = accumulate(&patients);

    for (p, r) in patients.iter().zip(results.iter()) {
        println!("{} {:.2}", p.name, r);
    }
}
