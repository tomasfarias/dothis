use std::collections::BTreeMap as Map;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use reqwest::{self, Client};
use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::api::resource::{Filter, Item, Label, Note, Project, ProjectNote, Reminder};

pub struct TodoistClient {
	token: String,
	client: Client,
	url: String,
}

impl TodoistClient {
	pub fn new(token: &str) -> TodoistClient {
		let timeout = Duration::new(10, 0);
		TodoistClient {
			token: token.to_owned(),
			client: Client::new(),
			url: "https://api.todoist.com/sync/v8/sync".to_owned(),
		}
	}

	#[tokio::main]
	pub async fn get_resource(
		&self,
		resource_types: Vec<String>,
	) -> Result<TodoistResponse, TodoistApiError> {
		let query = TodoistQuery::new(&self.token, resource_types);
		let response: TodoistResponse = self
			.client
			.get(&self.url)
			.query(&query)
			.send()
			.await?
			.json()
			.await?;

		Ok(response)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoistResponse {
	#[serde(default)]
	pub projects: Option<Vec<Project>>,
	#[serde(default)]
	pub items: Option<Vec<Item>>,
	#[serde(default)]
	pub notes: Option<Vec<Note>>,
	#[serde(default)]
	pub labels: Option<Vec<Label>>,
	#[serde(default)]
	pub filters: Option<Vec<Filter>>,
	#[serde(default)]
	pub project_notes: Option<Vec<ProjectNote>>,
	#[serde(default)]
	pub reminders: Option<Vec<Reminder>>,
	pub full_sync: bool,
	pub temp_id_mapping: Map<String, u32>,
	pub sync_token: String,
}

#[derive(Debug, Serialize)]
pub struct TodoistQuery {
	token: String,
	sync_token: String,
	resource_types: String,
}

impl TodoistQuery {
	fn new(token: &str, resource_types: Vec<String>) -> TodoistQuery {
		TodoistQuery {
			token: token.to_owned(),
			sync_token: "*".to_owned(),
			resource_types: serde_json::to_string(&resource_types).unwrap(),
		}
	}
}

#[derive(Debug, Serialize)]
pub struct TodoistForm {
	token: String,
	commands: String,
}

impl TodoistForm {
	fn new(token: &str, commands: Vec<String>) -> TodoistForm {
		TodoistForm {
			token: token.to_owned(),
			commands: serde_json::to_string(&commands).unwrap(),
		}
	}
}

#[derive(Debug)]
pub enum TodoistApiError {
	DeserializeError(serde_json::error::Error),
	RequestError(reqwest::Error),
}

impl From<serde_json::error::Error> for TodoistApiError {
	fn from(err: serde_json::error::Error) -> TodoistApiError {
		TodoistApiError::DeserializeError(err)
	}
}

impl From<reqwest::Error> for TodoistApiError {
	fn from(err: reqwest::Error) -> TodoistApiError {
		TodoistApiError::RequestError(err)
	}
}

impl Error for TodoistApiError {
	fn cause(&self) -> Option<&Error> {
		Some(match *self {
			TodoistApiError::DeserializeError(ref err) => err as &Error,
			TodoistApiError::RequestError(ref err) => err as &Error,
		})
	}
}

impl fmt::Display for TodoistApiError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			TodoistApiError::DeserializeError(ref err) => fmt::Display::fmt(err, f),
			TodoistApiError::RequestError(ref err) => fmt::Display::fmt(err, f),
		}
	}
}
