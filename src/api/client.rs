use log::debug;
use std::collections::BTreeMap as Map;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use reqwest::{self, Client};
use serde::{self, Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::api::resource::{
    Filter, Item, Label, NewProject, Note, Project, ProjectNote, Reminder, ToJson,
};

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
    pub async fn get_resources(
        &self,
        resource_types: Vec<String>,
    ) -> Result<TodoistResponse, TodoistApiError> {
        let query = TodoistQuery::get(&self.token, resource_types);
        debug!("Sending query: {:?}", query);
        let response: TodoistResponse = self
            .client
            .get(&self.url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        debug!("API response: {:?}", response);
        Ok(response)
    }

    #[tokio::main]
    pub async fn sync_commands(
        &self,
        resource_types: Vec<String>,
        sync_token: Option<String>,
        commands: Vec<TodoistCommand>,
    ) -> Result<TodoistResponse, TodoistApiError> {
        let query = TodoistQuery::command(&self.token, sync_token, resource_types, commands);
        debug!("Sending query: {:?}", query);
        let response: TodoistResponse = self
            .client
            .post(&self.url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        debug!("API response: {:?}", response);

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
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_token: Option<String>,
    resource_types: String,
    commands: Option<String>,
}

impl TodoistQuery {
    fn get(token: &str, resource_types: Vec<String>) -> TodoistQuery {
        TodoistQuery {
            token: token.to_owned(),
            sync_token: Some("*".to_owned()),
            resource_types: serde_json::to_string(&resource_types).unwrap(),
            commands: None,
        }
    }

    fn command(
        token: &str,
        sync_token: Option<String>,
        resource_types: Vec<String>,
        commands: Vec<TodoistCommand>,
    ) -> TodoistQuery {
        TodoistQuery {
            token: token.to_owned(),
            sync_token: sync_token,
            resource_types: serde_json::to_string(&resource_types).unwrap(),
            commands: Some(serde_json::to_string(&commands).unwrap()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TodoistForm {
    token: String,
    commands: String,
}

impl TodoistForm {
    fn new(token: &str, commands: Vec<TodoistCommand>) -> TodoistForm {
        TodoistForm {
            token: token.to_owned(),
            commands: serde_json::to_string(&commands).unwrap(),
        }
    }
}

// Represents a sync API command
// uuid uniquely identifies the request to allow for safe retries in case of failure
// temp_id assigns an id to a new object that can be referenced by other objects created in the same request
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoistCommand {
    #[serde(rename = "type")]
    command_type: String,
    // args varies according to object type
    args: serde_json::Value,
    uuid: Uuid,
    temp_id: Uuid,
}

impl TodoistCommand {
    pub fn new(resource: &impl ToJson, temp_id: Option<Uuid>) -> Self {
        TodoistCommand {
            command_type: "project_add".to_string(),
            args: resource.to_json(),
            uuid: Uuid::new_v4(),
            temp_id: temp_id.unwrap_or(Uuid::new_v4()),
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
