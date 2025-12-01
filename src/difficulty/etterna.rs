use crate::difficulty::BeatmapSsr;
use minacalc_rs::{AllRates, Calc, HashMapCalcExt, OsuCalcExt};
use rosu_map::Beatmap;

pub fn calculate_difficulty(
    map: &Beatmap,
    calc: &Calc,
    rate: f64,
) -> Result<BeatmapSsr, Box<dyn std::error::Error>> {
    let map_string = map.clone().encode_to_string().unwrap();
    let msd_results: AllRates = calc.calculate_msd_from_string(map_string)?;

    let hashmap_results = msd_results;
    let hashmap = hashmap_results.as_hashmap()?;

    let rate_key_precision_two = format!("{:.2}", rate);
    let rate_key_precision_one = format!("{:.1}", rate);

    let ssr_entry = hashmap
        .get(&rate_key_precision_two)
        .or_else(|| hashmap.get(&rate_key_precision_one))
        .or_else(|| hashmap.get("1.0"))
        .ok_or_else(|| {
            std::io::Error::other(format!("MinaCalc result missing for rate {}", rate))
        })?;

    let ssr = BeatmapSsr {
        overall: ssr_entry.overall as f64,
        stream: ssr_entry.stream as f64,
        jumpstream: ssr_entry.jumpstream as f64,
        handstream: ssr_entry.handstream as f64,
        stamina: ssr_entry.stamina as f64,
        jackspeed: ssr_entry.jackspeed as f64,
        chordjack: ssr_entry.chordjack as f64,
        technical: ssr_entry.technical as f64,
    };

    Ok(ssr)
}
