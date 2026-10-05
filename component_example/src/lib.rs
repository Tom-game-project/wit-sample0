#[allow(warnings)]
mod bindings;

use bindings::host::hello_world::*;

//use bindings::Guest;
use crate::bindings::exports::component::tom::user_funcs::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world(name: String) -> String {
        host_trait::set_name(&format!("component-user-{}", name));
        let current_name = host_trait::get_name();
        let greeting = host_trait::say_hello(&name);
        format!(
            "Hello, World!\ninput: {name}\nhost name: \"{current_name}\"\nhost say_hello: \"{greeting}\""
        )
    }
}

bindings::export!(Component with_types_in bindings);
