use leptos::prelude::*;

#[component]
pub fn HexInput(
    #[prop(into)] value: Signal<String>,
    #[prop(optional)] set_value: Option<WriteSignal<String>>,
) -> impl IntoView {
    let on_input = move |ev| {
        if let Some(set_value) = &set_value {
            let filtered = event_target_value(&ev)
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(16)
                .collect();

            set_value.set(filtered)
        }
    };

    view! {
        <input
            type="text"
            pattern="[0-9A-Fa-f]+"
            max=16 prop:value=move || value.get()
            on:input=on_input
        />
    }
}
