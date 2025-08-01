//! Signals can degrade into a Read variant automatically
//! This is done thanks to a conversion by the #[component] macro

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        Child {
            count,
            "hiiii"
        }
    }
}

#[component]
fn Child(count: ReadSignal<i32>, children: Element) -> Element {
    rsx! {
        div { "Count: {count}" }
        {children}
    }
}
