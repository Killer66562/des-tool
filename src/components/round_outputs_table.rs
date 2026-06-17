use crate::des::{Indexed, RoundState};
use leptos::prelude::*;

#[component]
pub fn RoundOutputsTable(#[prop(into)] value: Signal<[Indexed<RoundState>; 16]>) -> impl IntoView {
    view! {
        <table>
            <caption class="caption-top text-center font-bold p-4">"Outputs Per Round"</caption>
            <thead>
                <tr>
                    <th>"Round"</th>
                    <th>"L"</th>
                    <th>"R"</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || value.get()
                    key=move |round| (
                        round.state.l_out, round.state.r_out
                    )
                    let (Indexed { index, state })
                >
                    <tr>
                        <td>{index}</td>
                        <td>{format!("{:08X}", state.l_out)}</td>
                        <td>{format!("{:08X}", state.r_out)}</td>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}
