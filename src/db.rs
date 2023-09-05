use js_sys::Date;
use serde::{Deserialize, Serialize};
use web_sys::Storage;

use crate::app::{VolumeLevel, INITIAL_DELAY, INITIAL_DURATION};

const LEGACY_STORAGE_KEYS: [&str; 8] = [
    "_config:tryout",
    "_config:startOfWeekDay",
    "_config:publicProfile",
    "_data:prevSyncTime",
    "_data:offlineSessions",
    "_data:lastSyncTime",
    "_data:lastActionTime",
    "auth-token",
];

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
            if let Some(mut value_min) = value {
                let prefix = LegacyStorageValues::NumberValue.prefix();
                if value_min.contains(prefix) {
                    value_min = value_min.strip_prefix(prefix).unwrap().to_string();
                    // Durations were stored in seconds
                    value_min.parse::<usize>().unwrap()
                } else {
                    value_min.parse::<usize>().unwrap() * 60
                }
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
        self.local_storage.set_item("_config:sessionDuration", &duration_min.to_string())
            .expect("Unable to writo to LocalStorage");
    }

    pub fn get_active_session_delay(&self) -> usize {
        if let Ok(value) = self.local_storage.get_item("_config:bellsDeferral") {
            if let Some(mut value_min) = value {
                let prefix = LegacyStorageValues::NumberValue.prefix();
                if value_min.contains(prefix) {
                    value_min = value_min.strip_prefix(prefix).unwrap().to_string();
                    // Durations were stored in seconds
                    value_min.parse::<usize>().unwrap()
                } else {
                    value_min.parse::<usize>().unwrap() * 60
                }
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
        self.local_storage.set_item("_config:bellsDeferral", &delay_min.to_string())
            .expect("Unable to writo to LocalStorage");
    }

    pub fn add_session(&self, session: Session) {
        let date = session.date();
        let ts = session.ts();
        // Use minutes
        let duration = session.duration / 60;
        let score = session.score;
        // Last session date
        let last_session_today = if let Ok(maybe_value) = self.local_storage.get_item("_data:lastSessionDate") {
            if let Some(mut last_date) = maybe_value {
                let prefix = LegacyStorageValues::StringValue.prefix();
                if last_date.contains(prefix) {
                    last_date = last_date.strip_prefix(prefix).unwrap().to_string();
                }
                last_date == date
            } else {
                false
            }
        } else {
            false
        };
        self.local_storage.set_item("_data:lastSessionDate", &date)
            .expect("Unable to writo to LocalStorage");
        // Number of sessions today
        let mut sessions_today = 1;
        if let Ok(maybe_value) = self.local_storage.get_item("_data:sessionsToday") {
            let mut value = maybe_value.unwrap_or("0".to_string());
            let prefix = LegacyStorageValues::NumberValue.prefix();
            if value.contains(prefix) {
                value = value.strip_prefix(prefix).unwrap().to_string();
            }
            // If the last session was today, add to the number. Otherwise set to 1
            if last_session_today {
                sessions_today = value.parse::<usize>().unwrap() + 1;
            }
            self.local_storage.set_item("_data:sessionsToday", &sessions_today.to_string())
                .expect("Unable to writo to LocalStorage");
        }
        // Sessions data
        if let Ok(maybe_value) = self.local_storage.get_item("_data:sessions") {
            let session = SavedSession {
                date,
                ts,
                duration,
                score,
            };
            let mut sessions: Vec<SavedSession> = if let Some(mut value) = maybe_value {
                let prefix = LegacyStorageValues::ObjectValue.prefix();
                if value.contains(prefix) {
                    value = value.strip_prefix(prefix).unwrap().to_string();
                }
                serde_json::from_str(&value).unwrap()
            } else {
                Vec::new()
            };
            sessions.push(session);
            self.local_storage.set_item(
                "_data:sessions",
                &serde_json::to_string(&sessions).unwrap(),
            ).expect("Unable to writo to LocalStorage");
        }
        // Averages date
        if let Ok(maybe_value) = self.local_storage.get_item("_data:avgs") {
            let mut avgs: Vec<f32> = if let Some(mut value) = maybe_value {
                let prefix = LegacyStorageValues::ObjectValue.prefix();
                if value.contains(prefix) {
                    value = value.strip_prefix(prefix).unwrap().to_string();
                }
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
            ).expect("Unable to writo to LocalStorage");
        }
    }

    pub fn get_avgs(&self) -> Vec<f32> {
        if let Ok(maybe_value) = self.local_storage.get_item("_data:avgs") {
            if let Some(mut value) = maybe_value {
                let prefix = LegacyStorageValues::ObjectValue.prefix();
                if value.contains(prefix) {
                    value = value.strip_prefix(prefix).unwrap().to_string();
                }
                serde_json::from_str(&value).unwrap()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    pub fn get_sound_volume(&self) -> VolumeLevel {
        if let Ok(value) = self.local_storage.get_item("_config:soundVolume") {
            if let Some(mut value) = value {
                let prefix = LegacyStorageValues::NumberValue.prefix();
                if value.contains(prefix) {
                    value = value.strip_prefix(prefix).unwrap().to_string();
                    let value = match value.parse::<usize>().unwrap() {
                        5 => 2,
                        4 => 1,
                        3 => 1,
                        2 => 0,
                        1 => 0,
                        _ => unimplemented!(),
                    };
                    VolumeLevel::from_config_value(value)
                } else {
                    VolumeLevel::from_config_value(value.parse::<usize>().unwrap())
                }
            } else {
                VolumeLevel::default()
            }
        } else {
            VolumeLevel::default()
        }
    }

    pub fn set_sound_volume(&self, volume: &VolumeLevel) {
        let value = volume.config_value();
        self.local_storage.set_item("_config:soundVolume", &value.to_string())
            .expect("Unable to writo to LocalStorage");
    }

    pub fn remove_legacy_keys(&self) {
        for key in LEGACY_STORAGE_KEYS {
            self.local_storage.remove_item(key).unwrap();
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
            "{}-{:02}-{:02}",
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

enum LegacyStorageValues {
    NumberValue,
    ObjectValue,
    StringValue,
}

impl LegacyStorageValues {
    pub fn prefix(&self) -> &str {
        match self {
            LegacyStorageValues::NumberValue => "__q_numb|",
            LegacyStorageValues::StringValue => "__q_strn|",
            LegacyStorageValues::ObjectValue => "__q_objt|",
        }
    }
}
