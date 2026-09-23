use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // test 1 - accessing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // Print Blue
    println!("Blue score: {score}");

    // test 2 - loop example
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // test 3 - ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // OK I WILL
    println!("{0}", field_name);
    println!("{0}", field_value);
    // error[E0382]: borrow of moved value: `field_name`
    // error[E0382]: borrow of moved value: `field_value`
    // - means took ownership and dropped those values
    // yum yum - old memory was eaten
    // Note we could insert references - but the references must be
    // - valid for life of hashmap

}
