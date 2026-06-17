use crate::components::hex_input::HexInput;
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
    decrypt_mode: ReadSignal<bool>,
    set_decrypt_mode: WriteSignal<bool>,
    /// 提交表單時觸發的回調函數，傳入文本、密鑰和是否為解密模式。
    #[prop(into)]
    on_submit: Callback<(String, String, bool)>,
) -> impl IntoView {
    let is_key_invalid = Memo::new(move |_| !RE.is_match(key.get().as_str()));
    let is_text_invalid = Memo::new(move |_| !RE.is_match(text.get().as_str()));
    let is_button_disabled = Memo::new(move |_| is_key_invalid.get() || is_text_invalid.get());

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                on_submit.run((text.get(), key.get(), decrypt_mode.get()))
            }
        >
            <fieldset class="fieldset">
                <legend class="fieldset-legend">"輸入"</legend>

                <label for="key" class="label">"密鑰"</label>
                <HexInput
                    attr:id="key"
                    attr:name="key"
                    attr:class="input w-full"
                    value=key
                    set_value=set_key
                />
                <Show when=move || is_key_invalid.get()>
                    <p class="text-red-500">"請輸入一個合法的密鑰"</p>
                </Show>

                <label for="input" class="label">{move || if decrypt_mode.get() { "密文" } else { "明文" }}</label>
                <HexInput
                    attr:id="input"
                    attr:name="input"
                    attr:class="input w-full"
                    value=text
                    set_value=set_text
                />
                <Show when=move || is_text_invalid.get()>
                    <p class="text-red-500">"請輸入一個合法的"{move || if decrypt_mode.get() { "密文" } else { "明文" }}</p>
                </Show>
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">"模式"</legend>
                <div class="flex flex-0 gap-2">
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
                        <label for="decrypt_mode">"解密"</label>
                    </div>
                </div>
            </fieldset>

            <button type="submit" class="btn btn-primary mt-4 mb-4 w-full" disabled=move || is_button_disabled.get()>
                {move || if decrypt_mode.get() { "解密" } else { "加密" }}
            </button>
        </form>
    }
}
