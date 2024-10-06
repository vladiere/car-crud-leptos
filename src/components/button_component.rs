use ev::MouseEvent;
use leptos::*;

#[component]
pub fn ButtonComponent(
    #[prop(optional, default = "btn btn-primary")] class: &'static str,
    #[prop(into)] label: &'static str,
    #[prop(into)] btn_action: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button class=class on:click=btn_action>{label}</button>
    }
}

#[component]
pub fn ButtonComponentClosure<ActionBtn>(
    #[prop(optional, default = "btn btn-primary")] class: &'static str,
    #[prop(into)] label: &'static str,
    btn_action: ActionBtn,
) -> impl IntoView
where
    ActionBtn: Fn(MouseEvent) + 'static,
{
    view! {
        <button class=class on:click=btn_action>{label}</button>
    }
}
