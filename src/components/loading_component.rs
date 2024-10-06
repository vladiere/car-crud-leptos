use leptos::*;

#[component]
pub fn LoadingComponent(
    #[prop(
        into,
        optional,
        default = "min-h-full flex flex-col items-center justify-center"
    )]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class=class>
            <span class="loading loading-ring loading-lg"></span>
        </div>
    }
}
