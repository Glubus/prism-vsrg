//! Difficulty calculation module.

#![allow(dead_code)]

use minacalc_rs::{AllRates, Calc, HashMapCalcExt, OsuCalcExt};
use rosu_map::Beatmap;
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
mod etterna;
mod osu;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex, OnceLock};

struct CalcHolder(Calc);

unsafe impl Send for CalcHolder {}
unsafe impl Sync for CalcHolder {}

#[derive(Debug, Clone, Default)]
pub struct BeatmapSsr {
    pub overall: f64,
    pub stream: f64,
    pub jumpstream: f64,
    pub handstream: f64,
    pub stamina: f64,
    pub jackspeed: f64,
    pub chordjack: f64,
    pub technical: f64,
}

#[derive(Debug, Clone)]
pub struct BeatmapRatingValue {
    pub name: String,
    pub ssr: BeatmapSsr,
}

impl BeatmapRatingValue {
    pub fn new(name: impl Into<String>, ssr: BeatmapSsr) -> Self {
        Self {
            name: name.into(),
            ssr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DifficultyInfo {
    pub duration_ms: i32,
    pub nps: f64,
    pub ratings: Vec<BeatmapRatingValue>,
}

impl DifficultyInfo {
    pub fn new(duration_ms: i32, nps: f64, ratings: Vec<BeatmapRatingValue>) -> Self {
        Self {
            duration_ms,
            nps,
            ratings,
        }
    }
}

static GLOBAL_CALC: OnceLock<Arc<Mutex<CalcHolder>>> = OnceLock::new();

pub fn init_global_calc() -> Result<(), Box<dyn std::error::Error>> {
    if GLOBAL_CALC.get().is_none() {
        let calc = Calc::new()?;
        let holder = Arc::new(Mutex::new(CalcHolder(calc)));
        let _ = GLOBAL_CALC.set(holder);
    }
    Ok(())
}

fn with_global_calc<F, R>(f: F) -> Result<R, Box<dyn std::error::Error>>
where
    F: FnOnce(&Calc) -> Result<R, Box<dyn std::error::Error>>,
{
    init_global_calc()?;
    let calc_arc = GLOBAL_CALC
        .get()
        .ok_or_else(|| std::io::Error::other("Global MinaCalc not initialized"))?;
    let calc_guard = calc_arc
        .lock()
        .map_err(|_| std::io::Error::other("Calc lock poisoned"))?;
    f(&calc_guard.0)
}

/// Analyse basique d'une beatmap (placeholder pour calculs futurs)
pub fn analyze(map: &Beatmap) -> Result<DifficultyInfo, Box<dyn std::error::Error>> {
    init_global_calc()?;
    with_global_calc(|calc| analyze_with_calc(map, calc))
}

pub fn analyze_for_rate(
    map: &Beatmap,
    rate: f64,
) -> Result<Vec<BeatmapRatingValue>, Box<dyn std::error::Error>> {
    init_global_calc()?;
    with_global_calc(|calc| {
        let etterna_ssr = etterna::calculate_difficulty(map, calc, rate)?;
        let osu_ssr = osu::calculate_difficulty(map, &etterna_ssr, rate)?;
        Ok(vec![
            BeatmapRatingValue::new("etterna", etterna_ssr),
            BeatmapRatingValue::new("osu", osu_ssr),
        ])
    })
}

#[derive(Debug, Clone)]
pub struct RateDifficultyCache {
    pub available_rates: Vec<f64>,
    pub ratings_by_rate: Vec<(f64, Vec<BeatmapRatingValue>)>,
}

pub fn analyze_all_rates(map: &Beatmap) -> Result<RateDifficultyCache, Box<dyn std::error::Error>> {
    init_global_calc()?;
    with_global_calc(|calc| analyze_all_rates_with_calc(map, calc))
}

fn analyze_all_rates_with_calc(
    map: &Beatmap,
    calc: &Calc,
) -> Result<RateDifficultyCache, Box<dyn std::error::Error>> {
    let map_string = map.clone().encode_to_string().unwrap();
    let msd_results: AllRates = calc.calculate_msd_from_string(map_string)?;
    let hashmap = msd_results.as_hashmap()?;

    let mut per_rate: Vec<(f64, Vec<BeatmapRatingValue>)> = Vec::new();

    for (rate_key, ssr_entry) in hashmap.iter() {
        let Ok(rate_value) = rate_key.parse::<f64>() else {
            continue;
        };

        let etterna_ssr = BeatmapSsr {
            overall: ssr_entry.overall as f64,
            stream: ssr_entry.stream as f64,
            jumpstream: ssr_entry.jumpstream as f64,
            handstream: ssr_entry.handstream as f64,
            stamina: ssr_entry.stamina as f64,
            jackspeed: ssr_entry.jackspeed as f64,
            chordjack: ssr_entry.chordjack as f64,
            technical: ssr_entry.technical as f64,
        };

        let osu_ssr = osu::calculate_difficulty(map, &etterna_ssr, rate_value)?;

        per_rate.push((
            rate_value,
            vec![
                BeatmapRatingValue::new("etterna", etterna_ssr.clone()),
                BeatmapRatingValue::new("osu", osu_ssr),
            ],
        ));
    }

    per_rate.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    let available_rates = per_rate.iter().map(|(rate, _)| *rate).collect();

    Ok(RateDifficultyCache {
        available_rates,
        ratings_by_rate: per_rate,
    })
}

fn analyze_with_calc(
    map: &Beatmap,
    calc: &Calc,
) -> Result<DifficultyInfo, Box<dyn std::error::Error>> {
    if map.hit_objects.is_empty() {
        return Err(Box::new(std::io::Error::other("No hit objects found")));
    }

    let first = map.hit_objects.first().map(|h| h.start_time).unwrap_or(0.0);
    let last = map
        .hit_objects
        .last()
        .map(|h| h.start_time.max(resolve_end_time(h)))
        .unwrap_or(first);

    let duration = (last - first).max(0.0);
    let duration_secs = duration / 1000.0;
    let nps = if duration_secs > 0.0 {
        map.hit_objects.len() as f64 / duration_secs
    } else {
        0.0
    };

    let etterna_ssr = etterna::calculate_difficulty(map, calc, 1.0)?;
    let osu_ssr = osu::calculate_difficulty(map, &etterna_ssr, 1.0)?;

    let ratings = vec![
        BeatmapRatingValue::new("etterna", etterna_ssr),
        BeatmapRatingValue::new("osu", osu_ssr),
    ];
    // TODO: ajouter les ratings MinaCalc / rosu-pp ici.
    Ok(DifficultyInfo::new(duration as i32, nps, ratings))
}

fn resolve_end_time(obj: &HitObject) -> f64 {
    match &obj.kind {
        HitObjectKind::Hold(hold) => obj.start_time + hold.duration,
        // TODO: Slider duration requires path length + velocity; fallback to start time for now.
        _ => obj.start_time,
    }
}
