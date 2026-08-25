// compound data types
// arrays, tuples, slices and strings (slice string)

// Arrays
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
    // let mix  = [1,2, "apple", true];
    // println!("Mix Array: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "orange", "banana"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array 1st element: {}", fruits[0]);
    println!("Fruits array 2nd element: {}", fruits[1]);
    println!("Fruits array 3rd element: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    print!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices:&[&String] = &[&"IT" .to_string(), &"Harry Potter" .to_string(), &"ZEN" .to_string()];
    println!("Number Slice: {:?}", book_slices);

    // Strings vs String Slices (&str)
    let stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}

fn print(){
    println!("SLICE: {}", slice);
}
