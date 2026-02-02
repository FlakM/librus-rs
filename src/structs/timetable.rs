//! Timetable data types.
//!
//! Note: This module is not yet fully implemented in the public API.

#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TimetableLesson {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableClassroom {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableEntry {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableLessonSubject {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Short")]
    pub short: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableTeacher {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableClass {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct TimetableDay {
    #[serde(rename = "Lesson")]
    pub lesson: Option<TimetableLesson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Timetable")]
pub struct Timetable {
    pub timetable: Option<HashMap<String, Vec<Vec<TimetableDay>>>>,
}

#[derive(Debug, Deserialize)]
pub struct TimetablePages {
    #[serde(rename = "Next")]
    pub next: String,
    #[serde(rename = "Prev")]
    pub prev: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetablesUrl {
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableResources {
    #[serde(rename = "Timetables\\IndividualLearningPath")]
    pub individual_learning_path: TimetablesUrl,
    #[serde(rename = "Timetables\\OneToOneLearningPlan")]
    pub onetoone_learning_plan: TimetablesUrl,
    #[serde(rename = "Timetables\\OtherActivitiesRegister")]
    pub other_activities_register: TimetablesUrl,
    #[serde(rename = "..")]
    pub root: TimetablesUrl,
}

#[derive(Debug, Deserialize)]
pub struct ResponseTimetable {
    #[serde(rename = "Timetable")]
    pub timetable: Timetable,
    #[serde(rename = "Pages")]
    pub pages: TimetablePages,
    #[serde(rename = "Resources")]
    pub resources: TimetableResources,
    #[serde(rename = "Url")]
    pub url: String,
}
