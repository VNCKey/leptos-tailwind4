use console_error_panic_hook;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let (task_title, set_task_title) = signal(String::new());
    let (tasks, set_tasks) = signal(Vec::<String>::new());
    let (count, set_count) = signal(0);
    view! {
        <main>
            <div>
                <div>Lista de Productos</div>
                <For
                    each=move || tasks.get()
                    key=|task| task.clone()
                    children=move |task| {
                        view! { <div>{task}</div> }
                    }
                />
                <input
                    type="text"
                    on:input=move |event| {
                        set_task_title.set(event_target_value(&event));
                    }
                />
                <button
                    class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition"
                    on:click=move |_| {
                        set_tasks.update(|tasks| tasks.push(task_title.get()));
                    }
                >
                    Add Item
                </button>
            </div>
            <div>
                <button on:click=move |_| { *set_count.write() += 1 }>"Click me: "{count}</button>
                <p>{move || count.get()}</p>
            </div>
            <button class="btn btn-primary">"Hola con DaisyUI"</button>
        </main>
    }
}
