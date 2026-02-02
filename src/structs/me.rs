//! Current user data types.

use serde::Deserialize;

/// User account information.
#[derive(Debug, Deserialize)]
pub struct Account {
    /// Account ID.
    #[serde(rename = "Id")]
    pub id: u32,
    /// Associated user ID.
    #[serde(rename = "UserId")]
    pub user_id: u32,
    /// First name.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// Last name.
    #[serde(rename = "LastName")]
    pub last_name: String,
    /// Email address.
    #[serde(rename = "Email")]
    pub email: String,
    /// User group ID.
    #[serde(rename = "GroupId")]
    pub group_id: u32,
    /// Whether the account is active.
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    /// Login username.
    #[serde(rename = "Login")]
    pub login: String,
    /// Whether this is a premium account.
    #[serde(rename = "IsPremium")]
    pub is_premium: bool,
    /// Whether this is a premium demo account.
    #[serde(rename = "IsPremiumDemo")]
    pub is_premium_demo: bool,
    /// Premium expiration date timestamp.
    #[serde(rename = "ExpiredPremiumDate")]
    pub expired_premium_date: Option<u64>,
    /// List of premium add-ons.
    #[serde(rename = "PremiumAddons")]
    pub premium_addons: Vec<String>,
}

/// Basic user profile.
#[derive(Debug, Deserialize)]
pub struct User {
    /// First name.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// Last name.
    #[serde(rename = "LastName")]
    pub last_name: String,
}

/// Class reference.
#[derive(Debug, Deserialize)]
pub struct Class {
    /// Class ID.
    #[serde(rename = "Id")]
    pub id: u32,
    /// API URL for the class.
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    #[serde(rename = "..")]
    pub url: Resource,
}

/// Current user information combining account, profile, and class.
#[derive(Debug, Deserialize)]
pub struct Me {
    /// Account details.
    #[serde(rename = "Account")]
    pub account: Account,
    /// Session refresh interval.
    #[serde(rename = "Refresh")]
    pub refresh: u32,
    /// User profile.
    #[serde(rename = "User")]
    pub user: User,
    /// Class the user belongs to.
    #[serde(rename = "Class")]
    pub class: Class,
}

/// Response containing current user information.
#[derive(Debug, Deserialize)]
pub struct ResponseMe {
    /// Current user data.
    #[serde(rename = "Me")]
    pub me: Me,
    /// Related API resources.
    #[serde(rename = "Resources")]
    pub resources: Resources,
    /// API URL for this response.
    #[serde(rename = "Url")]
    pub url: String,
}
