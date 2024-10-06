use leptos::*;
use leptos_router::{use_query_map, ActionForm, Form};

use crate::{
    components::{
        LoadingComponent, PanicsFerrisComponent, TComponentProps, TableComponent,
        WtfFerrisComponent,
    },
    icons::{MagnifierGlassIcon, TrashIcon},
    servers::{get_colors, InsertColor},
};

#[component]
pub fn ColorView() -> impl IntoView {
    view! {
        "this is color view"
    }
}

#[component]
pub fn AddColorView() -> impl IntoView {
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();

    let theaders = RwSignal::new(vec![
        TComponentProps {
            key: 1,
            value: "#".to_string(),
        },
        TComponentProps {
            key: 2,
            value: "Color ID".to_string(),
        },
        TComponentProps {
            key: 3,
            value: "Color Name".to_string(),
        },
        TComponentProps {
            key: 4,
            value: "Color Type".to_string(),
        },
        TComponentProps {
            key: 5,
            value: "Created at".to_string(),
        },
        TComponentProps {
            key: 6,
            value: "Action".to_string(),
        },
    ]);

    let add_color = create_server_action::<InsertColor>();
    let list_color = create_resource(move || add_color.version().get(), move |_| get_colors());

    view! {
        <div class=r#"flex flex-col w-full mx-auto gap-3 px-8 md:px-10 2xl:px-12 3xl:px-14"#>
            <ActionForm action={add_color} class="flex items-end gap-3">
                <div class="form-control w-full">
                    <div class="label">
                        <span class="label-text text-md">"Color name"</span>
                    </div>
                    <input type="text" name="color_name" id="color_name" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                </div>

                <div class="form-control w-full">
                    <div class="label">
                        <span class="label-text text-md">"Color type"</span>
                    </div>
                    <input type="text" name="color_type" id="color_type" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                </div>

                <button class="btn btn-primary" type="submit">"Add new color"</button>
            </ActionForm>

            <Form method="GET" action="" class="input input-bordered flex flex-1 lg:flex-none items-center gap-2 w-full py-2.5">
                <MagnifierGlassIcon />
                <input
                    type="search"
                    name="q"
                    id="q"
                    value=search
                    placeholder="Search..."
                    class="grow"
                    required=true
                    autocomplete="of"
                />
                <input type="submit" hidden=true />
            </Form>

            <Transition fallback=move || view! { <LoadingComponent /> }>
                <TableComponent headers={theaders}>
                    {move || {
                        let existing_colors = {
                            move || {
                                list_color.get()
                                    .map(move |colors| match colors {
                                        Err(e) => {
                                            view! { <PanicsFerrisComponent message={format!("Ohh no!... Something went wrong error: {:?}", e.to_string())} /> }
                                        },
                                        Ok(colors) => {
                                            if colors.is_empty() {
                                                view! { <WtfFerrisComponent /> }
                                            } else {
                                                colors.into_iter().map(move |color| {
                                                    view! {
                                                        <tr class="hover">
                                                            <th>{color.id}</th>
                                                            <td>{color.color_id}</td>
                                                            <td>{color.color_name}</td>
                                                            <td>{color.color_type}</td>
                                                            <td>{color.ctime}</td>
                                                            <td class="flex items-center gap-2">
                                                                <button>
                                                                    <TrashIcon />
                                                                </button>
                                                            </td>
                                                        </tr>
                                                    }
                                                })
                                                .collect_view()
                                            }
                                        }
                                    })
                                        .unwrap_or_default()
                            }
                        };
                        view! {
                            {existing_colors}
                        }
                    }}
                </TableComponent>
            </Transition>
        </div>
    }
}

#[component]
pub fn ColorDetailsView() -> impl IntoView {
    view! {
        "this is color details view"
    }
}
