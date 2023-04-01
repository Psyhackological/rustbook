pub const NUM_DAYS: usize = 12;

pub const DAYS: [&str; NUM_DAYS] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

pub const GIFTS: [&str; NUM_DAYS] = [
    "a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

pub fn print_lyrics_for_day(day_index: usize) {
    println!("On the {} day of Christmas,", DAYS[day_index]);
    println!("my true love sent to me");

    for gift_index in (0..=day_index).rev() {
        if gift_index == 0 && day_index > 0 {
            print!("And ");
        }

        if gift_index == 0 && day_index == 0 {
            print!("A");
            print!("{}", &GIFTS[gift_index][1..]);
        } else {
            print!("{}", GIFTS[gift_index]);
        }

        if gift_index > 0 {
            println!(",");
        }
    }

    if day_index == NUM_DAYS - 1 {
        println!("!");
    } else {
        println!(".\n");
    }
}
