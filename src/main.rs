use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "leptos-todo.tasks";

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let initial_tasks = load_tasks().unwrap_or_else(initial_tasks);
    // Use the highest id, not the length, so ids stay unique after deletions
    let counter = RwSignal::new(initial_tasks.iter().map(|task| task.id).max().unwrap_or(0));

    let title = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let tasks = RwSignal::new(initial_tasks);

    // Re-runs whenever the list or any task's status changes
    Effect::new(move |_| {
        let stored: Vec<StoredTask> = tasks.with(|tasks| tasks.iter().map(StoredTask::from).collect());
        save_tasks(&stored);
    });

    let done_count = move || {
        tasks.with(|tasks| tasks.iter().filter(|task| task.status.get() == Status::Done).count())
    };

    let add_task = move || {
        if title.with(|title| title.trim().is_empty()) {
            return;
        }

        counter.update(|counter| *counter += 1);
        tasks.write().push(Item::new(counter.get(), &title.get(), &description.get(), Status::Todo));

        title.set(String::new());
        description.set(String::new());
    };

    view! {
        <main class="mx-auto flex max-w-xl flex-col gap-6 px-4 py-12 sm:py-20">
            <header class="flex items-end justify-between">
                <div>
                    <p class="text-sm font-medium text-indigo-600 dark:text-indigo-400">"Leptos + Tailwind"</p>
                    <h1 class="text-3xl font-bold tracking-tight">"My tasks"</h1>
                </div>
                <span class="rounded-full bg-indigo-100 px-3 py-1 text-sm font-semibold text-indigo-700 dark:bg-indigo-500/15 dark:text-indigo-300">
                    {done_count} " / " {move || tasks.with(|tasks| tasks.len())} " done"
                </span>
            </header>

            <form
                class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900"
                on:submit=move |event| {
                    event.prevent_default();
                    add_task();
                }
            >
                <div class="flex flex-col gap-3">
                    <input
                        type="text"
                        placeholder="What needs to be done?"
                        class="w-full rounded-lg border border-slate-300 bg-transparent px-4 py-2.5 text-base font-semibold placeholder:font-normal placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                        bind:value=title
                    />
                    <textarea
                        rows="3"
                        placeholder="Add a description (optional)"
                        class="w-full resize-none rounded-lg border border-slate-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                        bind:value=description
                    />
                </div>
                <button
                    type="submit"
                    class="mt-4 w-full rounded-lg bg-indigo-600 px-4 py-2.5 font-semibold text-white shadow-sm transition hover:bg-indigo-500 focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:outline-none active:scale-[0.99] disabled:cursor-not-allowed disabled:opacity-50 dark:focus-visible:ring-offset-slate-900"
                    disabled=move || title.with(|title| title.trim().is_empty())
                >
                    "Add task"
                </button>
            </form>

            <Show
                when=move || tasks.with(|tasks| !tasks.is_empty())
                fallback=|| view! {
                    <p class="rounded-2xl border border-dashed border-slate-300 py-10 text-center text-sm text-slate-500 dark:border-slate-700 dark:text-slate-400">
                        "No tasks yet. Add your first one above."
                    </p>
                }
            >
                <ol class="flex flex-col gap-3">
                    <For
                        each=move || tasks.get()
                        key=|task| task.id
                        children=move |task| {
                            let id = task.id;
                            let status = task.status;
                            let is_done = move || status.get() == Status::Done;

                            view! {
                                <li class="group flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm transition hover:border-indigo-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-indigo-500/50">
                                    <input
                                        type="checkbox"
                                        aria-label="Mark as done"
                                        class="mt-1 size-5 shrink-0 cursor-pointer accent-indigo-600"
                                        prop:checked=is_done
                                        on:change=move |event| {
                                            status.set(if event_target_checked(&event) { Status::Done } else { Status::Todo });
                                        }
                                    />
                                    <div class="min-w-0 flex-1 transition" class:opacity-60=is_done>
                                        <h3 class="truncate font-semibold" class:line-through=is_done>{task.title}</h3>
                                        <p class="mt-1 text-sm whitespace-pre-line text-slate-600 dark:text-slate-400">{task.description}</p>
                                        <select
                                            aria-label="Status"
                                            class=move || format!(
                                                "mt-3 cursor-pointer rounded-full px-3 py-1 text-xs font-semibold focus:ring-2 focus:ring-indigo-500/30 focus:outline-none {}",
                                                status.get().badge_class(),
                                            )
                                            on:change=move |event| status.set(Status::from_value(&event_target_value(&event)))
                                        >
                                            {Status::ALL.map(|option| view! {
                                                <option value=option.value() prop:selected=move || status.get() == option>
                                                    {option.label()}
                                                </option>
                                            })}
                                        </select>
                                    </div>
                                    <button
                                        type="button"
                                        aria-label="Delete task"
                                        title="Delete task"
                                        class="shrink-0 rounded-lg p-1.5 text-slate-400 transition hover:bg-red-50 hover:text-red-600 focus-visible:ring-2 focus-visible:ring-red-500 focus-visible:outline-none sm:opacity-0 sm:group-hover:opacity-100 sm:focus-visible:opacity-100 dark:hover:bg-red-500/10 dark:hover:text-red-400"
                                        on:click=move |_| tasks.write().retain(|task| task.id != id)
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-5">
                                            <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6M10 11v6M14 11v6" />
                                        </svg>
                                    </button>
                                </li>
                            }
                        }
                    />
                </ol>
            </Show>
        </main>
    }
}

