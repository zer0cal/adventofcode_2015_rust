pub fn answer() {
    let input = 36000000;
    let ans1 = pt1(input);
    println!("Answer to pt1 is {}", ans1);
    let ans2 = pt2(input);
    println!("Answer to pt2 is {}", ans2);
}

fn pt1(input: u32) -> u32 {
    let mut presents = 0;
    let mut current_house = 0;
    while presents < input {
        presents = 0;
        current_house += 1;
        for elf in 1..current_house + 1 {
            if current_house % elf == 0 {
                presents += elf * 10;
            }
        }
    }
    current_house
}

fn pt2(input: u32) -> u32 {
    let mut presents = 0;
    let mut current_house = 0;
    while presents < input {
        presents = 0;
        current_house += 1;
        for elf in 1..current_house + 1 {
            if current_house % elf == 0 && current_house / elf <= 50 {
                presents += elf * 11;
            }
        }
    }
    current_house
}
