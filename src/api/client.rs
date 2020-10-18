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
    CommandResource, Filter, Item, Label, Note, Project, ProjectNote, Reminder, Resource,
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
        resources: Vec<&str>,
    ) -> Result<TodoistResponse, TodoistApiError> {
        let mut builder = TodoistQueryBuilder::new(&self.token);
        for resource in resources.into_iter() {
            builder.get(&resource);
        }
        let query = builder.build();

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
    pub async fn sync_resources<T>(
        &self,
        resources: Vec<T>,
    ) -> Result<TodoistResponse, TodoistApiError>
    where
        T: CommandResource + Resource,
    {
        let mut builder = TodoistQueryBuilder::new(&self.token);
        for resource in resources.into_iter() {
            // TODO: uuid and temp_id should not be hardcoded to None
            builder.add(&resource, None, None);
        }
        let query = builder.build();

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
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<String>,
}

// Represents a sync API command
// uuid uniquely identifies the request to allow for safe retries in case of failure
// temp_id assigns an id to a new object that can be referenced by other objects created in the same request
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoistCommand {
    #[serde(rename = "type")]
    command_string: String,
    // args varies according to object type
    args: serde_json::Value,
    uuid: Uuid,
    temp_id: Uuid,
}

pub struct TodoistQueryBuilder {
    token: String,
    commands: Vec<String>,
    sync_token: Option<String>,
    resource_types: Vec<String>,
}

impl TodoistQueryBuilder {
    pub fn new(token: &str) -> TodoistQueryBuilder {
        TodoistQueryBuilder {
            token: String::from(token),
            commands: Vec::new(),
            resource_types: Vec::new(),
            sync_token: None,
        }
    }

    pub fn add<'a, T: CommandResource + Resource>(
        &'a mut self,
        resource: &T,
        uuid: Option<Uuid>,
        temp_id: Option<Uuid>,
    ) -> &'a mut TodoistQueryBuilder {
        let command = TodoistCommand {
            command_string: resource.command(),
            args: resource.to_json(),
            uuid: uuid.map_or(Uuid::new_v4(), |u| u.into()),
            temp_id: temp_id.map_or(Uuid::new_v4(), |u| u.into()),
        };
        if let Ok(c) = serde_json::to_string(&command) {
            self.commands.push(c);
        }
        self.resource_types.push(resource.resource());
        self
    }

    pub fn get<'a>(&'a mut self, resource_type: &str) -> &'a mut TodoistQueryBuilder {
        self.resource_types.push(resource_type.to_string());
        self
    }

    pub fn build(&self) -> TodoistQuery {
        TodoistQuery {
            token: self.token.clone(),
            sync_token: self.sync_token.clone(),
            resource_types: serde_json::to_string(&self.resource_types).unwrap(),
            commands: match self.commands.len() {
                0 => None,
                _ => Some(serde_json::to_string(&self.commands).unwrap()),
            },
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
