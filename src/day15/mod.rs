// --- Day 15: Science for Hungry People ---

pub fn answer() {
    let ingridents = vec![
        Ingrident {
            name: "Sprinkles",
            capacity: 2,
            durability: 0,
            flavor: -2,
            texture: 0,
            calories: 3,
        },
        Ingrident {
            name: "Butterscotch",
            capacity: 0,
            durability: 5,
            flavor: -3,
            texture: 0,
            calories: 3,
        },
        Ingrident {
            name: "Chocolate",
            capacity: 0,
            durability: 0,
            flavor: 5,
            texture: -1,
            calories: 8,
        },
        Ingrident {
            name: "Candy",
            capacity: 0,
            durability: -1,
            flavor: 0,
            texture: 5,
            calories: 8,
        },
    ];
}

struct Ingrident<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Teaspoon<'a, 'b> {
    number: u32,
    ingrident: &'a Ingrident<'b>,
}

pub fn calc_total_score(teaspoons: Vec<Teaspoon>) -> i32 {
    let (c, d, f, t) = teaspoons
        .iter()
        .map(|x| {
            (
                x.ingrident.capacity * x.number as i32,
                x.ingrident.durability * x.number as i32,
                x.ingrident.flavor * x.number as i32,
                x.ingrident.texture * x.number as i32,
            )
        })
        .fold((0, 0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
        });
    c * d * f * t
}
