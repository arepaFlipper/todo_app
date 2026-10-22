use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view ! {
        <main>
            <h1>"Hello 👋🏻, world! 🌍"</h1>
        </main>
    }
}
