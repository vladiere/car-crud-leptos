use leptos::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TComponentProps {
    pub key: i32,
    pub value: String,
}

#[component]
pub fn TableComponent(
    #[prop(optional, default = "table")] class: &'static str,
    #[prop(into, optional)] style: Option<AttributeValue>,
    headers: RwSignal<Vec<TComponentProps>>,
    children: Children,
) -> impl IntoView {
    view! {
        <table class=class style=style>
            <thead>
                <tr>
                    <For
                        each=headers
                        key=move |state| state.key
                        let:child
                    >
                        <th>{child.value}</th>
                    </For>
                </tr>
            </thead>
            <tbody>
                {children()}
            </tbody>
        </table>
    }
}
