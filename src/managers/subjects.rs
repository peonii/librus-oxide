use serde::{Deserialize, Serialize};
use crate::client::Librus;
use crate::common::NumberIDResource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    #[serde(alias = "Id")]
    id: i32,

    #[serde(alias = "Name")]
    name: String,

    #[serde(alias = "No")]
    number: i32,

    #[serde(alias = "Short")]
    short_name: String,

    #[serde(alias = "IsExtracurricular")]
    extracurricular: bool,

    #[serde(alias = "IsBlockLesson")]
    block_lesson: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRequest {
    #[serde(alias = "Subjects")]
    subjects: Vec<Subject>
}

impl Librus {
    pub async fn get_subjects_by_ids(&self, ids: Vec<String>) -> anyhow::Result<Vec<Subject>> {
        let joined_ids = ids.join(",");

        let response = self.request::<SubjectRequest>(&format!("https://api.librus.pl/3.0/Subjects?ids={}", joined_ids)).await?;

        Ok(response.subjects)
    }

    pub async fn get_subjects_by_numberids(&self, resources: Vec<NumberIDResource>) -> anyhow::Result<Vec<Subject>> {
        let ids = resources.iter().map(|r| r.id.to_string()).collect::<Vec<String>>();

        self.get_subjects_by_ids(ids).await
    }
}