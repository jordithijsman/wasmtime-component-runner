#[allow(warnings)]
mod bindings;

use rand::Rng;

use crate::bindings::exports::jthijsma::random_package::random_interface::Guest;

struct Component;

impl Guest for Component {
    fn random() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=100)
    }
}

bindings::export!(Component with_types_in bindings);
