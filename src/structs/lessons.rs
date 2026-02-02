//! Lesson and attendance data types.

use serde::Deserialize;

/// A lesson linking a teacher, subject, and class.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    /// Unique lesson identifier.
    pub id: i32,
    /// Reference to the teacher.
    pub teacher: LessonClass,
    /// Reference to the subject.
    pub subject: LessonClass,
    /// Reference to the class.
    pub class: LessonClass,
}

/// A reference to a lesson-related resource.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonClass {
    /// Resource ID.
    pub id: i32,
    /// API URL to fetch the resource.
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LessonResources {
    #[serde(rename = "..")]
    pub root: LessonUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonUrl {
    pub url: String,
}

/// Response containing a single lesson.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseLesson {
    /// The lesson data.
    pub lesson: Lesson,
    /// Related API resources.
    pub resources: LessonResources,
    /// API URL for this response.
    pub url: String,
}

/// An academic subject.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonSubject {
    /// Unique subject identifier.
    pub id: i32,
    /// Full subject name (e.g., "Mathematics").
    pub name: String,
    /// Subject number in the curriculum.
    #[serde(rename = "No")]
    pub num: i32,
    /// Short subject code (e.g., "MAT").
    pub short: String,
    /// Whether this is an extracurricular subject.
    pub is_extra_curricular: Option<bool>,
    /// Whether this is a block lesson.
    pub is_block_lesson: Option<bool>,
}

/// Response containing a single subject.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseLessonSubject {
    /// The subject data, if found.
    pub subject: Option<LessonSubject>,
    /// Related API resources.
    pub resources: LessonResources,
    /// API URL for this response.
    pub url: String,
}

/// A student's attendance record for a lesson.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attendance {
    /// Unique attendance record identifier.
    pub id: AttendanceId,
    /// Reference to the lesson.
    pub lesson: AttendanceAddedBy,
    /// Reference to the student.
    pub student: AttendanceAddedBy,
    /// Date of the lesson.
    pub date: String,
    /// Date when the record was added.
    pub add_date: String,
    /// Lesson number in the day (1-8+).
    pub lesson_no: i32,
    /// Semester number (1 or 2).
    pub semester: i32,
    /// Reference to the attendance type.
    #[serde(rename = "Type")]
    pub attendance_type: AttendanceAddedBy,
    /// Reference to the teacher who recorded attendance.
    pub added_by: AttendanceAddedBy,
    /// Reference to a school trip, if applicable.
    pub trip: Option<AttendanceAddedBy>,
}

/// A reference to an attendance-related resource.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceAddedBy {
    /// Resource ID.
    pub id: i32,
    /// API URL to fetch the resource.
    pub url: String,
}

/// Attendance record ID which can be numeric or string.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AttendanceId {
    /// Numeric ID.
    Integer(i32),
    /// String ID.
    String(String),
}

#[derive(Debug, Deserialize)]
pub struct AttendanceResources {
    #[serde(rename = "Attendances\\Types")]
    pub attendances_types: LessonUrl,
    #[serde(rename = "Attendances\\LessonsStatistics")]
    pub attendances_lessons_statistics: LessonUrl,
    #[serde(rename = "Attendances\\FilledByTeacher")]
    pub attendances_filled_by_teacher: LessonUrl,
    #[serde(rename = "..")]
    pub empty: LessonUrl,
}

/// Response containing all attendances.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseAttendances {
    /// List of attendance records.
    pub attendances: Vec<Attendance>,
    /// Related API resources.
    pub resources: AttendanceResources,
    /// API URL for this response.
    pub url: String,
}

/// A type of attendance (present, absent, late, etc.).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceType {
    /// Unique type identifier.
    pub id: i32,
    /// Full name (e.g., "Present", "Absent", "Late").
    pub name: String,
    /// Short code (e.g., "P", "A", "L").
    pub short: String,
    /// Whether this is a standard type.
    pub standard: bool,
    /// RGB color for display (e.g., "00FF00").
    #[serde(rename = "ColorRGB")]
    pub color_rgb: Option<String>,
    /// Whether this type counts as present.
    pub is_presence_kind: bool,
    /// Display order.
    pub order: i32,
    /// System identifier.
    pub identifier: String,
    /// Reference to a standard type.
    pub standard_type: Option<AttendanceColor>,
    /// Reference to the color.
    pub color: Option<AttendanceColor>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceColor {
    pub id: i32,
    pub url: String,
}

/// Response containing all attendance types.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseAttendancesType {
    /// List of attendance types.
    pub types: Vec<AttendanceType>,
    /// Related API resources.
    pub resources: LessonResources,
    /// API URL for this response.
    pub url: String,
}
