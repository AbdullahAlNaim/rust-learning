pub fn run_control_flow() {

    let test = 5;

    if test < 10 {
        println!("num is less than max 10");
    } else if test < 20 {
        println!("num is less than max 20");
    } else {
        println!("num is too high");
    }

    let state = true;

    match state {
        true => {
            println!("its one");
        }
        false => {
            println!("its two");
        }
    }

    let passed = false;

    match passed {
        true => println!("you passed"),
        false => println!("you failed")
    }

    let grade = 100;

    match grade {
        100 => println!("Its a perfect grade"),
        _ => println!("its a not perfect so its a terrible grade"),
    }

    let num = 9;

    match num {
        value if value % 2 == 0 => println!("number is even"),
        value if value % 2 == 1 => println!("number is odd"),
        _ => unreachable!(),
    }
}

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green" {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => return 1,
        "green" => return 2,
        "blue" => return 3,
        _ => return 0,
    }
}

fn factorial(mut num: i32) -> i32 {
    let mut answer = 0;

    if num <= 0 {
        return answer;
    } else {
        num = num * (num - 1);
        println!("{num}");
        factorial(num)
    }
}
