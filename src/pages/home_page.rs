use crate::components::encryption_form::EncryptionForm;
use crate::components::round_outputs_table::RoundOutputsTable;
use crate::des::encrypt_decrypt::des_encrypt_decrypt;
use crate::des::states::DesState;
use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (text, set_text) = signal(String::new());
    let (key, set_key) = signal(String::new());
    let (result, set_result) = signal(String::new());
    let (decrypt_mode, set_decrypt_mode) = signal(false);

    let (des_state, set_des_state) = signal::<Option<DesState>>(None);

    let show_details = Memo::new(move |_| !result.get().is_empty() && !des_state.get().is_none());
    let round_outputs = Memo::new(move |_| {
        des_state
            .get()
            .map(|state| state.rounds)
            .unwrap_or_default()
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let text_string = text.get();
        let key_string = key.get();
        let decrypt = decrypt_mode.get();

        let input = match u64::from_str_radix(&text_string, 16) {
            Ok(v) => v,
            Err(_) => return,
        };

        let key = match u64::from_str_radix(&key_string, 16) {
            Ok(v) => v,
            Err(_) => return,
        };

        let (result, des_state) = des_encrypt_decrypt(input, key, decrypt);

        log!("{:016X}", des_state.key);

        set_result.set(format!("{:016X}", result));
        set_des_state.set(Some(des_state));
    };

    view! {
        <h2 class="text-xl font-bold text-center">
            {move || if decrypt_mode.get() { "Decrypt" } else { "Encrypt" } }
        </h2>
        <EncryptionForm
            text=text
            set_text=set_text
            key=key
            set_key=set_key
            decrypt_mode=decrypt_mode
            set_decrypt_mode=set_decrypt_mode
            result=result
            set_result=set_result
            on_submit=on_submit
            attr:class="mb-4"
        />

        <h2 class="text-xl font-bold text-center">Round Outputs</h2>
        <Show
            when=move|| show_details.get()
        >
            <RoundOutputsTable
                value=round_outputs.into()
                attr:class="table table-auto"
            />
        </Show>
    }
}
