use leptos::prelude::*;
use leptos::attr::any_attribute::AnyAttribute;

#[component]
pub fn HexInput(
    value: ReadSignal<String>, 
    set_value: WriteSignal<String>, 
    #[prop(attrs, optional)] attrs: Vec<AnyAttribute>
) -> impl IntoView {
    let on_input = move |ev| {
        let filtered = 
            event_target_value(&ev)
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .take(16)
            .collect();

        set_value.set(filtered)
    };

    view! {
        <input 
            {..attrs}
            type="text" 
            pattern="[0-9A-Fa-f]+" 
            max=16 prop:value=move || value.get() 
            on:input=on_input
        />
    }
}
