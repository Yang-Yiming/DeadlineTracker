use crate::model::Deadline;
use crate::views::DeadlineItemView;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SortType {
    DueDate,
    Urgency,
    Progress,
}

fn sorted_deadlines(input: &[Deadline], sort: SortType) -> Vec<Deadline> {
    let mut v: Vec<Deadline> = input.iter().map(|d| (*d).clone()).collect();
    match sort {
        SortType::DueDate => {
            v.sort_by(|a, b| a.due_date.cmp(&b.due_date));
        }
        SortType::Urgency => {
            // Highest urgency first
            v.sort_by(|a, b| {
                b.urgency
                    .partial_cmp(&a.urgency)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortType::Progress => {
            // Lowest progress first
            v.sort_by(|a, b| a.progress.cmp(&b.progress));
        }
    }
    v
}

#[component]
pub fn DeadlineListView(deadlines: Vec<Deadline>, mut on_update: EventHandler<Deadline>, mut on_edit: EventHandler<Deadline>, mut on_delete: EventHandler<Deadline>) -> Element {
    let mut sort = use_signal(|| SortType::Urgency);
    let mut search = use_signal(|| String::new());

    let query = search().trim().to_lowercase();
    let filtered: Vec<Deadline> = deadlines
        .iter()
        .filter(|deadline| {
            if query.is_empty() {
                return true;
            }
            // Match query against name or any tag (case-insensitive).
            deadline.name.to_lowercase().contains(&query)
                || deadline
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query))
        })
        .cloned()
        .collect();

    let sorted = sorted_deadlines(&filtered, sort());

    rsx! {
        div {
            class: "flex flex-col gap-4",

            // Sorting controls
            div {
                class: "sort-controls",
                style: "display: flex; align-items: center; gap: 0.5rem; width: 100%;",
                span { class: "font-bold text-gray-600", "Sort by:" }
                
                button {
                    class: if sort() == SortType::DueDate { "sort-btn active" } else { "sort-btn" },
                    onclick: move |_| sort.set(SortType::DueDate),
                    "Due date"
                }
                button {
                    class: if sort() == SortType::Urgency { "sort-btn active" } else { "sort-btn" },
                    onclick: move |_| sort.set(SortType::Urgency),
                    "Urgency"
                }
                button {
                    class: if sort() == SortType::Progress { "sort-btn active" } else { "sort-btn" },
                    onclick: move |_| sort.set(SortType::Progress),
                    "Progress"
                }

                div {
                    class: "sort-search",
                    input {
                        r#type: "text",
                        class: "search-input",
                        placeholder: "Search name or tag",
                        value: "{search()}",
                        oninput: move |e| search.set(e.value().clone()),
                    }
                }
            }

            // Render items
            div {
                class: "flex flex-col gap-4",
                { sorted.into_iter().map(|d| {
                    let deadline_clone = d.clone();
                    let id = deadline_clone.id.clone();
                    rsx! {
                        DeadlineItemView {
                            key: "{id}",
                            deadline: deadline_clone,
                            on_update: move |d| on_update.call(d),
                            on_edit: move |d| on_edit.call(d),
                            on_delete: move |d| on_delete.call(d),
                        }
                    }
                }) }
            }
        }
    }
}
