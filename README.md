# librus-rs

Rust client for [Librus Synergia](https://synergia.librus.pl/) - the Polish school diary system.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
librus-rs = "2.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use librus_rs::Client;

#[tokio::main]
async fn main() -> Result<(), librus_rs::Error> {
    // From environment variables (LIBRUS_USERNAME, LIBRUS_PASSWORD)
    let mut client = Client::from_env().await?;

    // Fetch grades
    let grades = client.grades().await?;
    for grade in grades.grades {
        println!("{}: {}", grade.date, grade.grade);
    }

    // Fetch messages
    let unread = client.unread_counts().await?;
    println!("Unread messages: {}", unread.inbox);

    let messages = client.inbox_messages(1, 10).await?;
    for msg in messages {
        println!("{}: {}", msg.sender_name, msg.topic);
    }

    // Fetch school notices (announcements)
    let notices = client.school_notices().await?;
    for notice in notices.school_notices {
        let content = Client::notice_content_to_text(&notice.content);
        let preview: String = content.chars().take(80).collect();
        println!("{}: {}", notice.subject, preview);
    }

    Ok(())
}
```

## Client Construction

Three ways to create a client:

```rust
use librus_rs::Client;

// From environment variables
let client = Client::from_env().await?;

// With explicit credentials
let client = Client::new("username", "password").await?;

// Using the builder pattern
let client = Client::builder()
    .username("username")
    .password("password")
    .build()
    .await?;
```

## API Reference

### Synergia API

Base URL: `https://synergia.librus.pl/gateway/api/2.0/`

| Method | Description |
|--------|-------------|
| `me()` | Get current user info |
| `grades()` | Get all grades |
| `grade_category(id)` | Get grade category by ID |
| `grade_comment(id)` | Get grade comment by ID |
| `lesson(id)` | Get lesson info by ID |
| `subject(id)` | Get subject info by ID |
| `attendances()` | Get all attendances |
| `attendance_types()` | Get attendance types |
| `homeworks()` | Get all homeworks |
| `school_notices()` | Get school notices (announcements) |
| `user(id)` | Get user by ID |
| `current_user()` | Get current user details |

### Messages API

Base URL: `https://wiadomosci.librus.pl/api/`

| Method | Description |
|--------|-------------|
| `unread_counts()` | Get unread message counts for all folders |
| `inbox_messages(page, limit)` | List received messages |
| `outbox_messages(page, limit)` | List sent messages |
| `message(id)` | Get full message details |
| `attachment(attachment_id, message_id)` | Download attachment as bytes |
| `decode_message_content(base64)` | Decode base64 message content to string |
| `notice_content_to_text(html)` | Convert API-provided notice HTML to text |

## Error Handling

All methods return `Result<T, librus_rs::Error>`. Error variants:

```rust
pub enum Error {
    Authentication,                     // Invalid credentials
    MissingEnvVar(&'static str),        // Environment variable not set
    MissingCredentials(&'static str),   // Builder credential missing
    HttpClient(reqwest::Error),         // HTTP client error
    Request(reqwest::Error),            // Request failed
    ApiError { status, body },          // API returned error
    Parse { source, body },             // JSON parsing failed
}
```

## Types

### Key Exported Types

```rust
pub use librus_rs::{
    Client,         // Main API client
    Error,          // Error type

    // Grades
    Grade, GradeCategory, GradeComment,
    ResponseGrades, ResponseGradesCategories, ResponseGradesComments,

    // Lessons & Attendance
    Lesson, LessonSubject, Attendance, AttendanceType,
    ResponseLesson, ResponseLessonSubject, ResponseAttendances, ResponseAttendancesType,

    // User
    Me, User, ResponseMe, ResponseUser,

    // Homework
    Homework, ResponseHomeworks,

    // School notices (announcements)
    SchoolNotice, ResponseSchoolNotices,

    // Messages
    InboxMessage, OutboxMessage, MessageDetail, Attachment, UnreadCounts,
};
```

### InboxMessage

```rust
pub struct InboxMessage {
    pub message_id: String,
    pub sender_first_name: String,
    pub sender_last_name: String,
    pub sender_name: String,
    pub topic: String,
    pub content: String,       // Base64 encoded
    pub send_date: String,
    pub read_date: Option<String>,
    pub is_any_file_attached: bool,
    pub tags: Vec<String>,
    pub category: Option<String>,
}
```

### MessageDetail

```rust
pub struct MessageDetail {
    pub message_id: String,
    pub sender_id: Option<String>,
    pub sender_first_name: String,
    pub sender_last_name: String,
    pub sender_name: String,
    pub sender_group: Option<String>,
    pub topic: String,
    pub message: String,       // Base64 encoded
    pub send_date: String,
    pub read_date: Option<String>,
    pub attachments: Vec<Attachment>,
    pub receivers_count: Option<u32>,
    pub no_reply: Option<u8>,
    pub archive: Option<u8>,
}
```

## Development

For Nix users:

```bash
nix develop
cargo build
cargo test
```

## License

MIT
