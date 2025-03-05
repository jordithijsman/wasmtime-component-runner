#[allow(warnings)]
mod bindings;

use bindings::jthijsma::text_package::text_interface::hello_world;

fn main() {
    let number = hello_world();
    println!("{}", number);
}
