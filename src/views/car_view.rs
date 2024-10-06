use leptos::*;
use leptos_router::Form;

#[component]
pub fn CarsView() -> impl IntoView {
    view! {
        "this is the car view"
    }
}

#[component]
pub fn AddCarView() -> impl IntoView {
    view! {
        <div class=r#"flex items-center w-full md:w-1/2 xl:w-2/5 2xl:w-2/5 3xl:w-1/3 mx-auto p-8 md:p-10 2xl:p-12 3xl:p-14"#>
            <header class="text-xl font-semibold pb-8 text-center">"Add new car vehicle."</header>
            <Form method="POST" action="" class="flex flex-col">
                <div class="pb-2 grid grid-cols-2 gap-4">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"Car brand"</span>
                        </div>
                        <input type="text" name="brand" id="brand" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"Car model"</span>
                        </div>
                        <input type="text" name="model" id="model" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="pb-2 grid grid-cols-2 gap-4">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"Car year."</span>
                        </div>
                        <input type="number" min="0" max="9999" name="year" id="year" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>

                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"What fuel type."</span>
                        </div>
                        <input type="text" name="fuel_type" id="fuel_type" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="pb-2 grid grid-cols-2 gap-4">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"What engine size?."</span>
                        </div>
                        <input type="number" min="0" max="99" name="engine_size" id="engine_size" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>

                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"What transmission type?."</span>
                        </div>
                        <input type="text" name="transmission_type" id="transmission_type" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="pb-2">
                    <label class="form-control w-full">
                        <div class="label">
                            <span class="label-text text-md">"VIN (Vehicle Identification Number)."</span>
                        </div>
                        <input type="text" name="vin" id="vin" placeholder="Type here" class="input input-secondary w-full" autocomplete="off" required=true />
                    </label>
                </div>

                <div class="flex items-center mt-4">
                    <button class="btn btn-primary" type="submit">"Add car"</button>
                </div>
            </Form>
        </div>
    }
}

#[component]
pub fn CarDetailsView() -> impl IntoView {
    view! {
        "this is the CarDetailsView"
    }
}
