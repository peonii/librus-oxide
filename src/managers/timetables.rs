use crate::client::Librus;
use chrono::prelude::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use crate::common::{NumberIDResource, StringIDResource};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimetableResponse {
    #[serde(alias = "Timetable")]
    pub timetable: HashMap<String, TimetableDay>
}

pub type TimetableDay = Vec<Vec<TimetableEvent>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimetableSubject {
    #[serde(alias = "Id")]
    pub id: String,

    #[serde(alias = "Name")]
    pub name: String,

    #[serde(alias = "Short")]
    pub short_name: String,

    #[serde(alias = "Url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimetableTeacher {
    #[serde(alias = "Id")]
    pub id: String,

    #[serde(alias = "FirstName")]
    pub first_name: String,

    #[serde(alias = "LastName")]
    pub last_name: String,

    #[serde(alias = "Url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimetableEvent {
    #[serde(alias = "Lesson")]
    pub lesson: StringIDResource,

    #[serde(alias = "Classroom")]
    pub classroom: Option<StringIDResource>,

    #[serde(alias = "DateFrom")]
    pub date_from: String,

    #[serde(alias = "DateTo")]
    pub date_to: String,

    #[serde(alias = "LessonNo")]
    pub lesson_number: String,

    #[serde(alias = "TimetableEntry")]
    pub timetable_entry: StringIDResource,

    #[serde(alias = "DayNo")]
    pub day_number: String,

    #[serde(alias = "Subject")]
    pub subject: TimetableSubject,

    #[serde(alias = "Teacher")]
    pub teacher: TimetableTeacher,

    #[serde(alias = "IsSubstitutionClass")]
    pub substitution: bool,

    #[serde(alias = "IsCanceled")]
    pub cancelled: bool,

    #[serde(alias = "SubstitutionNote")]
    pub substitution_note: Option<String>,

    #[serde(alias = "HourFrom")]
    pub hour_from: String,

    #[serde(alias = "HourTo")]
    pub hour_to: String,

    #[serde(alias = "VirtualClass")]
    pub virtual_class: Option<NumberIDResource>,

    #[serde(alias = "VirtualClassName")]
    pub virtual_class_name: Option<String>,

    #[serde(alias = "Class")]
    pub class: Option<NumberIDResource>,

    #[serde(alias = "OrgDate")]
    pub original_date: Option<String>,

    #[serde(alias = "OrgLessonNo")]
    pub original_lesson_number: Option<String>,

    #[serde(alias = "OrgHourFrom")]
    pub original_hour_from: Option<String>,

    #[serde(alias = "OrgHourTo")]
    pub original_hour_to: Option<String>,

    #[serde(alias = "OrgSubject")]
    pub original_subject: Option<TimetableSubject>,

    #[serde(alias = "OrgTeacher")]
    pub original_teacher: Option<TimetableTeacher>,

    #[serde(alias = "OrgLesson")]
    pub original_lesson: Option<StringIDResource>,
}

impl Librus {
    pub async fn fetch_week(&self, day: DateTime<Local>) -> Result<TimetableResponse> {
        let difference_to_monday = day.weekday().number_from_monday() as i64 - 1;
        let monday = day - chrono::Duration::days(difference_to_monday);

        let monday = monday.format("%Y-%m-%d").to_string();

        let url = format!("https://api.librus.pl/3.0/Timetables?weekStart={}", monday);

        let response = self.request::<TimetableResponse>(&url).await?;

        Ok(response)
    }

    pub async fn fetch_day(&self, day: DateTime<Local>) -> Result<TimetableDay> {
        let response = self.fetch_week(day).await?;

        let day = day.format("%Y-%m-%d").to_string();

        let day = response.timetable.get(&day).unwrap();

        Ok(day.clone())
    }
}