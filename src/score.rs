use std::collections::HashMap;
use std::sync::LazyLock;

use crate::puller::RaceResult;

static TENTH_MULT: u32 = 5;
static SCORE_MULT_TOP: u32 = 1;
static SCORE_MULT_MID: u32 = 2;
static SCORE_MULT_BOT: u32 = 3;
static SCORE_MULT_ROOKIE: u32 = 2;

// NOTE: This file has year specific values, currently 2025
// Will need to be updated every year

static SCORE_MAPPING: LazyLock<HashMap<String, u32>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // Top
    m.insert("McLaren".to_string(), SCORE_MULT_TOP);
    m.insert("Mercedes".to_string(), SCORE_MULT_TOP);
    m.insert("Ferrari".to_string(), SCORE_MULT_TOP);
    m.insert("Red Bull".to_string(), SCORE_MULT_TOP);

    // Mid
    m.insert("Aston Martin".to_string(), SCORE_MULT_MID);
    m.insert("Alpine F1 Team".to_string(), SCORE_MULT_MID);
    m.insert("RB F1 Team".to_string(), SCORE_MULT_MID);
    m.insert("Haas F1 Team".to_string(), SCORE_MULT_MID);

    // Bot
    m.insert("Williams".to_string(), SCORE_MULT_BOT);
    m.insert("Sauber".to_string(), SCORE_MULT_BOT);

    m.insert("Retired".to_string(), 0);

    m
});

static SCORE_ROOKIES: LazyLock<HashMap<String, bool>> = LazyLock::new(|| {
    let mut s = HashMap::new();

    // Known rookies
    s.insert("Lawson".to_string(), true);
    s.insert("Antonelli".to_string(), true);
    s.insert("Doohan".to_string(), true);
    s.insert("Hadjar".to_string(), true);
    s.insert("Bearman".to_string(), true);
    s.insert("Bortoleto".to_string(), true);
    s.insert("Colapinto".to_string(), true);

    // known returning
    s.insert("Piastri".to_string(), false);
    s.insert("Norris".to_string(), false);
    s.insert("Leclerc".to_string(), false);
    s.insert("Hamilton".to_string(), false);
    s.insert("Verstappen".to_string(), false);
    s.insert("Russell".to_string(), false);
    s.insert("Stroll".to_string(), false);
    s.insert("Alonso".to_string(), false);
    s.insert("Gasly".to_string(), false);
    s.insert("Tsunoda".to_string(), false);
    s.insert("Ocon".to_string(), false);
    s.insert("Albon".to_string(), false);
    s.insert("Sainz".to_string(), false);
    s.insert("Hulkenberg".to_string(), false);

    s
});

/// Points for top 15, linear
/// top 12 if sprint
fn base_score(result: &RaceResult) -> u32 {
    let top: u32 = if result.sprint { 12 } else { 15 } + 1;
    if let Some(position) = result.get_position() {
        top.saturating_sub(position)
    } else {
        0
    }
}

fn rookie_mult(result: &RaceResult) -> u32 {
    match SCORE_ROOKIES.get(&result.driver.get_name()) {
        Some(true) => SCORE_MULT_ROOKIE,
        None => {
            eprintln!(
                "Unknown driver: {} (assuming rookie)",
                result.driver.get_name()
            );
            SCORE_MULT_ROOKIE
        }
        _ => 1,
    }
}

pub fn to_score(result: &RaceResult) -> u32 {
    base_score(result) * rookie_mult(result) * SCORE_MAPPING.get(&result.constructor.name).unwrap()
}

pub fn tenth_score(result: &RaceResult) -> u32 {
    let top: u32 = if result.sprint { 8 } else { 10 };
    if let Some(position) = result.get_position() {
        top.saturating_sub(position.abs_diff(top)) * TENTH_MULT
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenth_score() {
        assert_eq!(
            tenth_score(&RaceResult::new("10".to_owned(), "Aston Martin".to_owned())),
            10
        );
        assert_eq!(
            tenth_score(&RaceResult::new("1".to_owned(), "Aston Martin".to_owned())),
            1
        );
        assert_eq!(
            tenth_score(&RaceResult::new("19".to_owned(), "Aston Martin".to_owned())),
            1
        );
        assert_eq!(
            tenth_score(&RaceResult::new("R".to_owned(), "Aston Martin".to_owned())),
            0
        );
    }
}
