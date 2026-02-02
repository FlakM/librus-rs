//! # librus-rs
//!
//! Rust client for [Librus Synergia](https://synergia.librus.pl/) - the Polish school diary system.
//!
//! This crate provides an async API client for accessing student grades, attendance,
//! messages, and other data from Librus Synergia.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use librus_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), librus_rs::Error> {
//!     // Create client from environment variables
//!     let mut client = Client::from_env().await?;
//!
//!     // Fetch grades
//!     let grades = client.grades().await?;
//!     for grade in grades.grades {
//!         println!("{}: {}", grade.date, grade.grade);
//!     }
//!
//!     // Fetch unread message count
//!     let unread = client.unread_counts().await?;
//!     println!("Unread messages: {}", unread.inbox);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Client Construction
//!
//! There are three ways to create a [`Client`]:
//!
//! ## From Environment Variables
//!
//! Reads `LIBRUS_USERNAME` and `LIBRUS_PASSWORD` from the environment:
//!
//! ```rust,no_run
//! use librus_rs::Client;
//!
//! # async fn example() -> Result<(), librus_rs::Error> {
//! let client = Client::from_env().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## With Explicit Credentials
//!
//! ```rust,no_run
//! use librus_rs::Client;
//!
//! # async fn example() -> Result<(), librus_rs::Error> {
//! let client = Client::new("username", "password").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using the Builder Pattern
//!
//! ```rust,no_run
//! use librus_rs::Client;
//!
//! # async fn example() -> Result<(), librus_rs::Error> {
//! let client = Client::builder()
//!     .username("username")
//!     .password("password")
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # API Overview
//!
//! The client provides access to two APIs:
//!
//! ## Synergia API
//!
//! Academic data including grades, attendance, lessons, and users.
//!
//! | Method | Description |
//! |--------|-------------|
//! | [`Client::me()`] | Current user info |
//! | [`Client::grades()`] | All grades |
//! | [`Client::grade_category()`] | Grade category by ID |
//! | [`Client::grade_comment()`] | Grade comment by ID |
//! | [`Client::lesson()`] | Lesson info by ID |
//! | [`Client::subject()`] | Subject info by ID |
//! | [`Client::attendances()`] | All attendances |
//! | [`Client::attendance_types()`] | Attendance types |
//! | [`Client::homeworks()`] | All homeworks |
//! | [`Client::school_notices()`] | School notices (announcements) |
//! | [`Client::user()`] | User by ID |
//! | [`Client::current_user()`] | Current user details |
//!
//! ## Messages API
//!
//! Internal messaging system.
//!
//! | Method | Description |
//! |--------|-------------|
//! | [`Client::unread_counts()`] | Unread message counts |
//! | [`Client::inbox_messages()`] | Received messages |
//! | [`Client::outbox_messages()`] | Sent messages |
//! | [`Client::message()`] | Full message details |
//! | [`Client::attachment()`] | Download attachment |
//!
//! # Error Handling
//!
//! All API methods return `Result<T, Error>`. See [`Error`] for possible error variants.
//!
//! ```rust,no_run
//! use librus_rs::{Client, Error};
//!
//! # async fn example() {
//! let result = Client::from_env().await;
//! match result {
//!     Ok(client) => println!("Authenticated successfully"),
//!     Err(Error::MissingEnvVar(var)) => eprintln!("Missing: {}", var),
//!     Err(Error::Authentication) => eprintln!("Invalid credentials"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

mod error;
mod structs;

use reqwest::Client as HttpClient;

pub use crate::error::Error;
pub use crate::structs::announcements::{ResponseSchoolNotices, SchoolNotice};
pub use crate::structs::events::{Homework, ResponseHomeworks};
pub use crate::structs::grades::{
    Grade, GradeCategory, GradeComment, ResponseGrades, ResponseGradesCategories,
    ResponseGradesComments,
};
pub use crate::structs::lessons::{
    Attendance, AttendanceType, Lesson, LessonSubject, ResponseAttendances,
    ResponseAttendancesType, ResponseLesson, ResponseLessonSubject,
};
pub use crate::structs::me::{Me, ResponseMe};
pub use crate::structs::messages::{
    Attachment, InboxMessage, MessageDetail, OutboxMessage, UnreadCounts,
};
pub use crate::structs::users::{ResponseUser, User};

use crate::structs::messages::{
    ResponseInboxMessages, ResponseMessageDetail, ResponseOutboxMessages, ResponseUnreadCounts,
};

