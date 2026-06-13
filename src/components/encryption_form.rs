use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use std::sync::LazyLock;
use regex::Regex;
use crate::components::hex_input::HexInput;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^[0-9A-Fa-f]{16}$").unwrap()
});

#[component]
pub fn EncryptionForm() -> impl IntoView {
    let (plaintext, set_plaintext) = signal(String::new());
    let (key, set_key) = signal(String::new());
    let (ciphertext, set_ciphertext) = signal(String::new());

    let is_key_invalid = Memo::new(move |_| {
        !RE.is_match(key.get().as_str())
    });
    let is_plaintext_invalid = Memo::new(move |_| {
        !RE.is_match(plaintext.get().as_str())
    });
    let is_button_disabled = Memo::new(move |_| {
        is_key_invalid.get() || is_plaintext_invalid.get()
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default()
    };

    view! {
        <form on:submit=on_submit>
            <h2 class="text-xl font-bold text-center">Encrypt</h2>
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

                <label for="plaintext" class="label">明文</label>
                <HexInput 
                    attr:id="plaintext" 
                    attr:name="plaintext" 
                    attr:class="input w-full" 
                    value=plaintext 
                    set_value=set_plaintext 
                />
                <Show when=move || is_plaintext_invalid.get()>
                    <p class="text-red-500">請輸入一個合法的明文</p>
                </Show>
            </fieldset>

            <button type="submit" class="btn btn-primary mt-4 mb-4 w-full" disabled=move || is_button_disabled.get() >加密</button>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">輸出</legend>

                <label for="ciphertext" class="label">密文</label>
                <HexInput 
                    attr:id="ciphertext" 
                    attr:name="ciphertext" 
                    attr:class="input w-full" 
                    attr:readonly=true
                    value=ciphertext 
                    set_value=set_ciphertext 
                />
            </fieldset>
        </form>
    }
}