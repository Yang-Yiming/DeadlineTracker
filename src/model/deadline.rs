use super::datetime::Datetime;

#[derive(Clone, Debug, PartialEq)]
pub struct Deadline {
    pub id: String,
    pub name: String,
    pub due_date: Datetime,
    pub difficulty: u8,
    pub progress: u8, // percentage from 0 to 100
    pub milestones: Vec<(u8, String)>,
    pub urgency: f32, // calculated
    pub tags: Vec<String>,
}

#[allow(dead_code)]
impl Deadline {
    pub fn new(id: String, name: String, due_date: Datetime, difficulty: u8) -> Self {
        Self {
            id,
            name,
            due_date,
            difficulty,
            progress: 0,
            milestones: Vec::new(),
            urgency: 0.0,
            tags: Vec::new(),
        }
    }

    pub fn hours_until_due(&self) -> f32 {
        let now = Datetime::now();
        self.due_date.time_diff(&now).to_hours()
    }
    
    pub fn update_urgency(&mut self) -> f32 {
        let hours_left = self.hours_until_due();
        let delta = 0.0001; // safe.
        let hours_left_safe = if hours_left < delta { delta } else { hours_left };
        self.urgency = self.difficulty as f32 * (100.0 - self.progress as f32) / hours_left_safe;
        self.urgency
    }
}
