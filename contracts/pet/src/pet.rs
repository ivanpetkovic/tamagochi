use serde::{Serialize, Deserialize};

use crate::common::Minutes;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pet {
    pub name: String,
    pub color: String,
    pub last_feed_time: u64,
    pub satiated_interval: Minutes,
    pub starving_interval: Minutes,
}

impl Pet {
    pub fn new(last_fed: u64, satiated: Minutes, starving: Minutes, name: Option<&str>) -> Self {
        Pet {
            name: name.unwrap_or("Johny Doe").to_string(),
            color: "white".to_string(),
            last_feed_time: last_fed,
            satiated_interval: satiated,
            starving_interval: starving,
        }
    }

    pub fn is_dead(self: &Self, current_time: u64) -> bool {
        self.last_feed_time
            + to_seconds(self.satiated_interval)
            + to_seconds(self.starving_interval)
            < current_time
    }

    pub fn is_hungry(self: &Self, current_time: u64) -> bool {
        self.last_feed_time + to_seconds(self.satiated_interval) < current_time
    }
}

fn to_seconds(interval: Minutes) -> u64 {
    (interval * 60) as u64
}
