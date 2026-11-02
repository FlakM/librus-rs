//! Text (descriptive / formative) grade data types.
//!
//! Librus stores two families of grades:
//!
//! * numeric grades, exposed by [`Client::grades`](crate::Client::grades), and
//! * text grades, which carry a teacher-written description. Text grades come
//!   in two flavours:
//!   - "oceny bieżące" with a category (e.g. `aktywność`, `sprawdzian`), and
//!   - "oceny kształtujące" tied to a skill (e.g. `Czytanie`).
//!
//! [`Client::base_text_grades`](crate::Client::base_text_grades) returns both
//! flavours in one response, which makes it the simplest way to observe every
//! text grade for a student.

use serde::Deserialize;

use crate::serde_helpers::string_or_int_opt;

/// A reference to another grade-related resource.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextGradeRef {
    /// Resource ID.
    pub id: i32,
    /// API URL to fetch the resource.
    pub url: String,
}

/// A text grade written by a teacher.
///
/// Depending on the endpoint and the grade's origin, either `category` or
/// `skill` is present:
///
/// * `category` — used by "oceny bieżące" (e.g. `aktywność`, `sprawdzian`),
/// * `skill` — used by "oceny kształtujące" (e.g. `Czytanie`, `Mówienie`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextGrade {
    /// Unique grade identifier.
    pub id: i64,
    /// Reference to the lesson this grade is from.
    pub lesson: TextGradeRef,
    /// Reference to the subject.
    pub subject: TextGradeRef,
    /// Reference to the student who received this grade.
    pub student: TextGradeRef,
    /// Reference to the teacher who added this grade.
    pub added_by: TextGradeRef,
    /// Category reference (present for category-based text grades).
    pub category: Option<TextGradeRef>,
    /// Skill reference (present for skill-based text grades).
    pub skill: Option<TextGradeRef>,
    /// The teacher-written grade description.
    pub grade: String,
    /// Date when the grade was given (`YYYY-MM-DD`).
    pub date: String,
    /// Date and time when the grade was added to the system.
    pub add_date: String,
    /// Semester number (1 or 2).
    pub semester: i64,
    /// Whether the grade is visible in the student's grades view.
    #[serde(default)]
    pub show_in_grades_view: bool,
    /// Whether this is a semester grade. Only present on `TextGrades`.
    #[serde(default)]
    pub is_semester: bool,
    /// Whether this is a proposed semester grade. Only present on `TextGrades`.
    #[serde(default)]
    pub is_semester_proposition: bool,
    /// Whether this is a final grade. Only present on `TextGrades`.
    #[serde(default)]
    pub is_final: bool,
    /// Whether this is a proposed final grade. Only present on `TextGrades`.
    #[serde(default)]
    pub is_final_proposition: bool,
}

/// Response containing text grades.
///
/// Used by `BaseTextGrades`, `TextGrades` and `DescriptiveTextGrades`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseTextGrades {
    /// List of text grades.
    pub grades: Vec<TextGrade>,
    /// API URL for this response.
    pub url: String,
}

/// A category describing the type of a text grade (e.g. `aktywność`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextGradeCategory {
    /// Unique category identifier.
    pub id: i64,
    /// Colour used to display the category.
    pub color: TextGradeRef,
    /// Category name (e.g. `aktywność`).
    pub name: String,
    /// Short category code (e.g. `akt`). Not set on every category.
    pub short: Option<String>,
    /// Display weight of the category, sent as a number or a string.
    #[serde(default, deserialize_with = "string_or_int_opt")]
    pub weight: Option<String>,
    /// Whether the category counts towards the average.
    #[serde(default)]
    pub count_to_the_average: bool,
}

/// Response containing a single text grade category.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseTextGradeCategory {
    /// The category data.
    pub category: TextGradeCategory,
    /// API URL for this response.
    pub url: String,
}

/// Response containing all text grade categories.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseTextGradeCategories {
    /// List of categories.
    pub categories: Vec<TextGradeCategory>,
    /// API URL for this response.
    pub url: String,
}

/// A skill that a descriptive text grade can be attached to (e.g. `Czytanie`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescriptiveGradeSkill {
    /// Unique skill identifier.
    pub id: i64,
    /// Colour used to display the skill.
    #[serde(rename = "Color")]
    pub color: TextGradeRef,
    /// Skill name (e.g. `Czytanie`).
    pub name: String,
    /// Reference to the subject this skill belongs to.
    pub subject: TextGradeRef,
    /// Display weight of the skill, sent as a number or a string.
    #[serde(default, deserialize_with = "string_or_int_opt")]
    pub weight: Option<String>,
    /// Whether the skill counts towards the average.
    #[serde(default)]
    pub count_to_the_average: bool,
}

