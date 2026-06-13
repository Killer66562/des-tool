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
                <div class="grid grid-cols-1 md:grid-cols-3">
                    <div class="col-span-1"></div>
                    <div class="col-span-1">
                        {children()}
                    </div>
                    <div class="col-span-1"></div>
                </div>
            </main>
            <footer class="h-12">
                <p class="text-center">資工四 S1154012 黃獻毅</p>
            </footer>
        </div>
    }
}