use std::time::Duration;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "leptos-todo.tasks";
/// How long the "Task deleted · Undo" toast stays up
const UNDO_TIMEOUT: Duration = Duration::from_secs(3);
/// Fraction of the card's width a swipe must pass to delete the task
const SWIPE_DELETE_RATIO: f64 = 0.35;

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

    // Re-runs whenever the list or any task's title, description or status changes
    Effect::new(move |_| {
        let stored: Vec<StoredTask> =
            tasks.with(|tasks| tasks.iter().map(StoredTask::from).collect());
        save_tasks(&stored);
    });

    let done_count = move || {
        tasks.with(|tasks| {
            tasks
                .iter()
                .filter(|task| task.status.get() == Status::Done)
                .count()
        })
    };

    let add_task = move || {
        if title.with(|title| title.trim().is_empty()) {
            return;
        }

        counter.update(|counter| *counter += 1);
        tasks.write().push(Item::new(
            counter.get(),
            &title.get(),
            &description.get(),
            Status::Todo,
        ));

        title.set(String::new());
        description.set(String::new());
    };

    // The last deleted task and where it was, kept for a few seconds so it can be undone
    let last_deleted = RwSignal::new(None::<(usize, Item)>);
    let undo_timer = StoredValue::new(None::<TimeoutHandle>);
    let clear_undo_timer = move || {
        if let Some(timer) = undo_timer.get_value() {
            timer.clear();
        }
    };

    let delete_task = move |id: u32| {
        let Some(index) = tasks.with_untracked(|tasks| tasks.iter().position(|task| task.id == id))
        else {
            return;
        };
        let item = tasks.write().remove(index);
        last_deleted.set(Some((index, item)));

        clear_undo_timer();
        let timer = set_timeout_with_handle(move || last_deleted.set(None), UNDO_TIMEOUT).ok();
        undo_timer.set_value(timer);
    };

    let undo_delete = move || {
        clear_undo_timer();
        let deleted = last_deleted.write().take();
        if let Some((index, item)) = deleted {
            tasks.update(|tasks| tasks.insert(index.min(tasks.len()), item));
        }
    };

    view! {
        <main class="mx-auto flex max-w-xl flex-col gap-6 px-4 py-12 sm:py-20">
            <header class="flex items-end justify-between">
                <div>
                    <p class="text-sm font-medium text-indigo-600 dark:text-indigo-400">"Developed by arepa_flipper 🫓"</p>
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
                            let task_title = task.title;
                            let task_description = task.description;
                            let is_done = move || status.get() == Status::Done;

                            // Edits go into drafts first, so Cancel leaves the task untouched
                            let editing = RwSignal::new(false);
                            let draft_title = RwSignal::new(String::new());
                            let draft_description = RwSignal::new(String::new());

                            let start_editing = move || {
                                draft_title.set(task_title.get());
                                draft_description.set(task_description.get());
                                editing.set(true);
                            };
                            let save_edit = move || {
                                if draft_title.with(|title| title.trim().is_empty()) {
                                    return;
                                }
                                task_title.set(draft_title.get());
                                task_description.set(draft_description.get());
                                editing.set(false);
                            };

                            // Swipe left to delete, on touch screens only
                            let row = NodeRef::<leptos::html::Li>::new();
                            let swipe_start_x = StoredValue::new(None::<f64>);
                            let swipe_offset = RwSignal::new(0.0_f64);
                            let swiping = RwSignal::new(false);

                            let reset_swipe = move || {
                                swipe_start_x.set_value(None);
                                swiping.set(false);
                                swipe_offset.set(0.0);
                            };

                            view! {
                                <li
                                    node_ref=row
                                    class="relative touch-pan-y"
                                    on:pointerdown=move |event| {
                                        if event.pointer_type() == "touch" && !editing.get_untracked() {
                                            swipe_start_x.set_value(Some(event.client_x() as f64));
                                        }
                                    }
                                    on:pointermove=move |event| {
                                        let Some(start_x) = swipe_start_x.get_value() else {
                                            return;
                                        };
                                        let offset = (event.client_x() as f64 - start_x).min(0.0);
                                        // Ignore small movements, so taps on the checkbox or status still work
                                        if !swiping.get_untracked() {
                                            if offset > -10.0 {
                                                return;
                                            }
                                            swiping.set(true);
                                            if let Some(row) = row.get_untracked() {
                                                let _ = row.set_pointer_capture(event.pointer_id());
                                            }
                                        }
                                        swipe_offset.set(offset);
                                    }
                                    on:pointerup=move |_| {
                                        if !swiping.get_untracked() {
                                            swipe_start_x.set_value(None);
                                            return;
                                        }
                                        let width = row
                                            .get_untracked()
                                            .map(|row| row.offset_width() as f64)
                                            .unwrap_or(0.0);
                                        if swipe_offset.get_untracked() < -width * SWIPE_DELETE_RATIO {
                                            // Slide the card the rest of the way out, then delete it
                                            swipe_start_x.set_value(None);
                                            swiping.set(false);
                                            swipe_offset.set(-width);
                                            set_timeout(move || delete_task(id), Duration::from_millis(200));
                                        } else {
                                            reset_swipe();
                                        }
                                    }
                                    on:pointercancel=move |_| reset_swipe()
                                >
                                <div
                                    aria-hidden="true"
                                    class="absolute inset-0 flex items-center justify-end rounded-xl bg-red-500 pr-5 text-white transition-opacity"
                                    class:opacity-0=move || swipe_offset.get() == 0.0
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-6">
                                        <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6M10 11v6M14 11v6" />
                                    </svg>
                                </div>
                                <div
                                    class="group relative flex items-start gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm transition duration-200 hover:border-indigo-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-indigo-500/50"
                                    style:transform=move || format!("translateX({}px)", swipe_offset.get())
                                    // Follow the finger exactly while dragging; animate only the snap back or slide out
                                    style:transition=move || if swiping.get() { "none" } else { "" }
                                >
                                    <input
                                        type="checkbox"
                                        aria-label="Mark as done"
                                        class="mt-1 size-5 shrink-0 cursor-pointer accent-indigo-600"
                                        prop:checked=is_done
                                        on:change=move |event| {
                                            status.set(if event_target_checked(&event) { Status::Done } else { Status::Todo });
                                        }
                                    />
                                    <Show
                                        when=move || editing.get()
                                        fallback=move || view! {
                                            <div class="min-w-0 flex-1 transition" class:opacity-60=is_done>
                                                <h3
                                                    class="truncate font-semibold"
                                                    class:line-through=is_done
                                                    title="Double-click to edit"
                                                    on:dblclick=move |_| start_editing()
                                                >
                                                    {move || task_title.get()}
                                                </h3>
                                                <p class="mt-1 text-sm whitespace-pre-line text-slate-600 dark:text-slate-400">{move || task_description.get()}</p>
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
                                        }
                                    >
                                        <TaskEditForm
                                            title=draft_title
                                            description=draft_description
                                            on_save=save_edit
                                            on_cancel=move || editing.set(false)
                                        />
                                    </Show>
                                    <div class="flex shrink-0 flex-col gap-1" class:hidden=move || editing.get()>
                                        <button
                                            type="button"
                                            aria-label="Edit task"
                                            title="Edit task"
                                            class="rounded-lg p-1.5 text-slate-400 transition hover:bg-indigo-50 hover:text-indigo-600 focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:outline-none sm:opacity-0 sm:group-hover:opacity-100 sm:focus-visible:opacity-100 dark:hover:bg-indigo-500/10 dark:hover:text-indigo-300"
                                            on:click=move |_| start_editing()
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-5">
                                                <path d="M12 20h9M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
                                            </svg>
                                        </button>
                                        <button
                                            type="button"
                                            aria-label="Delete task"
                                            title="Delete task"
                                            class="rounded-lg p-1.5 text-slate-400 transition hover:bg-red-50 hover:text-red-600 focus-visible:ring-2 focus-visible:ring-red-500 focus-visible:outline-none max-sm:hidden sm:opacity-0 sm:group-hover:opacity-100 sm:focus-visible:opacity-100 dark:hover:bg-red-500/10 dark:hover:text-red-400"
                                            on:click=move |_| delete_task(id)
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-5">
                                                <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6M10 11v6M14 11v6" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                                </li>
                            }
                        }
                    />
                </ol>
            </Show>

            <Show when=move || last_deleted.with(Option::is_some)>
                <div
                    role="status"
                    class="fixed inset-x-4 bottom-4 z-10 mx-auto flex max-w-sm items-center justify-between gap-3 rounded-xl bg-slate-900 py-2 pr-2 pl-4 text-sm text-white shadow-lg dark:bg-slate-100 dark:text-slate-900"
                >
                    <span class="truncate">
                        "Deleted “"
                        {move || last_deleted.with(|deleted| {
                            deleted.as_ref().map(|(_, item)| item.title.get()).unwrap_or_default()
                        })}
                        "”"
                    </span>
                    <button
                        type="button"
                        class="shrink-0 rounded-lg px-3 py-1.5 font-semibold text-indigo-300 transition hover:bg-white/10 focus-visible:ring-2 focus-visible:ring-indigo-400 focus-visible:outline-none dark:text-indigo-600 dark:hover:bg-slate-900/10"
                        on:click=move |_| undo_delete()
                    >
                        "Undo"
                    </button>
                </div>
            </Show>
        </main>
    }
}

