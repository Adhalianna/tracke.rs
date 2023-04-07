use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Stylesheet href="pkg/trackers-web-ui.css"/>

        // sets the document title
        <Title text="Tracke.rs Task Management App"/>

        // content for this welcome page
        <Router>
            <Routes>
                <Route path="" view=|cx| view! { cx, <UserHomepage/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn UserHomepage(cx: Scope) -> impl IntoView {
    view! { cx,
        <main><Tracker/></main>
        <TopNavBar/>
        <BottomBar/>
    }
}

#[component]
fn TopNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bar top">
            <div class="container flex">
            <nav>
                <ul>
                <li class="nav"><a >"El1"</a></li>
                <li class="nav">"El2"</li>
                </ul>
            </nav>
            </div>
        </div>
    }
}

#[component]
fn BottomBar(cx: Scope) -> impl IntoView {
    // let on_click = move |_|
    view! {cx,
        <menu class="bar bottom">
            <span class="container flex">
                <span class="task-input">
                    <input type="text" id="title" spellcheck=true name="task title" required maxlength=256></input>
                </span>
                <span class="container flex">
                    <button><LeptosIcon icon=FiIcon::FiPlusSquare /></button>
                    <button><LeptosIcon icon=FiIcon::FiMoreHorizontal /></button>
                </span>
            </span>
        </menu>
    }
}

#[component]
fn Tracker(cx: Scope) -> impl IntoView {
    use crate::api;
    use trackers_models::Task;

    let tasks = create_resource(
        cx,
        || (),
        move |_| async move { api::fetch_api::<Vec<Task>>(cx, "/api/hello").await },
    );

    view! { cx,
        <span class="tracker">
            <div class="container flex">
                <Suspense fallback=|| view! { cx, "Loading tasks..." }>
                    {move || tasks.read(cx).map(|tasks| match tasks {
                        None => view! { cx, <div class="err">"Failed to load tasks."</div> },
                        Some(tasks) => view! { cx,
                            <div class="tracker">
                                <For
                                    each=move || tasks.clone()
                                    key=|task| task.task_id
                                    view=move |cx, task| view! {cx, <Task task/> }
                                />
                            </div>
                        }
                    })

                    }
                </Suspense>
            </div>
        </span>
    }
}

#[component]
fn Task(cx: Scope, task: trackers_models::Task) -> impl IntoView {
    let (details, set_details_open) = create_signal(cx, false);
    let on_details_click = move |_| {
        set_details_open.update(|open| {
            *open = !*open;
        })
    };

    view! { cx,
        <div class="task container">
            <div class="task head">
                <span>
                    <div class="task text task_id"> {task.task_id.to_string()} </div>
                    <div class="task text title"> {task.title} </div>
                </span>
                <span>
                    <div><button><LeptosIcon icon=FiIcon::FiCheckCircle /></button></div>
                    <div><button><LeptosIcon icon=FiIcon::FiEdit /></button></div>
                    <div><button on:click=on_details_click><LeptosIcon icon=FiIcon::FiMoreHorizontal /></button></div>
                </span>
            </div>
            {move || if details() {
                view! {cx,
                    <div class="container">
                        <span class="task text description"> {task.description.clone()} </span>
                        <span class="task text tags">{task.tags.clone()}</span>
                    </div>
                }
            } else {
                view! {cx, <div></div>}
            }}
        </div>
    }
}
