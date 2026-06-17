use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <nav class="p-4 bg-primary">
                <h1 class="text-2xl text-center font-bold">"Online DES Tool"</h1>
            </nav>
            <main class="flex-1 p-4">
                {children()}
            </main>
            <footer>
                <address class="text-center p-4">
                    "資工四 S1154012"
                    <br></br>
                    "Written in Rust with Leptos"
                </address>
            </footer>
        </div>
    }
}
