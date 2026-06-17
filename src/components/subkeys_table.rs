use crate::des::Indexed;
use leptos::prelude::*;

#[component]
pub fn SubkeysTable(#[prop(into)] value: Signal<[Indexed<u64>; 16]>) -> impl IntoView {
    view! {
        <table>
            <caption class="caption-top text-center font-bold p-4">"輪次密鑰"</caption>
            <thead>
                <tr>
                    <th>"輪次"</th>
                    <th>"密鑰"</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || value.get()
                    key=move |round| round.state
                    let (Indexed { index, state })
                >
                    <tr>
                        <td>{index}</td>
                        <td>{format!("{:08X}", state)}</td>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}
