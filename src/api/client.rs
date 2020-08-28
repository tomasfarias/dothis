use std::time::Duration;
use std::error::Error;
use std::fmt;

use serde::{ self, Serialize };
use serde_json;
use reqwest::{ self, Client };

use super::resource::{ Project, Resource };

pub struct TodoistClient {
    token: String,
    client: Client,
    url: String,
}

impl TodoistClient {
    pub fn new(token: &str) -> TodoistClient {
        let timeout = Duration::new(10, 0);
        TodoistClient{
            token: token.to_owned(),
            client: Client::new(),
            url: "https://api.todoist.com/sync/v8/sync".to_owned(),
        }
    }

    #[tokio::main]
    pub async fn get_resource(&self, resource_types: Vec<String>) -> Result<Project, RequestError> {
        let query = TodoistQuery::new(&self.token, resource_types);
        let resource = self.client.get(&self.url)
            .query(&query)
            .send()
            .await?
            .json::<Project>()
            .await?;

        Ok(resource)
    }
}

#[derive(Debug, Serialize)]
pub struct TodoistQuery {
    token: String,
    sync_token: String,
    resource_types: String,
}

impl TodoistQuery {
    fn new(token: &str, resource_types: Vec<String>) -> TodoistQuery {
        TodoistQuery{
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
        TodoistForm{
            token: token.to_owned(),
            commands: serde_json::to_string(&commands).unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum RequestError {
    DeserializeError(serde_json::error::Error),
    RequestError(reqwest::Error),
}

impl From<serde_json::error::Error> for RequestError {
    fn from(err: serde_json::error::Error) -> RequestError {
        RequestError::DeserializeError(err)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> RequestError {
        RequestError::RequestError(err)
    }
}


impl Error for RequestError {
    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            RequestError::DeserializeError(ref err) => err as &Error,
            RequestError::RequestError(ref err) => err as &Error,
        })
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestError::DeserializeError(ref err) => fmt::Display::fmt(err, f),
            RequestError::RequestError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
