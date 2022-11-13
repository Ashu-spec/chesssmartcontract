// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Fla {
  Tangy,
  Sweet,
  Blend,
}
struct Drink {
  fla: Fla,
  fl_oz: f64,
}
fn print_fl_oz (drink: Drink){
  match drink.fla {
    Fla::Tangy => println!("fla: tangy"),
    Fla::Sweet => println!("fla: sweet"),
    Fla::Blend => println!("fla: blend"),
  }
  println!("oz: {:?}", drink.fl_oz);
}
fn main() {
  let sweet = Drink {
    fla: Fla::Sweet,
    fl_oz: 6.0,
  };
  print_fl_oz(sweet);
  let tangy = Drink {
    fla: Fla::Tangy,
    fl_oz: 10.0,
  };
  print_fl_oz(tangy);
}
