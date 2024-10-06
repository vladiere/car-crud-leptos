use leptos::*;
use leptos_router::{Outlet, A};

use crate::icons::{ChevronDownIcon, HomeIcon, PlusIcon};

#[component]
pub fn MainLayout() -> impl IntoView {
    let theme_names = vec![
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ];
    // let search_results = create_resource(search, fetch_results);

    view! {
        <div class="min-h-screen flex flex-col">
            <div class="bg-base-100 flex flex-wrap lg:grid lg:grid-cols-3 items-center gap-2 px-14 py-6">
                <div class="flex flex-1 items-center">
                    <A href="/" class="btn btn-ghost text-xl">CarriTOON</A>
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn m-1">
                            "Theme"
                            <ChevronDownIcon />
                        </div>
                        <ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl h-52 overflow-y-scroll">
                            <li>
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label="default"
                                    value="default"
                                />
                            </li>
                            {theme_names.into_iter().map(|theme| view! {
                                <li>
                                    <input
                                        type="radio"
                                        name="theme-dropdown"
                                        class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                        aria-label=theme
                                        value=theme />
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                </div>
                <div class="flex items-center text-md">
                    <a href="/" class="btn btn-ghost flex items-center"> <HomeIcon class="h-4 w-4"/>"Home"</a>
                    <a href="/car/add" class="btn btn-ghost flex items-center"> <PlusIcon class="h-4 w-4" />"New Car"</a>
                    <a href="/category/add" class="btn btn-ghost flex items-center"> <PlusIcon class="h-4 w-4" />"New Category"</a>
                    <a href="/color/add" class="btn btn-ghost flex items-center"> <PlusIcon class="h-4 w-4" />"New Color"</a>
                </div>
            </div>

            <Outlet />
        </div>
    }
}
