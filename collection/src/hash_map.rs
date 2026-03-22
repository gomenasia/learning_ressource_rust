fn main() {
    // ============ Hash Map ============
    // stock ces donnée dans la heap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // indexé 
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // parcourire toute les clef valeur
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    //For owned values like String, the values will be moved and the hash map will be the owner of those

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // --------
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 10 a été deplacer

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // --------
    scores.entry(String::from("Yellow")).or_insert(50); // insert la valeur seulement si scores.get("Yellow") == null
    scores.entry(String::from("Blue")).or_insert(50);

    // --------
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    //*output {"world": 2, "hello": 1, "wonderful": 1} */
}