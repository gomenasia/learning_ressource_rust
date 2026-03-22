fn main() {
    // ================================= String =================================
    //String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities
    let mut s = String::new();
    
    let data = "initial contents";
    
    let s = data.to_string();
    
    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    
    let s = String::from("initial contents"); // même chose qu'au dessus
    
    // ajouter une slice ($str)
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    
    // ajouter un seul caracter
    let mut s: String = String::from("lo");
    s.push('l');
    
    // ajouter deux String
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    // concatenation de plusieur String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = s1 + "-" + &s2 + "-" + &s3; // les deux ligne font la même chose mais celle du bas est plus lisible
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{s1}-{s2}-{s3}");
    
    //on ne peut pas indexe un string
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // ici s contient uniquement Зд cf le livre 
    
    //pour iterer sur cette structure il faut preciser au compilateur ce qu'on attent des donnée
    for c in "Зд".chars() { // enn char
        println!("{c}");
    }
    
    for b in "Зд".bytes() { //en byte (1 byte != 1 char)
        println!("{b}");
    }
} 