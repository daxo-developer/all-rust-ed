fn main(){
  let x: i32 = -42;
  let y: u64 = 100;
  println!("Signed Integer: {}", x);
  println!("Unsigned Integer: {}", y);
// diff bet i32 (32 bits) and i64 (64 bits)
// range :
// i32 - 54183349
// i64 - 9266371353356782
  let e: i32 = 54183349;
  let i: i64 = 9266371353356782;
  println!("Maximum value of i32: {}", e);
  println!("Maximum value of i64: {}", i);

// ----------------------------------------
// Floats
// f32, f64
  let pi: f64 = 3.14;
  println!("Value of pi: {}", pi);
// boolean values: true, false
let is_snowing: bool = true;
println!("Is it snowing? {}", is_snowing);
// Charachter type - char
let letter: char = 'a';
println!("First letter of the alphabet: {}", letter);
  
}
