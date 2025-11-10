mod increasing_only;

use std::fs;
use serde_derive::Deserialize;
use anyhow::Result;
use serde_yaml::from_str;
use serde_json::json;
use increasing_only::IncreasingOnly;

#[derive(Debug, Deserialize, Clone, Copy)]
struct FitnessActivity {
    distance: f64,              // km
    time_elapsed_seconds: f64,  // sec
    avg_heart_rate: Option<u32>,
    max_heart_rate: Option<u32>,
    elevation_gain_m: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct FitnessData {
    fitness_activities: Vec<FitnessActivity>,
}

fn main() -> Result<()> {
    println!("📂 Reading data from: fitness_activities.yaml");
    let yaml = fs::read_to_string("fitness_activities.yaml")?;
    let data: FitnessData = from_str(&yaml)?;
    let runs = &data.fitness_activities;

    println!("🏃 Loaded {} runs successfully!\n", runs.len());

    // Compute paces (min/km)
    let paces: Vec<f64> = runs
        .iter()
        .map(|r| (r.time_elapsed_seconds / 60.0) / r.distance)
        .collect();

    // Statistics via iterators only
    let longest_distance = runs.iter().map(|r| r.distance).fold(0.0, f64::max);
    let mean_distance = runs.iter().map(|r| r.distance).sum::<f64>() / runs.len() as f64;
    let std_dev = (runs
        .iter()
        .map(|r| (r.distance - mean_distance).powi(2))
        .sum::<f64>()
        / runs.len() as f64)
        .sqrt();
    let longest_time = runs.iter().map(|r| r.time_elapsed_seconds).fold(0.0, f64::max);
    let fastest_pace = runs
        .iter()
        .filter(|r| r.distance > 5.0)
        .map(|r| (r.time_elapsed_seconds / 60.0) / r.distance)
        .fold(f64::INFINITY, f64::min);
    let improvements: Vec<f64> = paces.windows(2).map(|w| w[0] - w[1]).collect();
    let (max_improvement, idx) = improvements
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, &val)| (val, i))
        .unwrap_or((0.0, 0));
    let record_breaks = IncreasingOnly::new(paces.iter().rev().cloned()).count();

    // JSON summary
    let summary = json!({
        "longest_run_km": longest_distance,
        "mean_distance_km": mean_distance,
        "std_dev_km": std_dev,
        "longest_run_sec": longest_time,
        "fastest_pace_min_per_km": fastest_pace,
        "biggest_speed_improvement_min_per_km": max_improvement,
        "improvement_run_indices": [idx + 1, idx + 2],
        "record_breaking_days": record_breaks
    });
    fs::write("summary.json", serde_json::to_string_pretty(&summary)?)?;

    // Console summary (optional)
    println!("🏃‍♂️ Running Statistics Summary");
    println!("---------------------------------------------");
    println!("📏 Longest Run (by distance):      {:.2} km", longest_distance);
    println!(
        "⏱️ Longest Run (by time):          {:.0} sec ({:.2} hr)",
        longest_time,
        longest_time / 3600.0
    );
    println!("📈 Mean Distance:                  {:.2} km", mean_distance);
    println!("📉 Std Deviation (distance):        {:.2} km", std_dev);
    println!("⚡ Fastest Run (> 5km):             {:.2} min/km", fastest_pace);
    println!(
        "🚀 Biggest Speed Improvement:       +{:.2} min/km (Run {} → Run {})",
        max_improvement,
        idx + 1,
        idx + 2
    );
    println!("🏅 Record-Breaking Days:            {}", record_breaks);
    println!("---------------------------------------------");
    println!("💾 JSON summary written to summary.json ✅");

    Ok(())
}
