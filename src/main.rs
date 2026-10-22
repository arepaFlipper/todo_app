use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (task_title, task_title_writer) = signal(String::new());
    let (tasks, tasks_writer) = signal(Vec::<Item>::new());

    let (counter, counter_writer) = signal(0);

    view! {
        <main >
            <div>
                <div>List of items</div>
                <ol>
                    <For
                        each = move || tasks.get()
                        key = |task| task.id
                        children = move |task| {
                            view! {
                                <li style="list-style-type: none; display: flex; flex-direction: row">
                                    <h6 style="margin-right: 3px">{task.id}.</h6>
                                    <h3>{task.title}</h3>
                                </li>
                            }
                        }
                    />
                </ol>
                <input type="text" on:input=move|event| {
                    task_title_writer.set(event_target_value(&event));
                } />
                <button on:click=move |_| {
                        counter_writer.set(counter.get() + 1);

                        tasks_writer.write().push(Item {
                            id: counter.get(),
                            title: task_title.get()
                        });
                }>Add Item</button>

            </div>
        </main>
    }
}

#[derive(Clone)]
struct Item {
    id: u32,
    title: String,
}
