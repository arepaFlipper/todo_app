use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view ! {
        <main>
            <div>
                <div>List of items</div>
                <input type="text" />
                <button>Add Item</button>
            </div>
        </main>
    }
}
