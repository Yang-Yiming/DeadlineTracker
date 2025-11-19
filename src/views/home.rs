use crate::model::{datetime, Deadline};

use crate::views::{DeadlineListView, EditDeadlineView};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Create sample deadlines for demonstration and store them in state
    let mut deadlines_state = use_signal(|| {
        let d1 = Deadline {
            id: 1,
            name: "DSAA Project".to_string(),
            due_date: datetime::Datetime { year: 2025, month: 11, day: 4, hour: 16, minute: 0 },
            difficulty: 8,
            progress: 80,
            milestones: vec![(50, "Midpoint Review".to_string())],
            urgency: 5.5,
            tags: vec!["Important".to_string(), "Academic".to_string()],
        };
        let d2 = Deadline {
            id: 2,
            name: "Physics Exam".to_string(),
            due_date: datetime::Datetime { year: 2025, month: 11, day: 6, hour: 10, minute: 0 },
            difficulty: 7,
            progress: 45,
            milestones: vec![(25, "Review Chapters 1-3".to_string()), (75, "Full Practice Test".to_string())],
            urgency: 2.3,
            tags: vec!["Exam".to_string()],
        };
        let d3 = Deadline {
            id: 3,
            name: "Low Priority Task".to_string(),
            due_date: datetime::Datetime { year: 2025, month: 11, day: 15, hour: 23, minute: 59 },
            difficulty: 3,
            progress: 10,
            milestones: vec![],
            urgency: 0.5,
            tags: vec!["Optional".to_string()],
        };
        vec![d1, d2, d3]
    });

    // Selected deadline for editing
    let mut selected = use_signal(|| Option::<Deadline>::None);

    // inline handlers will be used below for clarity

    // cancel is handled inline when rendering the EditDeadlineView

    rsx! {
        div {
            class: "layout-grid",
            
            // Left Column: Deadline List
            div {
                class: "flex flex-col gap-6",
                
                // Header
                div {
                    class: "flex justify-between items-center",
                    h2 { class: "text-2xl font-bold", "Your Deadlines" }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let new_deadline = Deadline::new(0, "".to_string(), datetime::Datetime::now(), 5);
                            selected.set(Some(new_deadline));
                        },
                        "New Deadline"
                    }
                }

                DeadlineListView { deadlines: deadlines_state().clone(), on_update: move |d: Deadline| {
                    let mut v = deadlines_state().clone();
                    for dd in v.iter_mut() {
                        if dd.id == d.id { *dd = d.clone(); }
                    }
                    deadlines_state.set(v);
                }, on_edit: move |d: Deadline| selected.set(Some(d)) }
            }
            
            // Right Column: Edit Panel or Stats
            div {
                class: "card",
                style: "position: sticky; top: 1rem;",
                if let Some(sel) = selected().clone() {
                    EditDeadlineView { key: "{sel.id}", deadline: sel.clone(), on_save: move |d: Deadline| {
                        let mut v = deadlines_state().clone();
                        let mut found = false;
                        for slot in v.iter_mut() {
                            if slot.id == d.id { *slot = d.clone(); found = true; break; }
                        }
                        if !found { let mut newd = d.clone(); let next_id = v.iter().map(|x| x.id).max().unwrap_or(0) + 1; newd.id = next_id; v.push(newd); }
                        deadlines_state.set(v);
                        selected.set(None);
                    }, on_cancel: move |_| selected.set(None) }
                } else {
                    // Statistics View
                    {
                        let all = deadlines_state();
                        let total = all.len();
                        let completed = all.iter().filter(|d| d.progress == 100).count();
                        let in_progress = all.iter().filter(|d| d.progress > 0 && d.progress < 100).count();
                        let not_started = all.iter().filter(|d| d.progress == 0).count();
                        let avg_progress = if total > 0 { all.iter().map(|d| d.progress as f32).sum::<f32>() / total as f32 } else { 0.0 };
                        
                        rsx! {
                            div {
                                class: "flex flex-col gap-6",
                                div {
                                    class: "flex justify-between items-center border-b pb-4",
                                    h3 { class: "text-xl font-bold", "Statistics" }
                                    span { class: "text-sm text-gray-500", "{total} Items" }
                                }

                                // Overall Progress
                                div {
                                    class: "flex flex-col gap-2",
                                    div {
                                        class: "flex justify-between text-sm font-medium",
                                        span { "Overall Progress" }
                                        span { class: "text-primary-600", "{avg_progress.round()}%" }
                                    }
                                    div {
                                        class: "progress-track",
                                        div {
                                            class: "progress-fill",
                                            style: "width: {avg_progress}%; background-color: var(--primary-600);",
                                        }
                                    }
                                }

                                // Grid of stats
                                div {
                                    class: "grid-list",
                                    style: "grid-template-columns: 1fr 1fr; gap: 1rem;",
                                    
                                    div {
                                        class: "bg-gray-50 p-3 rounded-lg border border-gray-100",
                                        div { class: "text-xs text-gray-500 uppercase font-semibold", "Completed" }
                                        div { class: "text-2xl font-bold text-green-600", style: "color: #059669;", "{completed}" }
                                    }
                                    div {
                                        class: "bg-gray-50 p-3 rounded-lg border border-gray-100",
                                        div { class: "text-xs text-gray-500 uppercase font-semibold", "In Progress" }
                                        div { class: "text-2xl font-bold text-blue-600", style: "color: #2563eb;", "{in_progress}" }
                                    }
                                    div {
                                        class: "bg-gray-50 p-3 rounded-lg border border-gray-100",
                                        div { class: "text-xs text-gray-500 uppercase font-semibold", "Not Started" }
                                        div { class: "text-2xl font-bold text-gray-600", "{not_started}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
