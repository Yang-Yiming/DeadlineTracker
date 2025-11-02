use crate::components::{Echo, Hero};
use crate::model::{Deadline, datetime};
use crate::views::DeadlineItemView;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Create sample deadlines for demonstration
    let mut deadline1 = use_signal(|| Deadline {
        id: 1,
        name: "DSAA Project".to_string(),
        due_date: datetime::Datetime {
            year: 2025,
            month: 11,
            day: 4,
            hour: 16,
            minute: 0,
        },
        difficulty: 8,
        progress: 80,
        milestones: vec![(50, "Midpoint Review".to_string())],
        urgency: 5.5,
        tags: vec!["Important".to_string(), "Academic".to_string()],
    });
    
    let mut deadline2 = use_signal(|| Deadline {
        id: 2,
        name: "Physics Exam".to_string(),
        due_date: datetime::Datetime {
            year: 2025,
            month: 11,
            day: 6,
            hour: 10,
            minute: 0,
        },
        difficulty: 7,
        progress: 45,
        milestones: vec![(25, "Review Chapters 1-3".to_string()), (75, "Full Practice Test".to_string())],
        urgency: 2.3,
        tags: vec!["Exam".to_string()],
    });
    
    let mut deadline3 = use_signal(|| Deadline {
        id: 3,
        name: "Low Priority Task".to_string(),
        due_date: datetime::Datetime {
            year: 2025,
            month: 11,
            day: 15,
            hour: 23,
            minute: 59,
        },
        difficulty: 3,
        progress: 10,
        milestones: vec![],
        urgency: 0.5,
        tags: vec!["Optional".to_string()],
    });

    rsx! {
        Hero {}
        Echo {}
        
        div {
            style: "padding: 20px;",
            h2 { "Your Deadlines" }
            DeadlineItemView { deadline: deadline1(), on_update: move |d: Deadline| deadline1.set(d) }
            DeadlineItemView { deadline: deadline2(), on_update: move |d: Deadline| deadline2.set(d) }
            DeadlineItemView { deadline: deadline3(), on_update: move |d: Deadline| deadline3.set(d) }
        }
    }
}