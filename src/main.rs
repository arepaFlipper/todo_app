use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (task_title, task_title_writer) = signal(String::new());
    view! {
        <main >
            <div>
                <div>List of items</div>
                <input type="text" on:input=move|event| {
                    task_title_writer.set(event_target_value(&event));
                } />
                <button>Add Item</button>
            </div>
        </main>
    }
}
