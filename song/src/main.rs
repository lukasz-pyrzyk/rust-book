fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
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
        "Twelve drummers drumming"
    ];

    for day in 0..12 {
        println!("On the {} of Christmas, my true love gave to me:", ordinals[day]);

        for gift_index in (0..=day).rev() {
            println!("{}", gifts[gift_index]);
        }

        println!();
    }
}
