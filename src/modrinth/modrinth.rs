use std::ops::Add;
use std::os::unix::raw::off_t;
use std::time::Duration;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;

const DEFAULT_RATELIMIT: i32 = 300;
const BASE_URL: &str = "https://api.modrinth.com/v2";

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub struct Modrinth {
    user_agent: String,
    ratelimit_limit: i32, // maximum number of requests that can be made in a minute
    ratelimit_remaining: i32, // how many requests are remaining
    ratelimit_reset: i32, // time when ratelimit is reset
    client: reqwest::Client
}

pub_struct!(Project {
    slug: String,
    title: String,
    description: String,
    categories: Vec<String>,
    client_side: String,
    server_side: String,
    body: String,
    status: String,
    requested_status: Option<String>,
    issues_url: Option<String>,
    source_url: Option<String>,
    wiki_url: Option<String>,
    discord_url: Option<String>,
    project_type: String,
    downloads: i32,
    icon_url: Option<String>,
    color: Option<i32>,
    thread_id: Option<String>,
    id: String,
    team: String,
    published: String,
    updated: String,
    approved: Option<String>,
    queued: Option<String>,
    followers: i32,
    versions: Option<Vec<String>>,
    game_versions: Option<Vec<String>>,
    loaders: Option<Vec<String>>,
});

enum RequestType {
    POST,
    GET,
}

impl Modrinth {
    pub fn new(user_agent: String) -> Modrinth {
        Modrinth {
            user_agent: user_agent.clone(),
            ratelimit_limit: DEFAULT_RATELIMIT,
            ratelimit_remaining: DEFAULT_RATELIMIT,
            ratelimit_reset: 60,
            client: reqwest::ClientBuilder::new()
                .cookie_store(true)
                .user_agent(user_agent)
                .build().unwrap(),
        }
    }

    async fn do_request(&mut self, url: String, request_type: RequestType, headers: Option<HeaderMap>, params: Option<&[i32]>) -> Response {
        if self.ratelimit_remaining == 0 {
            async_std::task::sleep(Duration::from_secs(self.ratelimit_reset as u64)).await
        }
        let full_url = String::from(BASE_URL).add(url.as_str());
        let mut request_builder = match request_type {
            RequestType::GET => {
                self.client.get(full_url)
            },
            RequestType::POST => {
                self.client.post(full_url)
            }
        };

        if headers.is_some() {
            request_builder = request_builder.headers(headers.unwrap());
        }

        if params.is_some() {
            request_builder = request_builder.form(params.unwrap());
        }

        let response =  request_builder.send().await.unwrap();

        for (header_name, header_value) in response.headers() {
            match header_name.as_str() {
                "x-ratelimit-limit" => {
                    self.ratelimit_limit = str::parse::<i32>(header_value.to_str().unwrap()).unwrap();
                    println!("Updated ratelimit_limit to {}", self.ratelimit_limit)
                }

                "x-ratelimit-remaining" => {
                    self.ratelimit_remaining = str::parse::<i32>(header_value.to_str().unwrap()).unwrap();
                    println!("Updated ratelimit_remaining to {}", self.ratelimit_remaining)
                }

                "x-ratelimit-reset" => {
                    self.ratelimit_reset = str::parse::<i32>(header_value.to_str().unwrap()).unwrap();
                    println!("Updated ratelimit_reset to {}", self.ratelimit_reset)
                }

                _=> {
                    // println!("Matched nothing. {}", header_name)
                }
            }

        }

        return response;
    }

    pub async fn get_projects(&mut self, ids: Vec<String>) -> Vec<Project>{
        let result = self.do_request(
            String::from(format!("/projects?ids={:?}", ids)),
            RequestType::GET,
            None,
            None
        ).await;

        let value = json!(result.text().await.unwrap());

        let value_as_str = value.as_str().unwrap();
        return serde_json::from_str::<Vec<Project>>(value_as_str).unwrap();
    }
}
