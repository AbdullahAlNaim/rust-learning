use std::ffi::c_int;

pub fn run_data_types() {
    let some_letter: char = '😁';

    let n: u32 = 10000;
    let m: i32 = -100000;

    let a_num: usize = 34234;
    let b_bum: isize = -123123123;

    let _numbers = [1,2,3,4,5];

    let names = ["luffy", "sanji", "zoro", "nami"];
    println!("{:?}", names);

    let first = names[0];
    println!("{first}");

    let mut other_names = ["robin", "brook", "chopper"];
    other_names[1] = "usopp";
    println!("{}", other_names[1]);

    println!("{:?}", names);
    println!("{names:?}");
    println!("{names:#?}");

    dbg!(other_names);

    let character = ("luffy", 22, "emperor", true);
    println!("{character:?}");
    // let name = character.0;
    // let age = character.1;
    // let status = character.2;
    // let isString = character.3;

    let (name, age, role, isStrong) = character;
    println!("Name: {name}, age: {age}, role: {role}, isStrong: {isStrong}");

    let days = 1..=7;
    println!("{days:?}");

    for number in days {
        println!("{number}");
    }

    let letters = 'b'..'f';
    for letter in letters {
        println!("{letter}");
    }

    let month_days: std::ops::Range<i32> = 1..31;
    let some_letters: std::ops::Range<char> = 'a'..'z';

    let num = 1_337;
    let new_num = num as i16;

    let pi = 3.14159;
    println!("{pi:.3}");
    println!("{pi:.3}");

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;

    let is_acceptable_coffee = with_milk || with_sugar;

    let small_nums: [i8; 4] = [1,2,4,5];
    println!("{small_nums:?}");

    let something = (10, 9.81, true, small_nums);
    println!("{something:#?}");
    println!("{}", something.1);

    println!("{some_letter}");


}