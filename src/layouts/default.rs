use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <nav class="p-4 bg-accent">
                <h1 class="text-2xl text-center font-bold">Online DES Tool</h1>
                // navbar
            </nav>
            <main class="flex-1 p-4">
                {children()}
            </main>
            <footer>
                <address class="text-center">
                    資工四 S1154012 黃獻毅
                    <br></br>
                    Written in Rust with Leptos
                </address>
            </footer>
        </div>
    }
}
