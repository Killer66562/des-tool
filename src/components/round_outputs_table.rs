use crate::des::{Indexed, RoundState};
use leptos::prelude::*;

#[component]
pub fn RoundOutputsTable(#[prop(into)] value: Signal<[Indexed<RoundState>; 16]>) -> impl IntoView {
    view! {
        <table>
            <caption class="caption-top text-center font-bold p-4">"輪次輸出"</caption>
            <thead>
                <tr>
                    <th>"輪次"</th>
                    <th>"輸出L"</th>
                    <th>"輸出R"</th>
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