/// A specialized `Result` type for librus-rs operations.
pub type Result<T> = std::result::Result<T, Error>;

const SYNERGIA_API_BASE: &str = "https://synergia.librus.pl/gateway/api/2.0/";
const MESSAGES_API_BASE: &str = "https://wiadomosci.librus.pl/api/";
const AUTH_URL: &str = "https://api.librus.pl/OAuth/Authorization?client_id=46";
const AUTH_TEST_URL: &str =
    "https://api.librus.pl/OAuth/Authorization?client_id=46&response_type=code&scope=mydata";
const AUTH_GRANT_URL: &str = "https://api.librus.pl/OAuth/Authorization/Grant?client_id=46";
const TOKEN_INFO_URL: &str = "https://synergia.librus.pl/gateway/api/2.0/Auth/TokenInfo/";
const MESSAGES_INIT_URL: &str = "https://synergia.librus.pl/wiadomosci3";

/// Builder for creating a [`Client`] instance with custom configuration.
///
/// # Example
///
/// ```rust,no_run
/// use librus_rs::ClientBuilder;
///
/// # async fn example() -> Result<(), librus_rs::Error> {
/// let client = ClientBuilder::new()
///     .username("my_username")
///     .password("my_password")
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct ClientBuilder {
    username: Option<String>,
    password: Option<String>,
}

impl ClientBuilder {
    /// Creates a new builder instance with no credentials set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Librus username.
    ///
    /// # Example
    ///
    /// ```rust
    /// use librus_rs::ClientBuilder;
    ///
    /// let builder = ClientBuilder::new().username("my_username");
    /// ```
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Sets the Librus password.
    ///
    /// # Example
    ///
    /// ```rust
    /// use librus_rs::ClientBuilder;
    ///
    /// let builder = ClientBuilder::new()
    ///     .username("my_username")
    ///     .password("my_password");
    /// ```
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Builds and authenticates the client.
    ///
    /// This method consumes the builder and attempts to authenticate with Librus.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Username is missing ([`Error::MissingCredentials`])
    /// - Password is missing ([`Error::MissingCredentials`])
    /// - Authentication fails ([`Error::Authentication`])
    /// - Network error occurs ([`Error::Request`])
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::ClientBuilder;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = ClientBuilder::new()
    ///     .username("my_username")
    ///     .password("my_password")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build(self) -> Result<Client> {
        let username = self.username.ok_or(Error::MissingCredentials("username"))?;
        let password = self.password.ok_or(Error::MissingCredentials("password"))?;
        Client::authenticate(&username, &password).await
    }
}

/// An authenticated Librus API client.
///
/// This is the main entry point for interacting with Librus Synergia.
/// Create a client using one of the constructor methods, then call API methods
/// to fetch data.
///
/// # Example
///
/// ```rust,no_run
/// use librus_rs::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), librus_rs::Error> {
///     let mut client = Client::from_env().await?;
///
///     // Fetch user info
///     let me = client.me().await?;
///     println!("Logged in as: {} {}", me.me.user.first_name, me.me.user.last_name);
///
///     // Fetch grades
///     let grades = client.grades().await?;
///     println!("Total grades: {}", grades.grades.len());
///
///     Ok(())
/// }
/// ```
pub struct Client {
    http: HttpClient,
    messages_initialized: bool,
}

