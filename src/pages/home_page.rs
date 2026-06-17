use crate::components::{EncryptionForm, ResultForm, RoundOutputsTable, SubkeysTable};
use crate::des::{DesState, des_encrypt_decrypt};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (text, set_text) = signal(String::new());
    let (key, set_key) = signal(String::new());
    let (result, set_result) = signal(String::new());
    let (decrypt_mode, set_decrypt_mode) = signal(false);

    let (des_state, set_des_state) = signal::<Option<DesState>>(None);

    let show_details = Memo::new(move |_| !result.get().is_empty() && !des_state.get().is_none());

    let on_submit = move |t: String, k: String, d: bool| {
        let input = match u64::from_str_radix(&t, 16) {
            Ok(v) => v,
            Err(_) => return,
        };

        let key = match u64::from_str_radix(&k, 16) {
            Ok(v) => v,
            Err(_) => return,
        };

        let (result, des_state) = des_encrypt_decrypt(input, key, d);

        set_result.set(format!("{:016X}", result));
        set_des_state.set(Some(des_state));
    };

    view! {
        <h2 class="text-xl font-bold text-center mb-4">
            {move || if decrypt_mode.get() { "Decrypt" } else { "Encrypt" }}
        </h2>
        <EncryptionForm
            text=text
            set_text=set_text
            key=key
            set_key=set_key
            decrypt_mode=decrypt_mode
            set_decrypt_mode=set_decrypt_mode
            on_submit=on_submit
            attr:class="mb-4"
        />
        <Show
            when=move|| show_details.get()
        >
            <ResultForm
                ip_output=move || format!("{:016X}", des_state.get().unwrap_or_default().ip_output)
                pre_output=move || format!("{:016X}", des_state.get().unwrap_or_default().pre_output)
                result=move || format!("{:016X}", des_state.get().unwrap_or_default().output)
                decrypt_mode=decrypt_mode
            />
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="col-span-1 overflow-x-auto">
                    <RoundOutputsTable
                        value=move || des_state.get().map(|state| state.rounds).unwrap_or_default().into()
                        attr:class="table table-auto table-zebra text-center"
                    />
                </div>
                <div class="col-span-1 overflow-x-auto">
                    <SubkeysTable
                        value=move || des_state.get().map(|state| state.subkeys).unwrap_or_default().into()
                        attr:class="table table-auto table-zebra text-center"
                    />
                </div>
            </div>
        </Show>
    }
}
