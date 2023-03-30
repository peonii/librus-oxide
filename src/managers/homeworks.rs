use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{client::Librus, common::NumberIDResource};

use super::calendars::Calendar;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomeworkMultiResponse {
    #[serde(alias = "HomeWorks")]
    pub homeworks: Vec<Homework>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Homework {
    #[serde(alias = "Id")]
    pub id: i32,

    #[serde(alias = "Content")]
    pub content: String,

    #[serde(alias = "Date")]
    pub date: String,

    #[serde(alias = "Category")]
    pub category: NumberIDResource,

    #[serde(alias = "LessonNo")]
    pub lesson_number: String,

    #[serde(alias = "TimeFrom")]
    pub time_from: String,

    #[serde(alias = "TimeTo")]
    pub time_to: String,

    #[serde(alias = "CreatedBy")]
    pub created_by: NumberIDResource,

    #[serde(alias = "Class")]
    pub class: Option<NumberIDResource>,

    #[serde(alias = "Subject")]
    pub subject: NumberIDResource,

    #[serde(alias = "AddDate")]
    pub add_date: String,

    #[serde(alias = "VirtualClass")]
    pub virtual_class: Option<NumberIDResource>,
}

impl Librus {
    pub async fn homeworks_from_calendar(&self, calendar: Calendar) -> Result<Vec<Homework>> {
        let homework_ids = calendar
            .homeworks
            .iter()
            .map(|hmk| hmk.id.to_string())
            .collect::<Vec<String>>();

        let homework_full_ids = homework_ids.join(",");

        let url = format!("https://api.librus.pl/3.0/HomeWorks/{}", homework_full_ids);

        let homeworks = self.request::<HomeworkMultiResponse>(&url).await?;

        Ok(homeworks.homeworks)
    }
}
