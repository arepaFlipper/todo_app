use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let counter = RwSignal::new(0);

    counter.set(1);

    counter.get();

    view! {
        <div>
            <p>"Counter: "{counter}</p>
        </div>
    }
}
