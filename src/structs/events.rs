//! Homework and event data types.

use serde::Deserialize;

/// Response containing all homeworks.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseHomeworks {
    /// List of homework assignments.
    #[serde(rename = "HomeWorks")]
    pub homeworks: Vec<Homework>,
    /// Related API resources.
    pub resources: Option<HomeworksResources>,
    /// API URL for this response.
    pub url: String,
}

/// A homework assignment.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Homework {
    /// Unique homework identifier.
    pub id: i64,
    /// Homework content/description.
    pub content: String,
    /// Due date.
    pub date: String,
    /// Reference to the homework category.
    pub category: HomeworksCategory,
    /// Lesson number when assigned.
    pub lesson_no: Option<String>,
    /// Start time.
    pub time_from: String,
    /// End time.
    pub time_to: String,
    /// Reference to the teacher who created this homework.
    pub created_by: HomeworksCategory,
    /// Reference to the class.
    pub class: Option<HomeworksCategory>,
    /// Reference to the subject.
    pub subject: Option<HomeworksCategory>,
    /// Date when the homework was added.
    pub add_date: String,
    /// Classroom information.
    pub classroom: Option<HomeworksClassroom>,
}

/// Reference to a homework-related resource.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksCategory {
    /// Resource ID.
    pub id: i64,
    /// API URL to fetch the resource.
    pub url: String,
}

/// Classroom information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksClassroom {
    /// Classroom ID.
    pub id: i64,
    /// Classroom symbol/code.
    pub symbol: String,
    /// Classroom name.
    pub name: String,
    /// Classroom capacity.
    pub size: i64,
}

#[derive(Debug, Deserialize)]
pub struct HomeworksResources {
    #[serde(rename = "HomeWorks\\Categories")]
    pub homeworks_categories: HomeworksUrl,
    #[serde(rename = "..")]
    pub empty: HomeworksUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksUrl {
    pub url: String,
}
