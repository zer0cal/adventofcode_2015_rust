// --- Day 9: All in a Single Night ---
#![allow(dead_code)]
use std::fs;

pub fn answer() {
    println!("Day 9: All in a Single Night");
    let input = fs::read_to_string("day9_input.txt").expect("err reading day 9 input");
    let (ans1, ans2) = shortest_distance(&input);
    println!("answer to pt 1 is {}", ans1);
    println!("answer to pt 2 is {}", ans2);
}

#[derive(Debug)]
struct Route {
    pub locatin1: String,
    pub locatin2: String,
    pub length: u32,
}

fn shortest_distance(s: &str) -> (u32, u32) {
    let mut routes = Vec::new();
    for line in s.split("\n") {
        let mut iter = line.split_whitespace();
        match (
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
        ) {
            (Some(left), Some("to"), Some(right), Some("="), Some(dist)) => routes.push(Route {
                locatin1: left.to_string(),
                locatin2: right.to_string(),
                length: dist.parse::<u32>().unwrap(),
            }),
            _ => (),
        }
    }
    let mut places: Vec<String> = Vec::new();
    let routes = routes;
    for route in routes.iter() {
        if !places.contains(&route.locatin1) {
            places.push(route.locatin1.to_string());
        }
    }
    places.push(routes.last().unwrap().locatin2.to_string());
    let places = places;
    let places_len = places.len();
    let mut places_indexes: Vec<usize> = (0..places_len).into_iter().collect();
    let mut min = get_dist(&places_indexes, &places, &routes);
    let mut max = get_dist(&places_indexes, &places, &routes);

    // Heap's algorithm
    let mut i = 1;
    let mut c: Vec<usize> = vec![0; places_len];
    while i < places_len {
        if c[i] < i {
            if i % 2 == 0 {
                let tmp = places_indexes[i];
                places_indexes[i] = places_indexes[0];
                places_indexes[0] = tmp;
            } else {
                let tmp = places_indexes[c[i]];
                places_indexes[c[i]] = places_indexes[i];
                places_indexes[i] = tmp;
            }
            let tmp_dist = get_dist(&places_indexes, &places, &routes);
            if tmp_dist < min {
                min = tmp_dist;
            }
            if tmp_dist > max {
                max = tmp_dist;
            }
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    (min, max)
}

fn get_dist(route_indexes: &Vec<usize>, places: &Vec<String>, routes: &Vec<Route>) -> u32 {
    let mut dist = 0;
    let slice1 = &route_indexes[..route_indexes.len() - 1];
    let slice2 = &route_indexes[1..];
    for (prev, next) in slice1.iter().zip(slice2) {
        if let Some(r) = routes
            .iter()
            .find(|x| x.locatin1 == places[*prev] && x.locatin2 == places[*next])
        {
            dist += r.length;
        } else if let Some(r) = routes
            .iter()
            .find(|x| x.locatin1 == places[*next] && x.locatin2 == places[*prev])
        {
            dist += r.length;
        } else {
            panic!("route {:?} not found", route_indexes);
        }
    }
    dist
}

#[cfg(test)]
mod tests {}
