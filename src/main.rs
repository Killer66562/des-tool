use leptos::prelude::*;

use des_tool::layouts::default::Layout;
use des_tool::pages::home_page::HomePage;

#[component]
fn App() -> impl IntoView {
    view! {
        <Layout>
            <HomePage />
        </Layout>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
