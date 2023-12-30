fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);            //becomes unsafe if we move read(x) above the let x
}