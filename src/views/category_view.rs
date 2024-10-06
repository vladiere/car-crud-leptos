use leptos::*;
use leptos_router::Form;

#[component]
pub fn CategoryView() -> impl IntoView {
    view! {
        "this is the category view"
    }
}

#[component]
pub fn AddCategoryView() -> impl IntoView {
    view! {
        <div class=r#"flex flex-col w-full md:w-1/2 xl:w-2/5 2xl:w-2/5 3xl:w-1/3 mx-auto p-8 md:p-10 2xl:p-12 3xl:p-14"#>
            <div class="text-xl font-semibold pb-8 text-center">"Add new car category."</div>
            <Form method="POST" action="" class="flex flex-col">
                <div class="pb-2">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"Category name"</span>
                        </div>
                        <input type="text" name="category_name" id="category_name" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="pb-2">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"Drive type"</span>
                        </div>
                        <input type="text" name="drive_type" id="drive_type" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="flex items-center mt-4">
                    <button class="btn btn-primary" type="submit">"Add category"</button>
                </div>
            </Form>
        </div>
    }
}

#[component]
pub fn CategoryDetailsView() -> impl IntoView {
    view! {
        "this is category details"
    }
}
