// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
 let what = true;
// * Use an if..else block to determine which message to display
fn message() -> str {
  if what = true {
    return "hello";
  }
  else {
    return "goodbye";
  }
}
// * Use the println macro to display messages to the terminal

fn main() {
  println!("{:?}", message());
}
