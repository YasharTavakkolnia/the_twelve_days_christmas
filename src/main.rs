fn main() {
    let numbers = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let mut counter = 0;
    loop {
        
        if counter >= days.len() {
            break;
        }

        
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[counter]
        );

        let mut inner_counter = counter;
        loop {
            if inner_counter <= 0 {
                println!("{}", numbers[0]);
                break;
            } else {
                println!("{}", numbers[inner_counter]);
                inner_counter -= 1;
            }
        }
        println!();

        counter += 1;
    }
}
