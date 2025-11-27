use crate::difficulty::BeatmapSsr;
use rosu_map::Beatmap;
use rosu_pp;
use std::str::FromStr;

pub fn calculate_difficulty(
    osu_map: &Beatmap,
    etterna_ssr: &BeatmapSsr,
    rate: f64,
) -> Result<BeatmapSsr, Box<dyn std::error::Error>> {
    let map_str = osu_map
        .clone()
        .encode_to_string()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let map = rosu_pp::Beatmap::from_str(&map_str)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let diff_attrs = rosu_pp::Difficulty::new()
        .clock_rate(rate)
        .calculate(&map);
    let sr = diff_attrs.stars() as f64;

    let weight = |value: f64| -> f64 {
        if etterna_ssr.overall > 0.0 {
            (value / etterna_ssr.overall) * sr
        } else {
            sr
        }
    };

    let ssr = BeatmapSsr {
        overall: sr,
        stream: weight(etterna_ssr.stream),
        jumpstream: weight(etterna_ssr.jumpstream),
        handstream: weight(etterna_ssr.handstream),
        stamina: weight(etterna_ssr.stamina),
        jackspeed: weight(etterna_ssr.jackspeed),
        chordjack: weight(etterna_ssr.chordjack),
        technical: weight(etterna_ssr.technical),
    };

    Ok(ssr)
}