/// Response containing a single descriptive skill.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseDescriptiveGradeSkill {
    /// The skill data.
    pub skill: DescriptiveGradeSkill,
    /// API URL for this response.
    pub url: String,
}

/// Response containing all descriptive skills.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseDescriptiveGradeSkills {
    /// List of skills.
    pub skills: Vec<DescriptiveGradeSkill>,
    /// API URL for this response.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_text_grade_accepts_category_and_skill_variants() {
        let json = r#"{
            "Grades": [
                {
                    "Id": 97964,
                    "Lesson": {"Id": 5338244, "Url": "https://api.librus.pl/2.0/Lessons/5338244"},
                    "Subject": {"Id": 23324, "Url": "https://api.librus.pl/2.0/Subjects/23324"},
                    "Student": {"Id": 2220643, "Url": "https://api.librus.pl/2.0/Users/2220643"},
                    "AddedBy": {"Id": 1490731, "Url": "https://api.librus.pl/2.0/Users/1490731"},
                    "Category": {"Id": 90126, "Url": "https://api.librus.pl/2.0/TextGrades/Categories/90126"},
                    "Grade": "Bardzo aktywna na lekcji.",
                    "Date": "2026-10-02",
                    "AddDate": "2026-10-05 10:19:05",
                    "Semester": 1,
                    "ShowInGradesView": true
                },
                {
                    "Id": 15932,
                    "Lesson": {"Id": 5624188, "Url": "https://api.librus.pl/2.0/Lessons/5624188"},
                    "Subject": {"Id": 71224, "Url": "https://api.librus.pl/2.0/Subjects/71224"},
                    "Student": {"Id": 2220643, "Url": "https://api.librus.pl/2.0/Users/2220643"},
                    "AddedBy": {"Id": 1052227, "Url": "https://api.librus.pl/2.0/Users/1052227"},
                    "Skill": {"Id": 783797, "Url": "https://api.librus.pl/2.0/DescriptiveTextGrades/Skills/783797"},
                    "Grade": "Czyta płynnie.",
                    "Date": "2026-09-10",
                    "AddDate": "2026-09-10 21:37:23",
                    "Semester": 1,
                    "ShowInGradesView": true
                }
            ],
            "Url": "https://api.librus.pl/2.0/BaseTextGrades"
        }"#;

        let response: ResponseTextGrades = serde_json::from_str(json).unwrap();
        assert_eq!(response.grades.len(), 2);
        assert_eq!(response.grades[0].category.as_ref().unwrap().id, 90126);
        assert!(response.grades[0].skill.is_none());
        assert_eq!(response.grades[1].skill.as_ref().unwrap().id, 783797);
        assert!(response.grades[1].category.is_none());
    }

    #[test]
    fn grade_category_weight_accepts_number_and_string() {
        let json = r#"{
            "Categories": [
                {
                    "Id": 90126,
                    "Color": {"Id": 16, "Url": "https://api.librus.pl/2.0/Colors/16"},
                    "Name": "aktywność",
                    "Short": "akt",
                    "Weight": 3,
                    "CountToTheAverage": true
                },
                {
                    "Id": 90127,
                    "Color": {"Id": 7, "Url": "https://api.librus.pl/2.0/Colors/7"},
                    "Name": "inna",
                    "CountToTheAverage": true
                }
            ],
            "Url": "https://api.librus.pl/2.0/TextGrades/Categories"
        }"#;
        let response: ResponseTextGradeCategories = serde_json::from_str(json).unwrap();
        assert_eq!(response.categories[0].weight.as_deref(), Some("3"));
        assert_eq!(response.categories[0].short.as_deref(), Some("akt"));
        assert!(response.categories[1].weight.is_none());
        assert!(response.categories[1].short.is_none());
    }

    #[test]
    fn descriptive_skill_weight_accepts_number_and_string() {
        let json = r#"{
            "Skills": [
                {
                    "Id": 783797,
                    "Color": {"Id": 1, "Url": "https://api.librus.pl/2.0/Colors/1"},
                    "Name": "Czytanie",
                    "Weight": "1",
                    "CountToTheAverage": false,
                    "Subject": {"Id": 71224, "Url": "https://api.librus.pl/2.0/Subjects/71224"}
                }
            ],
            "Url": "https://api.librus.pl/2.0/DescriptiveTextGrades/Skills"
        }"#;
        let response: ResponseDescriptiveGradeSkills = serde_json::from_str(json).unwrap();
        assert_eq!(response.skills[0].name, "Czytanie");
        assert_eq!(response.skills[0].weight.as_deref(), Some("1"));
    }
}
