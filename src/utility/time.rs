use std::time::{self, Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct NInstant(time::Instant);

impl Serialize for NInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_bool(true)
    }
}

impl<'de> Deserialize<'de> for NInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(NInstant(Instant::now()))
    }
}

impl Into<Instant> for NInstant {
    fn into(self) -> Instant {
        self.0
    }
}

impl NInstant {
    pub fn now() -> NInstant {
        NInstant(Instant::now())
    }
    pub fn get_instant(&self) -> Instant {
        self.0
    }
}