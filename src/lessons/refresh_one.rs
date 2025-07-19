pub fn run_refresh_one() {
    println!("this is a refresh");

    let x = "gold";
    let y = 10;
    let pass = true;
    let pi = 3.14159265  3589793;
    const GRAVITY: f64 = 9.81;

    if x == "gold" {
        println!("You're rich")
    } else if x == "silver" {
        println!("You're NOT RICH");
    } else {
        println!("You're poor!");
    }

    match x {
        "gold" => {
            println!("You're rich")
        }
        "silver" => {
            println!("You're not rich")
        }
        _ => println!("You're poor"),
    }

    match x {
        "gold" => println!("You're rich"),
        "silver" => println!("You're NOT rich"),
        _ => println!("You're NOT RICH"),
    }

    speak("luffy");

    let names = ["Luffy", "Zorro"];
    println!("{:?}", names);

    for name in names.iter() {
        println!("{name}");
    }

    println!("{}", names[0]);

}

fn speak(word: &str) {
    println!("I am speaking as  {word}");
}