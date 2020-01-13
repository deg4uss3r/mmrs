use std::error::Error;
use std::fmt;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json as json;

/// Creating a custom error for mapping Errors to return result from the library handles
/// The possible errors so far are BadJSONData and HTTPRequestError
/// BadJSONData maps to a serde_json::error::Error
/// HTTPRequestError maps to a reqwest::Error
#[derive(Debug)]
pub enum MMRSError {
    BadJSONData(serde_json::error::Error),
    HTTPRequestError(reqwest::Error),
}

impl fmt::Display for MMRSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MMRSError::BadJSONData(e) => write!(f, "Error writing to JSON string: {}", e),
            MMRSError::HTTPRequestError(e) => write!(f, "Error while sending HTTP POST: {}", e),
        }
    }
}

impl Error for MMRSError {}

/// Custom struct to serialize the HTTP POST data into a json objecting using serde_json
/// For a description of these fields see the [Official MatterMost Developer Documentation](https://developers.mattermost.com/integrate/incoming-webhooks/#parameters)
#[derive(Serialize, Deserialize)]
pub struct MMBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<String>,
}

impl MMBody {
    pub fn new() -> MMBody {
        MMBody {
            text: None,
            channel: None,
            username: None,
            icon_url: None,
            icon_emoji: None,
            attachments: None,
            r#type: None,
            props: None,
        }
    }
    /// This function allows us to convert from the struct to a string of JSON which a web server
    /// will accept
    pub fn to_json(self) -> Result<String, MMRSError> {
        json::to_string(&self).map_err(MMRSError::BadJSONData)
    }
}

/// Main function of the library which asynchronously sends the request and returns the status code
/// response. Will error out on a reqwest::Error if the send results in a failure 
#[tokio::main]
pub async fn send_message(uri: &str, body: String) -> Result<reqwest::StatusCode, MMRSError> {
    let status_code: reqwest::StatusCode = reqwest::Client::new()
        .post(uri)
        .body(body)
        .send()
        .await
        .map_err(MMRSError::HTTPRequestError)?
        .status();

    Ok(status_code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_body() {
        use crate as mmrs;

        let x: mmrs::MMBody = mmrs::MMBody::new();
        assert_eq!(x.text, None);
    }

    #[test]
    fn modify_body() {
        use crate as mmrs;

        let mut x: mmrs::MMBody = mmrs::MMBody::new();

        x.text = Some("Hello world!".to_string());
        assert_eq!(x.text, Some("Hello world!".to_string()));
    }

    #[test]
    fn json_check() {
        use crate as mmrs;

        let x: mmrs::MMBody = mmrs::MMBody {
            text: Some("Hello, world!".to_string()),
            channel: None,
            username: None,
            icon_url: None,
            icon_emoji: None,
            attachments: None,
            r#type: None,
            props: None,
        };

        let body = x.to_json().unwrap();

        assert_eq!(body, "{\"text\":\"Hello, world!\"}");
    }

    #[test]
    fn send_test() {
        use crate as mmrs;
        use mockito::{mock, Matcher};

        let _m = mock("POST", "/")
            .match_body(
                Matcher::JsonString("{\"text\":\"Hello, world!\"}".to_string())
            )
            .create();

        let x: mmrs::MMBody = mmrs::MMBody {
            text: Some("Hello, world!".to_string()),
            channel: None,
            username: None,
            icon_url: None,
            icon_emoji: None,
            attachments: None,
            r#type: None,
            props: None,
        };

        let body = x.to_json().unwrap();

        assert_eq!(
            mmrs::send_message(&mockito::server_url(), body.to_string()).unwrap(),
            reqwest::StatusCode::OK
        );
    }
}