fn initial_tasks() -> Vec<Item> {
    vec![
        Item::new(1, "Set up the Nix flake", "Pin the Rust toolchain, Trunk and wasm-bindgen so nothing is installed globally.", Status::Done),
        Item::new(2, "Add the wasm32-unknown-unknown target", "Trunk needs it to compile the app to WebAssembly.", Status::Done),
        Item::new(3, "Style the app with Tailwind", "Let Trunk run tailwindcss on every build.", Status::InProgress),
        Item::new(4, "Learn ownership and borrowing", "Read chapter 4 of The Rust Book and redo the exercises.", Status::InProgress),
        Item::new(5, "Write unit tests with cargo test", "Cover the task list logic before adding more features.", Status::Todo),
        Item::new(6, "Fix all cargo clippy warnings", "Run cargo clippy --target wasm32-unknown-unknown and clean up.", Status::Todo),
        Item::new(7, "Persist tasks in localStorage", "Keep the list across page reloads.", Status::Todo),
    ]
}

fn local_storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

/// Returns `None` when nothing is saved yet or the saved data can't be read
fn load_tasks() -> Option<Vec<Item>> {
    let json = local_storage()?.get_item(STORAGE_KEY).ok().flatten()?;
    let stored: Vec<StoredTask> = serde_json::from_str(&json).ok()?;

    Some(stored.into_iter().map(Item::from).collect())
}

fn save_tasks(tasks: &[StoredTask]) {
    let (Some(storage), Ok(json)) = (local_storage(), serde_json::to_string(tasks)) else {
        return;
    };
    let _ = storage.set_item(STORAGE_KEY, &json);
}

/// Plain copy of an `Item` for localStorage, since signals can't be serialized
#[derive(Serialize, Deserialize)]
struct StoredTask {
    id: u32,
    title: String,
    description: String,
    status: Status,
}

impl From<&Item> for StoredTask {
    fn from(item: &Item) -> Self {
        Self {
            id: item.id,
            title: item.title.clone(),
            description: item.description.clone(),
            status: item.status.get(),
        }
    }
}

impl From<StoredTask> for Item {
    fn from(stored: StoredTask) -> Self {
        Item::new(stored.id, &stored.title, &stored.description, stored.status)
    }
}

#[derive(Clone)]
struct Item {
    id: u32,
    title: String,
    description: String,
    // Each task owns its status signal, so toggling it updates the row in place
    status: RwSignal<Status>,
}

impl Item {
    fn new(id: u32, title: &str, description: &str, status: Status) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            status: RwSignal::new(status),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

impl Status {
    const ALL: [Status; 3] = [Status::Todo, Status::InProgress, Status::Done];

    fn value(self) -> &'static str {
        match self {
            Status::Todo => "todo",
            Status::InProgress => "in-progress",
            Status::Done => "done",
        }
    }

    fn from_value(value: &str) -> Self {
        match value {
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            _ => Status::Todo,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Status::Todo => "To do",
            Status::InProgress => "In progress",
            Status::Done => "Done",
        }
    }

    fn badge_class(self) -> &'static str {
        match self {
            Status::Todo => "bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300",
            Status::InProgress => "bg-amber-100 text-amber-800 dark:bg-amber-500/15 dark:text-amber-300",
            Status::Done => "bg-emerald-100 text-emerald-800 dark:bg-emerald-500/15 dark:text-emerald-300",
        }
    }
}
