use crate::model::{datetime::Datetime, Deadline};
use dioxus::prelude::*;

fn parse_due_date(s: &str) -> Option<Datetime> {
    // Expect "YYYY-MM-DD HH:MM"
    let s = s.trim();
    if s.len() < 16 { return None; }
    let date = &s[..10];
    let time = &s[11..16];
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 { return None; }
    let year = parts[0].parse::<u16>().ok()?;
    let month = parts[1].parse::<u8>().ok()?;
    let day = parts[2].parse::<u8>().ok()?;
    let time_parts: Vec<&str> = time.split(':').collect();
    if time_parts.len() != 2 { return None; }
    let hour = time_parts[0].parse::<u8>().ok()?;
    let minute = time_parts[1].parse::<u8>().ok()?;
    Some(Datetime::new(year, month, day, hour, minute))
}

#[component]
pub fn EditDeadlineView(
    deadline: Deadline,
    on_save: EventHandler<Deadline>,
    on_cancel: EventHandler<()>,
) -> Element {
    // Local temporary state for editing
    let mut name = use_signal(|| deadline.name.clone());
    let mut due = use_signal(|| deadline.due_date.to_string());
    let mut difficulty = use_signal(|| deadline.difficulty);
    let mut progress = use_signal(|| deadline.progress);
    let mut tags = use_signal(|| deadline.tags.join(", "));

    rsx! {
        div {
            class: "flex flex-col gap-4",
            
            div {
                class: "flex justify-between items-center",
                style: "border-bottom: 1px solid var(--gray-200); padding-bottom: 0.5rem; margin-bottom: 0.5rem;",
                h3 { class: "text-xl font-bold", "Edit Deadline" }
                button {
                    class: "btn-icon",
                    onclick: move |_| on_cancel.call(()),
                    "✕"
                }
            }

            div {
                class: "form-group",
                label { class: "form-label", "Name" }
                input {
                    r#type: "text",
                    class: "form-input",
                    value: "{name}",
                    oninput: move |e| name.set(e.value().clone()),
                }
            }
            div {
                class: "form-group",
                label { class: "form-label", "Due (YYYY-MM-DD HH:MM)" }
                input {
                    r#type: "text",
                    class: "form-input",
                    value: "{due}",
                    oninput: move |e| due.set(e.value().clone()),
                }
            }
            div {
                class: "form-group",
                label { class: "form-label", "Difficulty (1-10)" }
                input {
                    r#type: "number",
                    min: "1",
                    max: "10",
                    class: "form-input",
                    value: "{difficulty}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u8>() { difficulty.set(v); }
                    },
                }
            }
            div {
                class: "form-group",
                label { class: "form-label", "Progress ({progress}%)" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    class: "w-full",
                    style: "cursor: pointer;",
                    value: "{progress}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u8>() { progress.set(v); }
                    },
                }
            }
            div {
                class: "form-group",
                label { class: "form-label", "Tags (comma separated)" }
                input {
                    r#type: "text",
                    class: "form-input",
                    value: "{tags}",
                    oninput: move |e| tags.set(e.value().clone()),
                }
            }
            div {
                class: "flex justify-end gap-3",
                style: "margin-top: 1rem;",
                button { 
                    class: "btn btn-secondary",
                    onclick: move |_| on_cancel.call(()) , 
                    "Cancel" 
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        // Build and validate
                        if let Some(parsed) = parse_due_date(&due()) {
                            let mut new = deadline.clone();
                            new.name = name();
                            new.due_date = parsed;
                            new.difficulty = difficulty();
                            new.progress = progress();
                            new.tags = tags().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                            new.update_urgency();
                            on_save.call(new);
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
