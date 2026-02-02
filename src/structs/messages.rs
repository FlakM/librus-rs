//! Message-related data types.

use serde::Deserialize;

/// Unread message counts across all folders.
#[derive(Debug, Deserialize)]
pub struct UnreadCounts {
    /// Unread messages in inbox.
    pub inbox: u32,
    /// Unread notes.
    pub notes: u32,
    /// Unread alerts.
    pub alerts: u32,
    /// Unread substitution notifications.
    pub substitutions: u32,
    /// Unread absence notifications.
    pub absences: u32,
    /// Unread justification requests.
    pub justifications: u32,
    /// Items in trash.
    pub trash: u32,
    #[serde(rename = "archiveInbox")]
    /// Archived inbox messages.
    pub archive_inbox: u32,
    #[serde(rename = "archiveNotes")]
    /// Archived notes.
    pub archive_notes: u32,
    #[serde(rename = "archiveAlerts")]
    /// Archived alerts.
    pub archive_alerts: u32,
    #[serde(rename = "archiveSubstitutions")]
    /// Archived substitution notifications.
    pub archive_substitutions: u32,
    #[serde(rename = "archiveAbsences")]
    /// Archived absence notifications.
    pub archive_absences: u32,
    #[serde(rename = "archiveJustifications")]
    /// Archived justification requests.
    pub archive_justifications: u32,
    #[serde(rename = "archiveTrash")]
    /// Archived trash items.
    pub archive_trash: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseUnreadCounts {
    pub data: UnreadCounts,
}

/// A message in the inbox (received message).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxMessage {
    /// Unique message identifier.
    pub message_id: String,
    /// Sender's first name.
    pub sender_first_name: String,
    /// Sender's last name.
    pub sender_last_name: String,
    /// Sender's full display name.
    pub sender_name: String,
    /// Message subject/topic.
    pub topic: String,
    /// Message content (base64-encoded).
    /// Use [`Client::decode_message_content`](crate::Client::decode_message_content) to decode.
    pub content: String,
    /// Date when the message was sent.
    pub send_date: String,
    /// Date when the message was read, if read.
    pub read_date: Option<String>,
    /// Whether the message has attachments.
    pub is_any_file_attached: bool,
    /// Message tags/labels.
    pub tags: Vec<String>,
    /// Message category.
    pub category: Option<String>,
}

/// A message in the outbox (sent message).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboxMessage {
    /// Unique message identifier.
    pub message_id: String,
    /// Receiver's first name.
    pub receiver_first_name: String,
    /// Receiver's last name.
    pub receiver_last_name: String,
    /// Receiver's full display name.
    pub receiver_name: String,
    /// Message subject/topic.
    pub topic: String,
    /// Message content (base64-encoded).
    pub content: String,
    /// Date when the message was sent.
    pub send_date: String,
    /// Whether the message has attachments.
    pub is_any_file_attached: bool,
    /// Message tags/labels.
    pub tags: Vec<String>,
    /// Message category.
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseInboxMessages {
    pub data: Vec<InboxMessage>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseOutboxMessages {
    pub data: Vec<OutboxMessage>,
}

/// A file attachment in a message.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Unique attachment identifier.
    pub id: String,
    /// Original filename.
    pub name: String,
    /// File size in bytes.
    pub size: Option<u64>,
}

/// Full message details including content and attachments.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDetail {
    /// Unique message identifier.
    pub message_id: String,
    /// Sender's user ID.
    pub sender_id: Option<String>,
    /// Sender's first name.
    pub sender_first_name: String,
    /// Sender's last name.
    pub sender_last_name: String,
    /// Sender's full display name.
    pub sender_name: String,
    /// Sender's group (e.g., teacher, parent).
    pub sender_group: Option<String>,
    /// Message subject/topic.
    pub topic: String,
    /// Full message content (base64-encoded).
    /// Use [`Client::decode_message_content`](crate::Client::decode_message_content) to decode.
    #[serde(rename = "Message")]
    pub message: String,
    /// Date when the message was sent.
    pub send_date: String,
    /// Date when the message was read, if read.
    pub read_date: Option<String>,
    /// List of file attachments.
    pub attachments: Vec<Attachment>,
    /// Number of receivers (for group messages).
    pub receivers_count: Option<u32>,
    /// Whether replies are disabled (1 = no reply allowed).
    pub no_reply: Option<u8>,
    /// Whether the message is archived (1 = archived).
    pub archive: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseMessageDetail {
    pub data: MessageDetail,
}
