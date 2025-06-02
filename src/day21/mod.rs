use std::fs;

pub fn answer() {
    let file = fs::read_to_string("src/day21/input.txt").unwrap();
    run_games(&file);
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    hp: i32,
    damage: i32,
    armor: i32,
}

struct Item {
    _name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Entity {
    pub fn new_player() -> Entity {
        Entity {
            hp: 100,
            damage: 0,
            armor: 0,
        }
    }

    pub fn new_boss(input: &str) -> Entity {
        let values: Vec<i32> = input
            .lines()
            .map(|line| line.split(':'))
            .map(|mut split| split.next_back().unwrap())
            .map(|value| value.trim())
            .map(|value| value.parse::<i32>().unwrap())
            .collect();

        Entity {
            hp: values[0],
            damage: values[1],
            armor: values[2],
        }
    }

    pub fn is_alife(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    pub fn calc_damage(&self, enemy: &Entity) -> i32 {
        let damage = enemy.damage - self.armor;
        if damage > 0 {
            damage
        } else {
            1
        }
    }

    pub fn add_item(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }
}

impl Item {
    pub fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            _name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }

    pub fn get_weapons() -> Vec<Item> {
        vec![
            Item::new("Dagger", 8, 4, 0),
            Item::new("Shortsword", 10, 5, 0),
            Item::new("Warhammer", 25, 6, 0),
            Item::new("Longsword", 40, 7, 0),
            Item::new("Greataxe", 74, 8, 0),
        ]
    }

    pub fn get_armors() -> Vec<Item> {
        vec![
            Item::new("None", 0, 0, 0),
            Item::new("Leather", 13, 0, 1),
            Item::new("Chainmail", 31, 0, 2),
            Item::new("Splintmail", 53, 0, 3),
            Item::new("Bandedmail", 75, 0, 4),
            Item::new("Platemail", 102, 0, 5),
        ]
    }

    pub fn get_rings() -> Vec<Item> {
        vec![
            Item::new("None", 0, 0, 0),
            Item::new("Damage +1", 25, 1, 0),
            Item::new("Damage +2", 50, 2, 0),
            Item::new("Damage +3", 100, 3, 0),
            Item::new("Defense +1", 20, 0, 1),
            Item::new("Defense +2", 40, 0, 2),
            Item::new("Defense +3", 80, 0, 3),
        ]
    }
}

enum RoundResult {
    PlayerWin,
    BossWin,
    KeepPlaying,
}

fn play_round(player: &mut Entity, boss: &mut Entity) -> RoundResult {
    let boss_damage = boss.calc_damage(player);
    boss.take_damage(boss_damage);
    if !boss.is_alife() {
        return RoundResult::PlayerWin;
    }
    let player_damage = player.calc_damage(boss);
    player.take_damage(player_damage);
    if !player.is_alife() {
        return RoundResult::BossWin;
    }
    RoundResult::KeepPlaying
}

fn calc_cost(items: &Vec<&Item>) -> i32 {
    items.iter().map(|item| item.cost).sum()
}

enum GameResult {
    Success,
    Failure,
}

fn play_game(player: Entity, boss: Entity, items: &Vec<&Item>) -> GameResult {
    let mut player = player;
    let mut boss = boss;
    items.iter().for_each(|item| player.add_item(item));
    loop {
        let round_result = play_round(&mut player, &mut boss);
        match round_result {
            RoundResult::PlayerWin => {
                return GameResult::Success;
            }
            RoundResult::BossWin => {
                return GameResult::Failure;
            }
            _ => (),
        }
    }
}

struct RingGetter {
    len: isize,
    ring_one_index: isize,
    ring_two_index: isize,
}

impl RingGetter {
    pub fn new(len: isize) -> RingGetter {
        RingGetter {
            len,
            ring_one_index: -1,
            ring_two_index: 0,
        }
    }

    pub fn get_next(&mut self) -> Option<(usize, usize)> {
        self.ring_one_index += 1;
        if self.ring_one_index == self.len {
            self.ring_one_index = 0;
            self.ring_two_index += 1;
        }
        if self.ring_two_index == self.len {
            return None;
        }
        Some((self.ring_one_index as usize, self.ring_two_index as usize))
    }
}

fn run_games(input: &str) {
    let player = Entity::new_player();
    let boss = Entity::new_boss(input);
    let weapons = Item::get_weapons();
    let armors = Item::get_armors();
    let rings = Item::get_rings();
    let mut min_win_cost = i32::MAX;
    let mut max_lose_cost = i32::MIN;
    for weapon in weapons.iter() {
        for armor in armors.iter() {
            let mut ringgetter = RingGetter::new(rings.len() as isize);
            while let Some((r, l)) = ringgetter.get_next() {
                let items = vec![weapon, armor, &rings[r], &rings[l]];
                let game_result = play_game(player, boss, &items);
                match game_result {
                    GameResult::Success => {
                        let cost = calc_cost(&items);
                        // println!("Player wins; cost of items: {}", cost);
                        if cost < min_win_cost {
                            min_win_cost = cost;
                        }
                    }
                    GameResult::Failure => {
                        let cost = calc_cost(&items);
                        // println!("Player loses; cost of items: {}", cost);
                        if cost > max_lose_cost {
                            max_lose_cost = cost;
                        }
                    }
                }
            }
        }
    }
    println!("Player wins with min cost of items: {}", min_win_cost);
    println!("Player loses with max cost of items: {}", max_lose_cost);
}
