use std::fmt::Display;
use worker::{FormEntry, FormData};

/// Represents the kind of the notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationKind {
	Startup,
	Download,
	PostProcessingStarted,
	Complete,
	Failed,
	Warning,
	Error,
	DiskFull,
	QueueDone,
	NewLogin,
	Other,
}

impl Display for NotificationKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NotificationKind::Startup => write!(f, "Startup"),
			NotificationKind::Download => write!(f, "Download"),
			NotificationKind::PostProcessingStarted => write!(f, "PostProcessingStarted"),
			NotificationKind::Complete => write!(f, "Complete"),
			NotificationKind::Failed => write!(f, "Failed"),
			NotificationKind::Warning => write!(f, "Warning"),
			NotificationKind::Error => write!(f, "Error"),
			NotificationKind::DiskFull => write!(f, "DiskFull"),
			NotificationKind::QueueDone => write!(f, "QueueDone"),
			NotificationKind::NewLogin => write!(f, "NewLogin"),
			NotificationKind::Other => write!(f, "Other"),
		}
	}
}

/// Convert a String to a NotificationKind
pub fn parse_notification_kind(kind: String) -> NotificationKind {
	match kind.as_str() {
		"startup" => NotificationKind::Startup,
		"download" => NotificationKind::Download,
		"pp" => NotificationKind::PostProcessingStarted,
		"complete" => NotificationKind::Complete,
		"failed" => NotificationKind::Failed,
		"warning" => NotificationKind::Warning,
		"error" => NotificationKind::Error,
		"disk_full" => NotificationKind::DiskFull,
		"queue_done" => NotificationKind::QueueDone,
		"new_login" => NotificationKind::NewLogin,
		_ => NotificationKind::Other,
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
	pub kind: NotificationKind,
	pub title: String,
	pub text: String,
	pub params: Option<String>,
}

impl Notification {
	pub fn create_embed(&self) -> String {
		serde_json::json!({
            "embeds": [{
                "title": self.title,
                "description": self.text
            }]
        }).to_string()
	}
}

impl From<FormData> for Notification {
	fn from(form: FormData) -> Self {
		let kind = match form.get("kind").unwrap() {
			FormEntry::Field(kind) => parse_notification_kind(kind),
			_ => NotificationKind::Other,
		};

		let title = match form.get("title").unwrap() {
			FormEntry::Field(value) => value,
			_ => String::from("Unknown"),
		};

		let text = match form.get("text").unwrap() {
			FormEntry::Field(value) => value,
			_ => String::from("No text"),
		};

		let params = match form.get("params") {
			Some(FormEntry::Field(value)) => Some(value),
			_ => None,
		};

		Notification {
			kind,
			title,
			text,
			params,
		}
	}
}
