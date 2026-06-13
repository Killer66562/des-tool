use leptos::prelude::*;

mod layouts;
use layouts::default::Layout;

mod components;
use components::encryption_form::EncryptionForm;

#[component]
fn App() -> impl IntoView {
    view! {
        <Layout>
            <EncryptionForm />
        </Layout>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
