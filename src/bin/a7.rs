// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
  Red,
  Green,
  Yellow,
  Blue,
}

fn print_col (col: Colors){
match col {
  Colors::Red => println!("Red"),
  Colors::Green => println!("Green"),
  Colors::Yellow => println!("Yellow"),
  Colors::Blue => println!("Blue"),
}
}
fn main() {
  print_col(Colors::Blue);
}
