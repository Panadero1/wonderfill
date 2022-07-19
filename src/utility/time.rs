use std::time::{self, Instant};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NInstant(time::Instant);

impl Serialize for NInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("0")
    }
}

impl<'de> Deserialize<'de> for NInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NInstantVisitor)
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

struct NInstantVisitor;

impl<'de> Visitor<'de> for NInstantVisitor {
    type Value = NInstant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("str")
    }

    fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NInstant::now())
    }
}
