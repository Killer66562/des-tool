use crate::components::HexInput;
use leptos::prelude::*;

#[component]
pub fn ResultForm(
    #[prop(into)] ip_output: Signal<String>,
    #[prop(into)] pre_output: Signal<String>,
    #[prop(into)] result: Signal<String>,
    #[prop(into)] decrypt_mode: Signal<bool>,
) -> impl IntoView {
    view! {
        <form>
            <fieldset class="fieldset">
                <legend class="fieldset-legend">"輸出"</legend>

                <label for="iped" class="label">
                    "After IP"
                </label>

                <HexInput
                    attr:id="iped"
                    attr:name="iped"
                    attr:class="input w-full"
                    attr:readonly=true
                    value=ip_output
                />

                <label for="pre-output" class="label">
                    "Before Last Swap"
                </label>

                <HexInput
                    attr:id="pre-output"
                    attr:name="pre-output"
                    attr:class="input w-full"
                    attr:readonly=true
                    value=pre_output
                />

                <label for="output" class="label">
                    {move || if decrypt_mode.get() { "明文" } else { "密文" }}
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
