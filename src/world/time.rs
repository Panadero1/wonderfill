use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub const DAYS_PER_YEAR: u16 = 100;
pub const HOURS_PER_DAY: u8 = 12;

#[derive(Debug, Serialize, Deserialize)]
pub struct Clock {
    hour: u8,
    day: u16,
    year: u16,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            hour: 0,
            day: 0,
            year: 0
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
        if self.day >= DAYS_PER_YEAR {
            self.day -= DAYS_PER_YEAR;
        }
        // println!("{:?} | {}", self.get_season(), self.day);
        // Seasons should be functional
    }

    pub fn get_hour(&self) -> u8 {
        self.hour
    }

    pub fn get_day(&self) -> u16 {
        self.day
    }

    pub fn get_year(&self) -> u16 {
        self.year
    }

    pub fn get_season(&self) -> Season {
        let q1 = DAYS_PER_YEAR / 4;
        let q2 = DAYS_PER_YEAR / 2;
        let q3 = 3 * DAYS_PER_YEAR / 4;
        let q4 = DAYS_PER_YEAR;
        if (0..q1).contains(&self.day) {
            Season::Spring
        }
        else if (q1..q2).contains(&self.day) {
            Season::Summer
        }
        else if (q2..q3).contains(&self.day) {
            Season::Autumn
        }
        else if (q3..q4).contains(&self.day) {
            Season::Winter
        }
        else {
            panic!("Days system is wrong: day = {}", &self.day);
        }
    }
}