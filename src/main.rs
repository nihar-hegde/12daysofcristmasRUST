fn days_intro(n: u8) {
    let day = match n {
        1 => "First",
        2 => "Sedond",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixth",
        7 => "Seventh",
        8 => "Eighth",
        9 => "Ninghth",
        10 => "Tenth",
        11 => "Eleventh",
        12 => "Twelvth",
        _ => "",
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love sent to me:",
        day
    );
}

fn gifts(n: u8, prefix: &str) {
    let gift_name = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };
    println!("{}{}", prefix, gift_name);
}

fn main() {
    println!("TWELVE DAYS OF CRISTMAS");

    for day in 1..13 {
        days_intro(day);

        for gift_day in (1..(day + 1)).rev() {
            gifts(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

// fn day_intro(n: u32) {
//     let day = match n {
//         1 => "first",
//         2 => "second",
//         3 => "third",
//         4 => "fourth",
//         5 => "fifth",
//         6 => "sixth",
//         7 => "seventh",
//         8 => "eighth",
//         9 => "ninth",
//         10 => "tenth",
//         11 => "eleventh",
//         12 => "twelfth",
//         _ => "",
//     };
//
//     println!(
//         "\nOn the {} day of Crhistmas\n my true love sent to me:",
//         day
//     )
// }
//
// fn gifts(n: u32, prefix: &str) {
//     let gift_text = match n {
//         1 => "a Partridge in a Pear Tree",
//         2 => "Two turtle Doves",
//         3 => "Three French Hens",
//         4 => "Four Calling Birds",
//         5 => "Five Golden Bings",
//         6 => "Six Geese a Laying",
//         7 => "Seven Swans a Swimming",
//         8 => "Eight Maids a Milking",
//         9 => "Nine Ladies Dancing",
//         10 => "Ten Lords a Leaping",
//         11 => "Eleven Pipers Piping",
//         12 => "Twelve Drummers Drumming",
//         _ => "",
//     };
//     println!("{}{}", prefix, gift_text);
// }
//
// fn main() {
//     println!("TWELVE DAYS OF CHRISTMAS");
//
//     for day in 1..13 {
//         day_intro(day);
//
//         for gift_day in (1..(day + 1)).rev() {
//             gifts(
//                 gift_day,
//                 if gift_day == 1 && day != 1 {
//                     "and "
//                 } else {
//                     ""
//                 },
//             )
//         }
//     }
// }
