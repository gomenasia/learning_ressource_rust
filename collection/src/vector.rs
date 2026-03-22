fn main() {
    
    // ================================= Vecteur =================================
    //Vec<T>
    let mut vect: Vec<i32> = Vec::new();
    
    let vect_macro = vec![1, 2, 3]; // la macro vec! detect automatiquement le type du vecteur
    
    vect.push(4); //ajout de valeurs
    vect.push(7);
    
    // acess au donnée
    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2];                // cette methode panick quand out of band
    println!("The third element is {third}");
    
    let third: Option<&i32> = v.get(2); // cette methode renvoie just None quand out of band
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    // parcourir les element
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    // mutable
    let mut mut_v = vec![100, 32, 57];
    for i in &mut mut_v {
        *i += 50; // `*` dereference i
    }
    
    // stocker des valuer de type different
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // dropping a vector mean dropping its element
}