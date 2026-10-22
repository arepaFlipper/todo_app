use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (task, task_writer) = signal(Item::default());
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
                                    <div style="flex-grow: 1">
                                        <h3>{task.title}</h3>
                                        <p>{task.description}</p>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ol>
                <div style="display: flex; flex-flow: column">
                    <input type="text" on:input=move|event| {
                        task_writer.write().title = event_target_value(&event);
                    } />
                    <textarea on:input=move|event| {
                        task_writer.write().description = event_target_value(&event);
                    } />
                </div>
                <button on:click=move |_| {
                        counter_writer.set(counter.get() + 1);

                        tasks_writer.write().push(Item {
                            id: counter.get(),
                            title: task.get().title,
                            description: task.get().description
                        });
                }>Add Item</button>

            </div>
        </main>
    }
}

#[derive(Clone, Default)]
struct Item {
    id: u32,
    title: String,
    description: String,
}
