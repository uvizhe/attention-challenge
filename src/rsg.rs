use rand::prelude::*;

const SIGNAL_COUNT: usize = 5;
const MINIMUM_T_MIN: usize = 30;
const MAXIMUM_T_MIN: usize = 60;

/// Generates random times for signals.
pub fn generate_random_signals(active_session_duration: usize, session_delay: usize) -> Vec<usize> {
    /*
      We suppose that mean interval value (t_min+t_max)/2 == active_session_duration/signal_count
      from which follows that for active_session_duration=15*60, signal_count=5 and t_min=1*60
      t_max shoud be 5*60.
      Adding new signal to active_session_duration shortens (remaining) active_session_duration
      and decreases (remaining) signal_count, and we should reassess t_max and t_min (the former
      can't be more than initial, the latter can't be less than initial).
    */
    let mut rng = thread_rng();

    let t_min = t_min_for_duration(active_session_duration);
    let mut double_mean = 2 * active_session_duration / SIGNAL_COUNT;
    let t_max = double_mean - t_min;

    let mut signals: Vec<usize> = Vec::new();
    let mut time_remaining = active_session_duration;
    let mut signals_left = SIGNAL_COUNT;
    let mut next_t_min = t_min;
    let mut next_t_max;
    // Generate random intervals
    for _ in 1..SIGNAL_COUNT {
        double_mean = 2 * time_remaining / signals_left;
        next_t_max = double_mean - next_t_min;
        while next_t_max > t_max {
            // Adjust values
            next_t_min += 1;
            next_t_max -= 1;
        }
        let interval = rng.gen_range(next_t_min..=next_t_max);
        time_remaining -= interval;
        signals.push(interval);
        signals_left = SIGNAL_COUNT - signals.len();
    }
    // Shuffle intervals
    signals.as_mut_slice().shuffle(&mut rng);
    // Add the last signal and apply session_delay
    signals.push(active_session_duration + session_delay);
    if session_delay > 0 {
        signals[0] += session_delay;
    }
    // Substitute interval lengths with timestamps
    for i in 1..signals.len() - 1 {
        signals[i] = signals[i - 1] + signals[i];
    }
    return signals;
}

/// Reduces minimum interval for short sessions to keep interval variability.
fn t_min_for_duration(session_duration: usize) -> usize {
    // Min duration where t_min is minimum
    let min_duration = MINIMUM_T_MIN * 10;
    let extra_duration = session_duration - min_duration;
    // Add 2 seconds for every 60 sec of extra duration
    let added_sec = extra_duration / 60 * 2;
    if MINIMUM_T_MIN + added_sec > MAXIMUM_T_MIN {
        MAXIMUM_T_MIN
    } else {
        MINIMUM_T_MIN + added_sec
    }
}
