//! Grade-related data types.

use serde::Deserialize;

/// A student's grade.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Grade {
    /// Unique grade identifier.
    pub id: i64,
    /// Reference to the lesson this grade is from.
    pub lesson: GradesRedirect,
    /// Reference to the subject.
    pub subject: GradesRedirect,
    /// Reference to the student who received this grade.
    pub student: GradesRedirect,
    /// Reference to the grade category (e.g., test, quiz).
    pub category: GradesRedirect,
    /// Reference to the teacher who added this grade.
    pub added_by: GradesRedirect,
    /// The grade value (e.g., "5", "4+", "A").
    pub grade: String,
    /// Date when the grade was given.
    pub date: String,
    /// Date when the grade was added to the system.
    pub add_date: String,
    /// Semester number (1 or 2).
    pub semester: i64,
    /// Whether this grade counts toward the average.
    pub is_constituent: bool,
    /// Whether this is a semester grade.
    pub is_semester: bool,
    /// Whether this is a proposed semester grade.
    pub is_semester_proposition: bool,
    /// Whether this is a final grade.
    pub is_final: bool,
    /// Whether this is a proposed final grade.
    pub is_final_proposition: bool,
    /// References to comments on this grade.
    pub comments: Option<Vec<GradesRedirect>>,
    /// Reference to an improvement grade.
    pub improvement: Option<GradesRedirect>,
    /// Reference to a resit grade.
    pub resit: Option<GradesRedirect>,
}

/// A reference to another resource with ID and URL.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesRedirect {
    /// Resource ID.
    pub id: i32,
    /// API URL to fetch the resource.
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesResources {
    #[serde(rename = "Grades\\Averages")]
    pub grades_averages: GradesUrl,
    #[serde(rename = "Grades\\StudentsAverages")]
    pub grades_students_averages: GradesUrl,
    #[serde(rename = "Grades\\CategoriesAverages")]
    pub grades_categories_averages: GradesUrl,
    #[serde(rename = "Grades\\Categories")]
    pub grades_categories: GradesUrl,
    #[serde(rename = "Grades\\Comments")]
    pub grades_comments: GradesUrl,
    #[serde(rename = "Grades\\Scales")]
    pub grades_scales: GradesUrl,
    #[serde(rename = "Grades\\Types")]
    pub grades_types: GradesUrl,
    #[serde(rename = "Grades\\UnpreparednessPerSemesterAndSubject")]
    pub grades_unpreparedness_per_semester_and_subject: GradesUrl,
    #[serde(rename = "..")]
    pub root: GradesUrl,
}

/// Response containing all grades.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGrades {
    /// List of grades.
    pub grades: Vec<Grade>,
    /// Related API resources.
    pub resources: GradesResources,
    /// API URL for this response.
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeColor {
    pub id: i64,
    pub url: String,
}

/// A grade category describing the type of assessment.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeCategory {
    /// Unique category identifier.
    pub id: i64,
    /// Color for display purposes.
    pub color: GradeColor,
    /// Category name (e.g., "Test", "Quiz", "Homework").
    pub name: String,
    /// Whether applicable to adult extramural students.
    pub adults_extramural: bool,
    /// Whether applicable to adult daily students.
    pub adults_daily: bool,
    /// Whether this is a standard category.
    pub standard: bool,
    /// Whether this category is read-only.
    pub is_read_only: String,
    /// Whether grades in this category count toward average.
    pub count_to_the_average: bool,
    /// Whether this category blocks other grades.
    pub block_any_grades: bool,
    /// Whether this assessment is mandatory.
    pub obligation_to_perform: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesCategoryResources {
    #[serde(rename = "..")]
    pub root: GradesUrl,
}

/// A comment attached to a grade.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeComment {
    /// Unique comment identifier.
    pub id: i32,
    /// Reference to the teacher who added the comment.
    pub added_by: GradeDetails,
    /// Reference to the grade this comment is attached to.
    pub grade: GradeDetails,
    /// The comment text.
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeDetails {
    pub id: i64,
    pub url: String,
}

/// Response containing a single grade category.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGradesCategories {
    /// The grade category.
    pub category: GradeCategory,
    /// Related API resources.
    pub resources: GradesCategoryResources,
}

/// Response containing a single grade comment.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGradesComments {
    /// The grade comment, if it exists.
    pub comment: Option<GradeComment>,
    /// Related API resources.
    pub resources: GradesCategoryResources,
    /// API URL for this response.
    pub url: String,
}
