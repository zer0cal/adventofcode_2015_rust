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

    fn dist_over_time(&self, time: u32) -> u32 {
        let time_both = self.fly_time + self.rest_time;
        let cycles = time / time_both;
        let dist = cycles * (self.fly_speed * self.fly_time);
        let time_left = time - (time_both * cycles);
        let dist_left = (self.fly_speed * self.fly_time).min(time_left * self.fly_speed);

        dist + dist_left
    }
}

fn what_dist_has_the_winning_reindeer(time: u32, reindeers: &[Reindeer]) {
    let mut dists: HashMap<&str, u32> = HashMap::new();
    for reindeer in reindeers.iter() {
        dists.insert(reindeer.name, reindeer.dist_over_time(time));
    }
    let max = dists.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("{}: {}", max.0, max.1);
}

fn how_many_points_does_the_winning_reindeer_have(time: u32, reindeers: &[Reindeer]) {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    let mut dists: HashMap<&str, u32> = HashMap::new();
    for t in 1..=time {
        for reindeer in reindeers.iter() {
            dists.insert(reindeer.name, reindeer.dist_over_time(t));
        }
        let max = dists.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        scores
            .entry(max.0)
            .and_modify(|score| *score += 1)
            .or_insert(1);
    }
    println!("{:#?}", scores);
    let max = scores.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("{}: {}", max.0, max.1);
}

#[cfg(test)]
mod tests {
    use super::Reindeer;

    #[test]
    fn dist_over_time_test() {
        let reindeer = Reindeer::new("Test", 1, 1, 1);
        let dist = reindeer.dist_over_time(10);
        let expected_dist = 5;
        assert_eq!(expected_dist, dist);
    }

    #[test]
    fn dist_over_time_test_2() {
        let reindeer = Reindeer::new("Test", 2, 2, 8);
        let dist = reindeer.dist_over_time(10);
        let expected_dist = 4;
        assert_eq!(expected_dist, dist);
    }
}
