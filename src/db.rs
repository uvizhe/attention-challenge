use js_sys::Date;
use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};

use crate::app::{INITIAL_DELAY, INITIAL_DURATION};

pub struct Db {
    local_storage: Storage,
}

impl Db {
    pub fn new() -> Self {
        let window = web_sys::window().expect("Window not available");
        let local_storage = window.local_storage()
            .expect("LocalStorage not available")
            .expect("LocalStorage not defined");

        Self { local_storage }
    }

    pub fn get_session_duration(&self) -> usize {
        if let Ok(value) = self.local_storage.get_item("_config:sessionDuration") {
            if let Some(value_min) = value {
                value_min.parse::<usize>().unwrap() * 60
            } else {
                INITIAL_DURATION
            }
        } else {
            INITIAL_DURATION
        }
    }

    pub fn set_session_duration(&self, duration: usize) {
        // Use minutes
        let duration_min = duration / 60;
        self.local_storage.set_item("_config:sessionDuration", &duration_min.to_string()).unwrap();
    }

    pub fn get_active_session_delay(&self) -> usize {
        if let Ok(value) = self.local_storage.get_item("_config:bellsDeferral") {
            if let Some(value_min) = value {
                value_min.parse::<usize>().unwrap() * 60
            } else {
                INITIAL_DELAY
            }
        } else {
            INITIAL_DELAY
        }
    }

    pub fn set_active_session_delay(&self, delay: usize) {
        // Use minutes
        let delay_min = delay / 60;
        self.local_storage.set_item("_config:bellsDeferral", &delay_min.to_string()).unwrap();
    }

    pub fn add_session(&self, session: Session) {
        let date = session.date();
        let ts = session.ts();
        // Use minutes
        let duration = session.duration / 60;
        let score = session.score;
        // Last session date
        let last_session_today = if let Ok(maybe_value) = self.local_storage.get_item("_data:lastSessionDate") {
            if let Some(last_date) = maybe_value {
                last_date == date
            } else {
                false
            }
        } else {
            false
        };
        self.local_storage.set_item("_data:lastSessionDate", &date).unwrap();
        // Number of sessions today
        let mut sessions_today = 1;
        if let Ok(maybe_value) = self.local_storage.get_item("_data:sessionsToday") {
            let value = maybe_value.unwrap_or("0".to_string());
            // If the last session was today, add to the number. Otherwise set to 1
            if last_session_today {
                sessions_today = value.parse::<usize>().unwrap() + 1;
            }
            self.local_storage.set_item("_data:sessionsToday", &sessions_today.to_string());
        }
        // Sessions data
        if let Ok(maybe_value) = self.local_storage.get_item("_data:sessions") {
            let session = SavedSession {
                date,
                ts,
                duration,
                score,
            };
            let mut sessions: Vec<SavedSession> = if let Some(value) = maybe_value {
                serde_json::from_str(&value).unwrap()
            } else {
                Vec::new()
            };
            sessions.push(session);
            self.local_storage.set_item(
                "_data:sessions",
                &serde_json::to_string(&sessions).unwrap(),
            );
        }
        // Averages date
        if let Ok(maybe_value) = self.local_storage.get_item("_data:avgs") {
            let mut avgs: Vec<f32> = if let Some(value) = maybe_value {
                serde_json::from_str(&value).unwrap()
            } else {
                Vec::new()
            };
            let new_value = if last_session_today {
                let last_value = avgs.pop().unwrap();
                (last_value * (sessions_today as f32 - 1.0) + score as f32) / sessions_today as f32
            } else {
                score as f32
            };
            avgs.push(new_value);
            self.local_storage.set_item(
                "_data:avgs",
                &serde_json::to_string(&avgs).unwrap(),
            );
        }
    }

    pub fn get_avgs(&self) -> Vec<f32> {
        if let Ok(maybe_value) = self.local_storage.get_item("_data:avgs") {
            if let Some(value) = maybe_value {
                serde_json::from_str(&value).unwrap()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}

pub struct Session {
    date: Date,
    pub duration: usize,
    pub score: usize,
}

impl Session {
    pub fn new(date: Date, duration: usize, score: usize) -> Self {
        Self {
            date,
            duration,
            score,
        }
    }

    pub fn ts(&self) -> usize {
        (self.date.get_time() / 1000.0) as usize
    }

    pub fn date(&self) -> String {
        format!(
            "{}-{:02}-{}",
            self.date.get_full_year(),
            self.date.get_month() + 1,
            self.date.get_date(),
        )
    }
}

/// Session representation in a database
#[derive(Deserialize, Serialize, Debug)]
struct SavedSession {
    date: String,
    ts: usize,
    duration: usize,
    score: usize,
}
