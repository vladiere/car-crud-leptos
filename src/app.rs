use crate::{
    error_template::{AppError, ErrorTemplate},
    layouts::MainLayout,
    views::{
        AddCarView, AddCategoryView, AddColorView, CarDetailsView, CarsView, CategoryDetailsView,
        CategoryView, ColorDetailsView, ColorView,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ThemeState {
    pub theme: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // let (theme_state, set_theme, _) =
    //     use_local_storage::<ThemeState, codee::string::JsonSerdeCodec>("theme");
    // let theme_ctx = RwSignal::new("default");

    // provide_context(theme_ctx);

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/car-crud.css"/>

        // sets the document title
        <Title text="CarriTOON"/>
        <Html attr:data-theme="black" />
        <Body class="min-h-screen" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Routes>
                <Route path="" view=MainLayout >
                    <Route path="" view=HomePage />
                    <Route path="/car" view=|| view! { <Outlet /> } >
                        <Route path="" view=CarsView />
                        <Route path="/add" view=AddCarView />
                        <Route path="/:id" view=CarDetailsView />
                    </Route>
                    <Route path="/category" view=|| view! { <Outlet /> } >
                        <Route path="" view=CategoryView />
                        <Route path="/add" view=AddCategoryView />
                        <Route path="/:id" view=CategoryDetailsView />
                    </Route>
                    <Route path="/color" view=|| view! { <Outlet /> } >
                        <Route path="" view=ColorView />
                        <Route path="/add" view=AddColorView />
                        <Route path="/:id" view=ColorDetailsView />
                    </Route>
                </Route>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
