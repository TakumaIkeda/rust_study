fn main() {
    let serial_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for (i, n) in serial_numbers.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", n);
        for j in (0..i + 1).rev() {
            let gift = match j {
                0 => "A Partridge in a Pear Tree\n",
                1 => "two Turtle Doves",
                2 => "three French Hens",
                3 => "four Calling Birds",
                4 => "five Gold Rings",
                5 => "six Geese-a-Laying",
                6 => "seven Swans-a-Swimming",
                7 => "eight Maids-a-Milking",
                8 => "nine Ladies Dancing",
                9 => "ten Lords-a-Leaping",
                10 => "eleven Pipers Piping",
                11 => "twelve Drummers Drumming",
                _ => unreachable!(),
            };
            if i == 0 {
                println!("{}.", gift);
            } else if j == 0 {
                println!("and {}.", gift);
            } else {
                println!("{}", gift);
            }
        }
    }
}
