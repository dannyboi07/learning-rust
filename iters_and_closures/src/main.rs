#[derive(Debug, Clone, Copy, PartialEq)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red: i32 = 0;
        let mut num_blue: i32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

fn main() {
    println!("Hello, world!");

    let store: Inventory = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1: Option<ShirtColour> = Some(ShirtColour::Red);
    let giveaway1: ShirtColour = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    maps();
}

fn maps() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

# [derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Using closure that captures it's environment
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}