impl Client {
    /// Creates a new client from environment variables.
    ///
    /// Reads `LIBRUS_USERNAME` and `LIBRUS_PASSWORD` from the environment
    /// and authenticates with Librus.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `LIBRUS_USERNAME` is not set ([`Error::MissingEnvVar`])
    /// - `LIBRUS_PASSWORD` is not set ([`Error::MissingEnvVar`])
    /// - Authentication fails ([`Error::Authentication`])
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// // Ensure LIBRUS_USERNAME and LIBRUS_PASSWORD are set
    /// let client = Client::from_env().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn from_env() -> Result<Self> {
        let username = std::env::var("LIBRUS_USERNAME")
            .map_err(|_| Error::MissingEnvVar("LIBRUS_USERNAME"))?;
        let password = std::env::var("LIBRUS_PASSWORD")
            .map_err(|_| Error::MissingEnvVar("LIBRUS_PASSWORD"))?;
        Self::authenticate(&username, &password).await
    }

    /// Creates a new client with explicit credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails ([`Error::Authentication`])
    /// or a network error occurs ([`Error::Request`]).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::new("username", "password").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(username: &str, password: &str) -> Result<Self> {
        Self::authenticate(username, password).await
    }

    /// Creates a builder for configuring the client.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::builder()
    ///     .username("username")
    ///     .password("password")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    async fn authenticate(username: &str, password: &str) -> Result<Self> {
        let http = HttpClient::builder()
            .cookie_store(true)
            .build()
            .map_err(Error::HttpClient)?;

        let form_params = [("action", "login"), ("login", username), ("pass", password)];

        http.get(AUTH_TEST_URL)
            .send()
            .await
            .map_err(Error::Request)?;

        http.post(AUTH_URL)
            .form(&form_params)
            .send()
            .await
            .map_err(Error::Request)?;

        http.get(AUTH_GRANT_URL)
            .send()
            .await
            .map_err(Error::Request)?;

        let token_response = http
            .get(TOKEN_INFO_URL)
            .send()
            .await
            .map_err(Error::Request)?;

        if token_response.status() != 200 {
            return Err(Error::Authentication);
        }

        Ok(Self {
            http,
            messages_initialized: false,
        })
    }

    async fn get_api(&self, endpoint: &str) -> Result<String> {
        let url = format!("{}{}", SYNERGIA_API_BASE, endpoint);
        let response = self
            .http
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(Error::Request)?;

        let status = response.status();
        let text = response.text().await.map_err(Error::Request)?;

        if !status.is_success() {
            return Err(Error::ApiError {
                status: status.as_u16(),
                body: text,
            });
        }

        Ok(text)
    }

    async fn get_messages_api(&self, endpoint: &str) -> Result<String> {
        let url = format!("{}{}", MESSAGES_API_BASE, endpoint);
        let response = self.http.get(&url).send().await.map_err(Error::Request)?;

        let status = response.status();
        let text = response.text().await.map_err(Error::Request)?;

        if !status.is_success() {
            return Err(Error::ApiError {
                status: status.as_u16(),
                body: text,
            });
        }

        Ok(text)
    }

    async fn ensure_messages_initialized(&mut self) -> Result<()> {
        if self.messages_initialized {
            return Ok(());
        }
        self.http
            .get(MESSAGES_INIT_URL)
            .send()
            .await
            .map_err(Error::Request)?;
        self.messages_initialized = true;
        Ok(())
    }

    /// Gets current user information.
    ///
    /// Returns account details, user profile, and class information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let me = client.me().await?;
    /// println!("User: {} {}", me.me.user.first_name, me.me.user.last_name);
    /// println!("Email: {}", me.me.account.email);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn me(&self) -> Result<ResponseMe> {
        let json = self.get_api("Me").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets all grades for the student.
    ///
    /// Returns a list of all grades across all subjects.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let grades = client.grades().await?;
    /// for grade in grades.grades {
    ///     println!("{}: {} ({})", grade.date, grade.grade, grade.semester);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn grades(&self) -> Result<ResponseGrades> {
        let json = self.get_api("Grades").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets a grade category by ID.
    ///
    /// Categories describe the type of grade (e.g., test, homework, quiz).
    ///
    /// # Arguments
    ///
    /// * `id` - The category ID from a [`Grade`]'s `category` field
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the category is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let category = client.grade_category(123).await?;
    /// println!("Category: {}", category.category.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn grade_category(&self, id: i32) -> Result<ResponseGradesCategories> {
        let json = self.get_api(&format!("Grades/Categories/{}", id)).await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets a grade comment by ID.
    ///
    /// Comments provide additional context for a grade.
    ///
    /// # Arguments
    ///
    /// * `id` - The comment ID from a [`Grade`]'s `comments` field
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the comment is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let comment = client.grade_comment(456).await?;
    /// if let Some(c) = comment.comment {
    ///     println!("Comment: {}", c.text);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn grade_comment(&self, id: i32) -> Result<ResponseGradesComments> {
        let json = self.get_api(&format!("Grades/Comments/{}", id)).await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets a lesson by ID.
    ///
    /// Lessons contain information about which teacher teaches which subject to which class.
    ///
    /// # Arguments
    ///
    /// * `id` - The lesson ID
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the lesson is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let lesson = client.lesson(789).await?;
    /// println!("Lesson ID: {}", lesson.lesson.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn lesson(&self, id: i32) -> Result<ResponseLesson> {
        let json = self.get_api(&format!("Lessons/{}", id)).await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets a subject by ID.
    ///
    /// Subjects contain the name and short code for academic subjects.
    ///
    /// # Arguments
    ///
    /// * `id` - The subject ID
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the subject is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let subject = client.subject(101).await?;
    /// if let Some(s) = subject.subject {
    ///     println!("Subject: {} ({})", s.name, s.short);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject(&self, id: i32) -> Result<ResponseLessonSubject> {
        let json = self.get_api(&format!("Subjects/{}", id)).await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets all attendances for the student.
    ///
    /// Returns attendance records for all lessons.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let attendances = client.attendances().await?;
    /// println!("Total records: {}", attendances.attendances.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn attendances(&self) -> Result<ResponseAttendances> {
        let json = self.get_api("Attendances/").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets all attendance types.
    ///
    /// Types describe the kind of attendance (present, absent, late, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let types = client.attendance_types().await?;
    /// for t in types.types {
    ///     println!("{}: {} ({})", t.id, t.name, t.short);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn attendance_types(&self) -> Result<ResponseAttendancesType> {
        let json = self.get_api("Attendances/Types/").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets all homeworks.
    ///
    /// Returns a list of all homework assignments.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let homeworks = client.homeworks().await?;
    /// for hw in homeworks.homeworks {
    ///     println!("{}: {}", hw.date, hw.content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn homeworks(&self) -> Result<ResponseHomeworks> {
        let json = self.get_api("HomeWorks/").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets school notices (announcements).
    ///
    /// Returns a list of school notices.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let notices = client.school_notices().await?;
    /// for notice in notices.school_notices {
    ///     println!("{}: {}", notice.creation_date, notice.subject);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn school_notices(&self) -> Result<ResponseSchoolNotices> {
        let json = self.get_api("SchoolNotices").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets a user by ID.
    ///
    /// Users include teachers, students, and parents.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the user is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let client = Client::from_env().await?;
    /// let user = client.user(12345).await?;
    /// if let Some(u) = user.user {
    ///     println!("{} {}", u.first_name, u.last_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user(&self, id: i32) -> Result<ResponseUser> {
        let json = self.get_api(&format!("Users/{}", id)).await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets current user details.
    ///
    /// Returns detailed information about the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    pub async fn current_user(&self) -> Result<ResponseUser> {
        let json = self.get_api("Users").await?;
        serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })
    }

    /// Gets unread message counts for all folders.
    ///
    /// Returns counts for inbox, notes, alerts, and other message categories.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let mut client = Client::from_env().await?;
    /// let counts = client.unread_counts().await?;
    /// println!("Unread inbox: {}", counts.inbox);
    /// println!("Unread alerts: {}", counts.alerts);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unread_counts(&mut self) -> Result<UnreadCounts> {
        self.ensure_messages_initialized().await?;
        let json = self.get_messages_api("inbox/unreadMessagesCount").await?;
        let resp: ResponseUnreadCounts = serde_json::from_str(&json).map_err(|e| Error::Parse {
            source: e,
            body: json,
        })?;
        Ok(resp.data)
    }

    /// Gets inbox messages (received).
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (1-indexed)
    /// * `limit` - Number of messages per page
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let mut client = Client::from_env().await?;
    /// let messages = client.inbox_messages(1, 10).await?;
    /// for msg in messages {
    ///     println!("{}: {}", msg.sender_name, msg.topic);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn inbox_messages(&mut self, page: u32, limit: u32) -> Result<Vec<InboxMessage>> {
        self.ensure_messages_initialized().await?;
        let endpoint = format!("inbox/messages?page={}&limit={}", page, limit);
        let json = self.get_messages_api(&endpoint).await?;
        let resp: ResponseInboxMessages =
            serde_json::from_str(&json).map_err(|e| Error::Parse {
                source: e,
                body: json,
            })?;
        Ok(resp.data)
    }

    /// Gets outbox messages (sent).
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (1-indexed)
    /// * `limit` - Number of messages per page
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let mut client = Client::from_env().await?;
    /// let messages = client.outbox_messages(1, 10).await?;
    /// for msg in messages {
    ///     println!("To {}: {}", msg.receiver_name, msg.topic);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn outbox_messages(&mut self, page: u32, limit: u32) -> Result<Vec<OutboxMessage>> {
        self.ensure_messages_initialized().await?;
        let endpoint = format!("outbox/messages?page={}&limit={}", page, limit);
        let json = self.get_messages_api(&endpoint).await?;
        let resp: ResponseOutboxMessages =
            serde_json::from_str(&json).map_err(|e| Error::Parse {
                source: e,
                body: json,
            })?;
        Ok(resp.data)
    }

    /// Gets full message details by ID.
    ///
    /// Returns the complete message including body content and attachments.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID from an [`InboxMessage`] or [`OutboxMessage`]
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the message is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let mut client = Client::from_env().await?;
    /// let detail = client.message("12345").await?;
    /// if let Some(content) = Client::decode_message_content(&detail.message) {
    ///     println!("Content: {}", content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn message(&mut self, message_id: &str) -> Result<MessageDetail> {
        self.ensure_messages_initialized().await?;
        let endpoint = format!("inbox/messages/{}", message_id);
        let json = self.get_messages_api(&endpoint).await?;
        let resp: ResponseMessageDetail =
            serde_json::from_str(&json).map_err(|e| Error::Parse {
                source: e,
                body: json,
            })?;
        Ok(resp.data)
    }

    /// Downloads attachment bytes.
    ///
    /// # Arguments
    ///
    /// * `attachment_id` - The attachment ID from a [`MessageDetail`]'s attachments
    /// * `message_id` - The message ID containing the attachment
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the attachment is not found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use librus_rs::Client;
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), librus_rs::Error> {
    /// let mut client = Client::from_env().await?;
    /// let detail = client.message("12345").await?;
    /// for attachment in &detail.attachments {
    ///     let bytes = client.attachment(&attachment.id, &detail.message_id).await?;
    ///     fs::write(&attachment.name, &bytes).expect("Failed to save file");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn attachment(&mut self, attachment_id: &str, message_id: &str) -> Result<Vec<u8>> {
        self.ensure_messages_initialized().await?;
        let url = format!(
            "https://wiadomosci.librus.pl/api/attachments/{}/messages/{}",
            attachment_id, message_id
        );
        let response = self.http.get(&url).send().await.map_err(Error::Request)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                status: status.as_u16(),
                body,
            });
        }

        let bytes = response.bytes().await.map_err(Error::Request)?;
        Ok(bytes.to_vec())
    }

    /// Decodes base64-encoded message content to a string.
    ///
    /// Message bodies in Librus are base64-encoded. Use this helper to decode them.
    ///
    /// # Arguments
    ///
    /// * `content` - The base64-encoded content string
    ///
    /// # Returns
    ///
    /// `Some(String)` if decoding succeeds, `None` if the content is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use librus_rs::Client;
    ///
    /// let encoded = "SGVsbG8sIFdvcmxkIQ==";
    /// let decoded = Client::decode_message_content(encoded);
    /// assert_eq!(decoded, Some("Hello, World!".to_string()));
    /// ```
    pub fn decode_message_content(content: &str) -> Option<String> {
        use base64::{engine::general_purpose::STANDARD, Engine};
        STANDARD
            .decode(content)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
    }

    /// Formats API-provided HTML content into readable text.
    ///
    /// School notices (announcements) are often HTML-formatted. This helper removes tags
    /// and performs a minimal entity decode to make the content readable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use librus_rs::Client;
    ///
    /// let html = "<p>Hello&nbsp;<b>World</b> &amp; friends</p>";
    /// let text = Client::notice_content_to_text(html);
    /// assert_eq!(text, "Hello World & friends");
    /// ```
    pub fn notice_content_to_text(content: &str) -> String {
        let mut out = String::with_capacity(content.len());
        let mut in_tag = false;

        for ch in content.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => out.push(ch),
                _ => {}
            }
        }

        // Minimal entity decoding for common cases.
        let out = out
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");

        out.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[test]
    fn test_decode_message_content() {
        let encoded = base64::engine::general_purpose::STANDARD.encode("Hello, World!");
        let decoded = Client::decode_message_content(&encoded);
        assert_eq!(decoded, Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_decode_invalid_content() {
        let decoded = Client::decode_message_content("not valid base64!!!");
        assert!(decoded.is_none());
    }

    #[test]
    fn test_notice_content_to_text() {
        let html = "<p>Hello&nbsp;<b>World</b> &amp; friends</p>";
        let text = Client::notice_content_to_text(html);
        assert_eq!(text, "Hello World & friends");
    }
}
