use leptos::*;

#[component]
pub fn PanicsFerrisComponent(
    #[prop(
        optional,
        default = "min-h-ful flex flex-col items-center justify-center text-center"
    )]
    class: &'static str,
    #[prop(optional, default = "h-1/4 w-1/4")] img_class: &'static str,
    #[prop(optional, default = String::from("Something went wrong."))] message: String,
) -> impl IntoView {
    view! {
        <div class=class>
            <img class=img_class src="/assets/ferris-panics.svg" alt="ferris_on_panics" />
            <h1 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl">{message}</h1>
        </div>
    }
}

#[component]
pub fn WtfFerrisComponent(
    #[prop(
        optional,
        default = "min-h-ful flex flex-col items-center justify-center text-center"
    )]
    class: &'static str,
    #[prop(optional, default = "h-1/4 w-1/4")] img_class: &'static str,
    #[prop(optional, default = "Something went wrong.")] message: &'static str,
) -> impl IntoView {
    view! {
        <div class=class>
            <img class=img_class src="/assets/wtfferris.svg" alt="ferris_on_panics" />
            <h1 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl">{message}</h1>
        </div>
    }
}

#[component]
pub fn WaveFerrisComponent(
    #[prop(
        optional,
        default = "min-h-ful flex flex-col items-center justify-center text-center"
    )]
    class: &'static str,
    #[prop(optional, default = "h-1/4 w-1/4")] img_class: &'static str,
    #[prop(optional, default = "Something went wrong.")] message: &'static str,
) -> impl IntoView {
    view! {
        <div class=class>
            <img class=img_class src="/assets/waveferris.svg" alt="ferris_on_panics" />
            <h1 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl">{message}</h1>
        </div>
    }
}

#[component]
pub fn CuddlyFerrisComponent(
    #[prop(
        optional,
        default = "min-h-ful flex flex-col items-center justify-center text-center"
    )]
    class: &'static str,
    #[prop(optional, default = "h-1/4 w-1/4")] img_class: &'static str,
    #[prop(optional, default = "Something went wrong.")] message: &'static str,
) -> impl IntoView {
    view! {
        <div class=class>
            <img class=img_class src="/assets/cuddlyferris.svg" alt="ferris_on_panics" />
            <h1 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl">{message}</h1>
        </div>
    }
}