/// Inline form that edits a task's title and description drafts
#[component]
fn TaskEditForm(
    title: RwSignal<String>,
    description: RwSignal<String>,
    on_save: impl Fn() + 'static,
    on_cancel: impl Fn() + Copy + 'static,
) -> impl IntoView {
    let title_input = NodeRef::<leptos::html::Input>::new();

    // Put the cursor in the title as soon as the form appears
    Effect::new(move |_| {
        if let Some(input) = title_input.get() {
            let _ = input.focus();
        }
    });

    view! {
        <form
            class="flex min-w-0 flex-1 flex-col gap-2"
            on:submit=move |event| {
                event.prevent_default();
                on_save();
            }
            on:keydown=move |event| {
                if event.key() == "Escape" {
                    on_cancel();
                }
            }
        >
            <input
                type="text"
                aria-label="Task title"
                class="w-full rounded-lg border border-slate-300 bg-transparent px-3 py-1.5 font-semibold focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                node_ref=title_input
                bind:value=title
            />
            <textarea
                rows="2"
                aria-label="Task description"
                placeholder="Description (optional)"
                class="w-full resize-none rounded-lg border border-slate-300 bg-transparent px-3 py-1.5 text-sm placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                bind:value=description
            />
            <div class="flex justify-end gap-2">
                <button
                    type="button"
                    class="rounded-lg px-3 py-1.5 text-sm font-semibold text-slate-600 transition hover:bg-slate-100 focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:outline-none dark:text-slate-300 dark:hover:bg-slate-800"
                    on:click=move |_| on_cancel()
                >
                    "Cancel"
                </button>
                <button
                    type="submit"
                    class="rounded-lg bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white shadow-sm transition hover:bg-indigo-500 focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:focus-visible:ring-offset-slate-900"
                    disabled=move || title.with(|title| title.trim().is_empty())
                >
                    "Save"
                </button>
            </div>
        </form>
    }
}

