use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (task_title, task_title_writer) = signal(String::new());
    let (tasks, tasks_writer) = signal(Vec::<String>::new());
    view! {
        <main >
            <div>
                <div>List of items</div>
                <div>
                    <For
                        each= move || tasks.get()
                        key = |task| task.clone()
                        children = move |task| {
                            view! {
                                <div>{task}</div>
                            }
                        }
                    />
                </div>
                <input type="text" on:input=move|event| {
                    task_title_writer.set(event_target_value(&event));
                } />
                <button on:click=move |_| {
                    tasks_writer.write().push(task_title.get())
                }>Add Item</button>
            </div>
        </main>
    }
}
