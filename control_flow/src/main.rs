fn main() {
    let number: i32 = 91;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    count();
    alrighty()
}

fn count() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            counter = counter * 2;
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("the counter is {counter}")
}

fn alrighty() {

    let mut x = 0;

    'a: loop {

        x += 1;

        'b: loop {

            if x > 10 {

                continue 'a;

            } else {

                break 'b;

            }      

        }

        break;       

    }

}