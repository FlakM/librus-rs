//! User data types.

use serde::Deserialize;

/// A user in the Librus system (student, teacher, or parent).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    /// Unique user identifier.
    pub id: i64,
    /// Account ID as a string.
    pub account_id: String,
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Class the user belongs to (for students).
    pub class: Option<UserClass>,
    /// School unit the user belongs to.
    pub unit: Option<UserUnit>,
    /// Class register number (for students).
    pub class_register_number: Option<i64>,
    /// Whether this user is a school employee.
    pub is_employee: bool,
    /// User group ID.
    pub group_id: i64,
}

/// Reference to a user's class.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserClass {
    /// Class ID.
    pub id: i64,
    /// API URL for the class.
    pub url: String,
    /// Class UUID.
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// Reference to a school unit.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserUnit {
    /// Unit ID.
    pub id: i64,
    /// API URL for the unit.
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UserResources {
    #[serde(rename = "Users\\IndividualEducationPeriods")]
    pub users_individual_education_periods: UserUrl,
    #[serde(rename = "Users\\CrossedOutStudents")]
    pub users_crossed_out_students: UserUrl,
    #[serde(rename = "..")]
    pub root: UserUrl,
}

/// Response containing a single user.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseUser {
    /// The user data, if found.
    pub user: Option<User>,
    /// Related API resources.
    pub resources: UserResources,
    /// API URL for this response.
    pub url: String,
}
