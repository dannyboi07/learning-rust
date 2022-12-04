// const BASE_LYRICS: &str = "On the first day of Chrismas \nMy good friends brought to me";
const ORDINAL_MAPPER: [&str; 13] = [
    "", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelth",
];
const COUNT_MAPPER: [&str; 12] = [
    "A song and a Christmas tree",
    "Two candy canes",
    "Three boughs of holly",
    "Four coloured lights",
    "A shining star",
    "Little silver bells",
    "Candles a-glowing",
    "Gold and silver tinsel",
    "A guardian angel",
    "Some mistletoe",
    "Gifts for one and all",
    "All their good wishes"    
];

fn main() {
    let mut lyrics: Vec<String> = Vec::new();

    for i in 1..ORDINAL_MAPPER.len() {
        let first_two: String = format!(
            "On the {day} of Christmas\nMy good friends brought to me",
            day = ORDINAL_MAPPER[i]
        );

        let mut rest_of_para = String::new();
        for j in (0..i).rev() {
            rest_of_para = format!("{prev}\n{new}", prev=rest_of_para, new=COUNT_MAPPER[j]);
        }

        let ans_to_push: String = format!(
            "{first_two}{rest_of_para}\n",
            first_two = first_two,
            rest_of_para = rest_of_para
        );

        lyrics.push(ans_to_push);
    }

    for elem in lyrics {
        println!("{elem}");
    }
}
