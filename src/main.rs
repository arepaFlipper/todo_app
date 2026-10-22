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
        <main class="mx-auto flex max-w-xl flex-col gap-6 px-4 py-12 sm:py-20">
            <header class="flex items-end justify-between">
                <div>
                    <p class="text-sm font-medium text-indigo-600 dark:text-indigo-400">"Leptos + Tailwind"</p>
                    <h1 class="text-3xl font-bold tracking-tight">"My tasks"</h1>
                </div>
                <span class="rounded-full bg-indigo-100 px-3 py-1 text-sm font-semibold text-indigo-700 dark:bg-indigo-500/15 dark:text-indigo-300">
                    {move || tasks.with(|tasks| tasks.len())} " items"
                </span>
            </header>

            <section class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900">
                <div class="flex flex-col gap-3">
                    <input
                        type="text"
                        placeholder="What needs to be done?"
                        class="w-full rounded-lg border border-slate-300 bg-transparent px-4 py-2.5 text-base font-semibold placeholder:font-normal placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                        on:input=move|event| {
                            task_writer.write().title = event_target_value(&event);
                        }
                    />
                    <textarea
                        rows="3"
                        placeholder="Add a description (optional)"
                        class="w-full resize-none rounded-lg border border-slate-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/30 focus:outline-none dark:border-slate-700"
                        on:input=move|event| {
                            task_writer.write().description = event_target_value(&event);
                        }
                    />
                </div>
                <button
                    class="mt-4 w-full rounded-lg bg-indigo-600 px-4 py-2.5 font-semibold text-white shadow-sm transition hover:bg-indigo-500 focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:outline-none active:scale-[0.99] dark:focus-visible:ring-offset-slate-900"
                    on:click=move |_| {
                        counter_writer.set(counter.get() + 1);

                        tasks_writer.write().push(Item {
                            id: counter.get(),
                            title: task.get().title,
                            description: task.get().description
                        });
                }>"Add task"</button>
            </section>

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
                        each = move || tasks.get()
                        key = |task| task.id
                        children = move |task| {
                            view! {
                                <li class="flex gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm transition hover:border-indigo-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-indigo-500/50">
                                    <span class="flex size-8 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-sm font-bold text-indigo-700 dark:bg-indigo-500/15 dark:text-indigo-300">
                                        {task.id}
                                    </span>
                                    <div class="min-w-0">
                                        <h3 class="truncate font-semibold">{task.title}</h3>
                                        <p class="mt-1 text-sm whitespace-pre-line text-slate-600 dark:text-slate-400">{task.description}</p>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ol>
            </Show>
        </main>
    }
}

#[derive(Clone, Default)]
struct Item {
    id: u32,
    title: String,
    description: String,
}
