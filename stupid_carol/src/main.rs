fn main() {
    let gifts = [
        "1 Partridge in a pear tree",
        "2 turtle doves",
        "3 french hens",
        "4 colly birds",
        "5 golden rings",
        "6 geese-a-laying",
        "7 swans-a-swimming",
        "8 maids-a-milking",
        "9 ladies dancing",
        "10 lords-a-leaping",
        "11 pipers piping",
        "12 drummer drumming",
    ];

    for (days, gifts) in gifts.iter().enumerate() {
        let correct_day = days + 1;
        println!("On the {correct_day} day of Christmas, my true love gave to me: {gifts}");
    }
}
