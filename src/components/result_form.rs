use crate::components::HexInput;
use leptos::prelude::*;

#[component]
pub fn ResultForm(
    #[prop(into)] result: Signal<String>,
    #[prop(into)] decrypt_mode: Signal<bool>,
) -> impl IntoView {
    view! {
        <form>
            <fieldset class="fieldset">
                <legend class="fieldset-legend">"Output"</legend>

                <label for="output" class="label">
                    {move || if decrypt_mode.get() { "Plaintext" } else { "Ciphertext" }}
                </label>

                <HexInput
                    attr:id="output"
                    attr:name="output"
                    attr:class="input input-success w-full"
                    attr:readonly=true
                    value=result
                />
            </fieldset>
        </form>
    }
}
