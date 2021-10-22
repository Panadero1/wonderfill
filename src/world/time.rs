use serde::{Deserialize, Serialize};


pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub const DAYS_PER_YEAR: u8 = 100;
pub const HOURS_PER_DAY: u8 = 12;

#[derive(Serialize, Deserialize)]
pub struct Clock {
    hour: u8,
    day: u8,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            hour: 0,
            day: 0,
        }
    }

    pub fn tick(&mut self) {
        self.hour += 1;
        self.format();
    }

    fn format(&mut self) {
        if self.hour >= HOURS_PER_DAY {
            self.hour -= HOURS_PER_DAY;
            self.day += 1;
        }
        if self.day > DAYS_PER_YEAR {
            self.day -= DAYS_PER_YEAR;
        }
    }

    pub fn get_hour(&self) -> u8 {
        self.hour
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }
}