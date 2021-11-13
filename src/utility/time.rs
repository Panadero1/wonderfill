use std::time::{self, Instant};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NInstant(time::Instant);

impl Serialize for NInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl<'de> Deserialize<'de> for NInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bool(NInstantVisitor)?;
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

struct NInstantVisitor;

impl<'de> Visitor<'de> for NInstantVisitor {
    type Value = NInstant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bool")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NInstant::now())
    }
}
