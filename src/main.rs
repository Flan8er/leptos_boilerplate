use leptos::prelude::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <main class="w-screen h-screen">"Hello World"</main> }
    });
}
