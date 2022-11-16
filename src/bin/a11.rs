// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
struct GItem {
// * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32
}
// * Create a function to display the quantity, with the struct as a parameter
fn quantity (item: &GItem) {
  println!("quantity: {:?}", item.quantity)
}
// * Create a function to display the id number, with the struct as a parameter
fn id (item: &GItem) {
  println!("id: {:?}", item.id)
}

fn main() {
  let items = GItem {
    quantity: 3,
    id: 873979294,
  };
  quantity(&items);
  id(&items);
}
