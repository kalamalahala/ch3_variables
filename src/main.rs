fn main() {
    let dayth = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let thing = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "FIVE~ GOLDEN~ RINGS~",
        "six geese-a-layin'",
        "seven swans-a-swimmin'",
        "eight maids-a-milkin'",
        "nine ladies dancin'",
        "ten lords-a-leapin'",
        "eleven pipers pipin'",
        "twelve drummers drummin'"
    ];

    // let day_count = dayth.len();
    let mut current_day = 0;

    for day in dayth {

        println!("On the {day} day of Christmas my true love gave to me:");

        if current_day > 0 {
            for gift in (0..(current_day + 1)).rev() {

                if gift < 1 {
                    println!("and {}", thing[gift]);
                    break;
                }

                println!("{}", thing[gift]);
            }
        } else {
            println!("{}", thing[0]);
        }
        
        current_day = current_day + 1;
    }

}