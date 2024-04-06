// Conditionals - Used to check the condition of something and act accordingly

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // if/else
    if age > 21 {
        println!("bartender: what world yo like to drink?")
    } else {
        println!("bartender: Sorry, you have to leave")
    }

    // if/else if/else
    if age > 21 && (check_id || knows_person_of_age) {
        println!("bartender: what world yo like to drink?")
    } else if age < 21 && check_id {
        println!("bartender: Sorry, you have to leave")
    } else {
        println!("bartender: I will need to see your id")
    }

    // Shorthand IF
    let is_age_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_age_age);
}
