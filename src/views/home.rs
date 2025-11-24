use crate::model::{datetime, Deadline};
use crate::persistence::{HomeworkRepo, NewHomework};
use crate::views::{DeadlineListView, EditDeadlineView, CalendarView};
use dioxus::prelude::*;
use std::sync::Arc;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let repo = use_context::<Arc<dyn HomeworkRepo>>();
    let mut deadlines_state = use_signal(Vec::<Deadline>::new);
    let mut selected = use_signal(|| Option::<Deadline>::None);
    let mut is_calendar_view = use_signal(|| false);
    
    // Signal to trigger reload
    let mut reload_trigger = use_signal(|| 0);

    use_effect({
        let repo = repo.clone();
        move || {
            let _ = reload_trigger(); // Subscribe
            let repo = repo.clone();
            spawn(async move {
                if let Ok(records) = repo.list() {
                    let deadlines: Vec<Deadline> = records.into_iter().map(|r| {
                        let due_date = datetime::Datetime::from_string(&r.due_text).unwrap_or_else(datetime::Datetime::now);
                        let mut d = Deadline {
                            id: r.uid,
                            name: r.name,
                            due_date,
                            difficulty: r.difficulty,
                            progress: r.progress,
                            milestones: r.milestones,
                            urgency: 0.0,
                            tags: r.tags,
                        };
                        d.update_urgency();
                        d
                    }).collect();
                    deadlines_state.set(deadlines);
                }
            });
        }
    });

    rsx! {
        div {
            class: "layout-grid",
            
            // Left Column: Deadline List or Calendar
            div {
                class: "flex flex-col gap-6",
                
                // Header
                div {
                    class: "flex justify-between items-center",
                    h2 { class: "text-2xl font-bold", "Your Deadlines" }
                    div { class: "flex items-center gap-2",
                        button {
                            class: if is_calendar_view() { "btn btn-primary" } else { "btn btn-ghost p-2" },
                            title: if is_calendar_view() { "Switch to List" } else { "Switch to Calendar" },
                            onclick: move |_| {
                                is_calendar_view.set(!is_calendar_view());
                                selected.set(None);
                            },
                            if is_calendar_view() { "📝 List" } else { "📅 Calendar" }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let new_deadline = Deadline::new("".to_string(), "".to_string(), datetime::Datetime::now(), 5);
                                selected.set(Some(new_deadline));
                            },
                            "New Deadline"
                        }
                    }
                }

                if is_calendar_view() {
                    CalendarView {
                        deadlines: deadlines_state().clone(),
                        on_select_date: move |dt: datetime::Datetime| {
                            let new_deadline = Deadline::new("".to_string(), "".to_string(), dt, 5);
                            selected.set(Some(new_deadline));
                        },
                        on_edit_deadline: move |d: Deadline| {
                            selected.set(Some(d));
                        }
                    }
                } else {
                    DeadlineListView { 
                        deadlines: deadlines_state().clone(), 
                        on_update: {
                            let repo = repo.clone();
                            move |d: Deadline| {
                                let repo = repo.clone();
                                spawn(async move {
                                    if let Ok(Some(mut rec)) = repo.get(&d.id) {
                                        rec.name = d.name;
                                        rec.due_text = d.due_date.to_string();
                                        rec.difficulty = d.difficulty;
                                        rec.progress = d.progress;
                                        rec.tags = d.tags;
                                        rec.milestones = d.milestones;
                                        let _ = repo.update(rec);
                                        reload_trigger.with_mut(|x| *x += 1);
                                    }
                                });
                            }
                        }, 
                        on_edit: move |d: Deadline| {
                            selected.set(Some(d));
                        }
                        ,
                        on_delete: {
                            let repo = repo.clone();
                            move |d: Deadline| {
                                let repo = repo.clone();
                                spawn(async move {
                                    let _ = repo.delete(&d.id);
                                });
                                reload_trigger.with_mut(|x| *x += 1);
                            }
                        }
                    }
                }
            }
            
            // Right Column: Edit Panel or Stats
            div {
                class: "card",
                style: "position: sticky; top: 1rem;",
                if let Some(sel) = selected().clone() {
                    EditDeadlineView { 
                        key: "{sel.id}", 
                        deadline: sel.clone(), 
                        on_save: {
                            let repo = repo.clone();
                            move |d: Deadline| {
                                let repo = repo.clone();
                                spawn(async move {
                                    if d.id.is_empty() {
                                        let payload = NewHomework {
                                            name: d.name,
                                            due_text: d.due_date.to_string(),
                                            difficulty: d.difficulty,
                                            progress: d.progress,
                                            tags: d.tags,
                                            milestones: d.milestones,
                                        };
                                        let _ = repo.create(payload);
                                    } else {
                                        if let Ok(Some(mut rec)) = repo.get(&d.id) {
                                            rec.name = d.name;
                                            rec.due_text = d.due_date.to_string();
                                            rec.difficulty = d.difficulty;
                                            rec.progress = d.progress;
                                            rec.tags = d.tags;
                                            rec.milestones = d.milestones;
                                            let _ = repo.update(rec);
                                        }
                                    }
                                    reload_trigger.with_mut(|x| *x += 1);
                                    selected.set(None);
                                });
                            }
                        }, 
                        on_cancel: move |_| selected.set(None) 
                    }
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
