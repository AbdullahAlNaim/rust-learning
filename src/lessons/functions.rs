pub fn run_functions() {

    println!("this is a function");

    something("chakra");

    provide_num(5);

    let exp = some_num(10);
    println!("{exp}");


    let result = mystery();

    apply_to_jobs(100, "Rust Developer");

    is_even(3);

    let results = alphabets("azir");
    let one = results.0;
    let two = results.1;
    println!("({one}, {two})")
}

fn alphabets(word: &str) -> (bool,bool) {
    let first = word.contains('a');
    let second = word.contains('z');

    (first, second)
}

fn is_even(num: i32) -> bool {
    num % 2 == 0 
}

fn apply_to_jobs(num: i32, title: &str) {
    println!("I'm applying to {num} {title} jobs")
}

fn something(word: &str) {
    println!("{word} was input by the user")
}

fn provide_num(num: i32) -> i32 {
    return num * 2;
}

fn some_num(num: i32) -> i32 {
    num * num
}

fn mystery() {}