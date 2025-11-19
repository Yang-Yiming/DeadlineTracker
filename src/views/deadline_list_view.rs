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
pub fn DeadlineListView(deadlines: Vec<Deadline>, mut on_update: EventHandler<Deadline>, mut on_edit: EventHandler<Deadline>) -> Element {
    let mut sort = use_signal(|| SortType::Urgency);

    let sorted = sorted_deadlines(&deadlines, sort());

    rsx! {
        div {
            class: "flex flex-col gap-4",

            // Sorting controls
            div {
                class: "sort-controls",
                span { class: "font-bold text-gray-600", style: "padding: 0 0.5rem;", "Sort by:" }
                
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
            }

            // Render items
            div {
                class: "flex flex-col gap-4",
                { sorted.into_iter().map(|d| {
                    let id = d.id;
                    rsx! {
                        DeadlineItemView {
                            key: "{id}",
                            deadline: d,
                            on_update: move |d| on_update.call(d),
                            on_edit: move |d| on_edit.call(d),
                        }
                    }
                }) }
            }
        }
    }
}
