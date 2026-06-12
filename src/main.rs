use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <p>"Hello, world!"</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