fn initial_tasks() -> Vec<Item> {
    vec![
        Item::new(
            1,
            "Set up the Nix flake",
            "Pin the Rust toolchain, Trunk and wasm-bindgen so nothing is installed globally.",
            Status::Done,
        ),
        Item::new(
            2,
            "Add the wasm32-unknown-unknown target",
            "Trunk needs it to compile the app to WebAssembly.",
            Status::Done,
        ),
        Item::new(
            3,
            "Style the app with Tailwind",
            "Let Trunk run tailwindcss on every build.",
            Status::InProgress,
        ),
        Item::new(
            4,
            "Learn ownership and borrowing",
            "Read chapter 4 of The Rust Book and redo the exercises.",
            Status::InProgress,
        ),
        Item::new(
            5,
            "Write unit tests with cargo test",
            "Cover the task list logic before adding more features.",
            Status::Todo,
        ),
        Item::new(
            6,
            "Fix all cargo clippy warnings",
            "Run cargo clippy --target wasm32-unknown-unknown and clean up.",
            Status::Todo,
        ),
        Item::new(
            7,
            "Persist tasks in localStorage",
            "Keep the list across page reloads.",
            Status::Todo,
        ),
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
            title: item.title.get(),
            description: item.description.get(),
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
    // Editable fields are signals, so changing one updates its row in place
    title: RwSignal<String>,
    description: RwSignal<String>,
    status: RwSignal<Status>,
}

impl Item {
    fn new(id: u32, title: &str, description: &str, status: Status) -> Self {
        Self {
            id,
            title: RwSignal::new(title.to_string()),
            description: RwSignal::new(description.to_string()),
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
            Status::InProgress => {
                "bg-amber-100 text-amber-800 dark:bg-amber-500/15 dark:text-amber-300"
            }
            Status::Done => {
                "bg-emerald-100 text-emerald-800 dark:bg-emerald-500/15 dark:text-emerald-300"
            }
        }
    }
}
