use std::collections::HashMap;

// --- Day 14: Reindeer Olympics ---
pub fn answer() {
    let reindeers = vec![
        Reindeer::new("Vixen", 8, 8, 53),
        Reindeer::new("Blitzen", 13, 4, 49),
        Reindeer::new("Rudolph", 20, 7, 132),
        Reindeer::new("Cupid", 12, 4, 43),
        Reindeer::new("Donner", 9, 5, 38),
        Reindeer::new("Dasher", 10, 4, 37),
        Reindeer::new("Comet", 3, 37, 76),
        Reindeer::new("Prancer", 9, 12, 97),
        Reindeer::new("Dancer", 37, 1, 36),
    ];
    println!("Day 14: Reindeer Olympics");
    what_dist_has_the_winning_reindeer(2503, &reindeers);
    how_many_points_does_the_winning_reindeer_have(2503, &reindeers);
}

struct Reindeer<'a> {
    name: &'a str,
    fly_speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer<'_> {
    pub fn new(name: &str, fly_speed: u32, fly_time: u32, rest_time: u32) -> Reindeer {
        Reindeer {
            name,
            fly_speed,
            fly_time,
            rest_time,
        }
    }
    fn calc_dist_of_time(&self, time: u32) -> u32 {
        let time_resting_and_flying = self.fly_time + self.rest_time;
        let full_cycles = time / time_resting_and_flying;
        let dist_flight_in_full_cycles = full_cycles * (self.fly_speed * self.fly_time);
        let time_left_after_full_cycles = time - (time_resting_and_flying * full_cycles);
        let dist_in_left_time =
            (self.fly_speed * self.fly_time).min(time_left_after_full_cycles * self.fly_speed);

        dist_flight_in_full_cycles + dist_in_left_time
    }
}

fn what_dist_has_the_winning_reindeer(time: u32, reindeers: &[Reindeer]) {
    let mut dists: HashMap<&str, u32> = HashMap::new();
    for reindeer in reindeers.iter() {
        dists.insert(reindeer.name, reindeer.calc_dist_of_time(time));
    }
    let max = dists.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("{}: {}", max.0, max.1);
}

fn how_many_points_does_the_winning_reindeer_have(time: u32, reindeers: &[Reindeer]) {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    let mut dists: HashMap<&str, u32> = HashMap::new();
    for t in 1..=time {
        for reindeer in reindeers.iter() {
            dists.insert(reindeer.name, reindeer.calc_dist_of_time(t));
        }
        let max = dists.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        scores
            .entry(max.0)
            .and_modify(|score| *score += 1)
            .or_insert(1);
        dists.clear();
    }
    println!("{:#?}", scores);
    let max = scores.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("{}: {}", max.0, max.1);
}
