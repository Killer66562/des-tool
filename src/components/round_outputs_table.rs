use crate::des::states::IndexedRoundState;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn RoundOutputsTable(
    value: Signal<[IndexedRoundState; 16]>,
    #[prop(attrs, optional)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    view! {
        <table
            {..attrs}
        >
            <thead>
                <tr>
                    <th>輪次</th>
                    <th>輸入L</th>
                    <th>輸入R</th>
                    <th>Subkey</th>
                    <th>輸出L</th>
                    <th>輸出R</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || value.get()
                    key=move |round| (
                        round.state.l_out, round.state.r_out
                    )
                    let (IndexedRoundState { index, state })
                >
                    <tr>
                        <td>{index}</td>
                        <td>{format!("{:08X}", state.l_in)}</td>
                        <td>{format!("{:08X}", state.r_in)}</td>
                        <td>{format!("{:12X}", state.subkey)}</td>
                        <td>{format!("{:08X}", state.l_out)}</td>
                        <td>{format!("{:08X}", state.r_out)}</td>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}
