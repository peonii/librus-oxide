use anyhow::Result;
use chrono::{DateTime, Datelike, Local};
use serde::{Deserialize, Serialize};

use crate::{
    client::Librus,
    common::{NumberIDResource, StringIDResource},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarInitResponse {
    #[serde(alias = "Calendars")]
    pub calendars: Vec<StringIDResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarResponse {
    #[serde(alias = "Calendar")]
    pub calendar: Calendar,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calendar {
    #[serde(alias = "SchoolFreeDays")]
    pub school_free_days: Vec<NumberIDResource>,

    #[serde(alias = "TeacherFreeDays")]
    pub teacher_free_days: Vec<NumberIDResource>,

    #[serde(alias = "HomeWorks")]
    pub homeworks: Vec<NumberIDResource>,

    #[serde(alias = "Substitutions")]
    pub substitutions: Vec<NumberIDResource>,

    #[serde(alias = "ParentTeacherConferences")]
    pub parent_teacher_conferences: Vec<NumberIDResource>,

    #[serde(alias = "ClassFreeDays")]
    pub class_free_days: Vec<NumberIDResource>,
}

impl Librus {
    pub async fn calendar_from_date(&self, date: DateTime<Local>) -> Result<Calendar> {
        let init_response = self
            .request::<CalendarInitResponse>("https://api.librus.pl/3.0/Calendars")
            .await?;

        if init_response.calendars.is_empty() {
            return Err(anyhow::anyhow!("No calendars found!"));
        }

        let calendar_id = init_response.calendars[0].id.clone();

        let response = self
            .request::<CalendarResponse>(&format!(
                "https://api.librus.pl/3.0/Calendars/{}?month={}&year={}",
                calendar_id,
                date.month(),
                date.year()
            ))
            .await?;

        Ok(response.calendar)
    }
}
