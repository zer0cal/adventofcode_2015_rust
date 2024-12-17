// --- Day 2: I Was Told There Would Be No Math ---

#![allow(dead_code)]

use std::fs;

pub fn answer() {
    println!("Day 2: I Was Told There Wiuld Be No Match");
    let input = fs::read_to_string("day2_input.txt").expect("err reading day 2 input");
    let mut ans = 0;
    let mut ans2 = 0;
    for line in input.split_whitespace() {
        let splt: std::str::Split<'_, &str> = line.split("x");
        let v: Vec<i32> = splt.map(|x| x.parse::<i32>().unwrap()).collect();
        let l = v.get(0).unwrap();
        let w = v.get(1).unwrap();
        let h = v.get(2).unwrap();
        ans += square_feet_of_wrapping_paper(l, w, h);
        ans2 += feet_of_ribbon(l, w, h);
    }
    println!("answer to pt 1 is {}", ans);
    println!("answer to pt 2 is {}", ans2);
}

fn square_feet_of_wrapping_paper(l: &i32, w: &i32, h: &i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let min = lw.min(wh).min(hl);

    2 * lw + 2 * wh + 2 * hl + min
}

fn feet_of_ribbon(l: &i32, w: &i32, h: &i32) -> i32 {
    let first = l.min(w).min(h);
    let second = middle_of_three(l, w, h);
    first * 2 + second * 2 + l * w * h
}

fn middle_of_three<'l>(a: &'l i32, b: &'l i32, c: &'l i32) -> &'l i32 {
    if a > b && b > c || a < b && b < c {
        b
    } else if b > a && a > c || b < a && a < c {
        a
    } else if a > c && c > b || a < c && c < b {
        c
    } else if a == b {
        a
    } else if a == c {
        c
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::feet_of_ribbon;
    use super::middle_of_three;
    use super::square_feet_of_wrapping_paper;

    #[test]
    fn first() {
        let res = square_feet_of_wrapping_paper(&2, &3, &4);
        let exp = 58;
        assert_eq!(res, exp);
    }

    #[test]
    fn second() {
        let res = square_feet_of_wrapping_paper(&1, &1, &10);
        let exp = 43;
        assert_eq!(res, exp);
    }

    #[test]
    fn first_pt2() {
        let res = feet_of_ribbon(&2, &3, &4);
        let exp = 34;
        assert_eq!(res, exp);
    }

    #[test]
    fn second_pt2() {
        let res = feet_of_ribbon(&1, &1, &10);
        let exp = 14;
        assert_eq!(res, exp);
    }

    #[test]
    fn third_pt2() {
        let res = feet_of_ribbon(&10, &1, &1);
        let exp = 14;
        assert_eq!(res, exp);
    }

    #[test]
    fn fourth_pt2() {
        let res = feet_of_ribbon(&11, &1, &11);
        let exp = 145;
        assert_eq!(res, exp);
    }

    #[test]
    fn middle_of_three_1() {
        assert_eq!(middle_of_three(&1, &2, &3), &2, "1 2 3");
        assert_eq!(middle_of_three(&1, &1, &3), &1, "1 1 3");
        assert_eq!(middle_of_three(&3, &2, &1), &2, "3 2 1");
        assert_eq!(middle_of_three(&1, &1, &1), &1, "1 1 1");
        assert_eq!(middle_of_three(&2, &1, &3), &2, "2 1 3");
    }
}
