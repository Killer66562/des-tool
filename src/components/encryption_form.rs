use crate::components::hex_input::HexInput;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use regex::Regex;
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[0-9A-Fa-f]{16}$").unwrap());

#[component]
pub fn EncryptionForm(
    text: ReadSignal<String>,
    set_text: WriteSignal<String>,
    key: ReadSignal<String>,
    set_key: WriteSignal<String>,
    result: ReadSignal<String>,
    set_result: WriteSignal<String>,
    decrypt_mode: ReadSignal<bool>,
    set_decrypt_mode: WriteSignal<bool>,
    #[prop(into)] on_submit: Callback<SubmitEvent>,
    #[prop(attrs, optional)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let is_key_invalid = Memo::new(move |_| !RE.is_match(key.get().as_str()));
    let is_text_invalid = Memo::new(move |_| !RE.is_match(text.get().as_str()));
    let is_button_disabled = Memo::new(move |_| is_key_invalid.get() || is_text_invalid.get());

    view! {
        <form
            {..attrs}
            on:submit=move |ev| on_submit.run(ev)
        >
            <fieldset class="fieldset">
                <legend class="fieldset-legend">輸入</legend>

                <label for="key" class="label">密鑰</label>
                <HexInput
                    attr:id="key"
                    attr:name="key"
                    attr:class="input w-full"
                    value=key
                    set_value=set_key
                />
                <Show when=move || is_key_invalid.get()>
                    <p class="text-red-500">請輸入一個合法的密鑰</p>
                </Show>

                <label for="plaintext" class="label">{move || if decrypt_mode.get() { "密文" } else { "明文" }}</label>
                <HexInput
                    attr:id="plaintext"
                    attr:name="plaintext"
                    attr:class="input w-full"
                    value=text
                    set_value=set_text
                />
                <Show when=move || is_text_invalid.get()>
                    <p class="text-red-500">請輸入一個合法的{move || if decrypt_mode.get() { "密文" } else { "明文" }}</p>
                </Show>
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">模式</legend>
                <div class="flex flex-0">
                    <div>
                        <input
                            type="radio"
                            id="encrypt_mode"
                            name="encrypt_mode"
                            class="radio"
                            value=false
                            prop:checked=move || !decrypt_mode.get()
                            on:change=move |_| set_decrypt_mode.set(false)
                        />
                        <label for="encrypt_mode">加密</label>
                    </div>
                    <div>
                        <input
                            type="radio"
                            id="decrypt_mode"
                            name="decrypt_mode"
                            class="radio"
                            value=true
                            prop:checked=move || decrypt_mode.get()
                            on:change=move |_| set_decrypt_mode.set(true)
                        />
                        <label for="decrypt_mode">解密</label>
                    </div>
                </div>
            </fieldset>

            <button type="submit" class="btn btn-primary mt-4 mb-4 w-full" disabled=move || is_button_disabled.get()>
                {move || if decrypt_mode.get() { "解密" } else { "加密" }}
            </button>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">輸出</legend>

                <label for="ciphertext" class="label">
                    {move || if decrypt_mode.get() { "明文" } else { "密文" }}
                </label>
                <HexInput
                    attr:id="ciphertext"
                    attr:name="ciphertext"
                    attr:class="input w-full"
                    attr:readonly=true
                    value=result
                    set_value=set_result
                />
            </fieldset>
        </form>
    }
}
