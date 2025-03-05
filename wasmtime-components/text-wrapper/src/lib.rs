#[allow(warnings)]
mod bindings;

use crate::bindings::exports::jthijsma::text_package::text_interface::Guest;

use bindings::jthijsma::random_package::random_interface::random;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> u32 {
        random()
    }
}

bindings::export!(Component with_types_in bindings);
