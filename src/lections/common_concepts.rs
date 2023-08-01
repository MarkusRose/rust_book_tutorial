pub fn run() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value inner scope: {x}");
    }

    println!("The value outside of scope: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}")
}
