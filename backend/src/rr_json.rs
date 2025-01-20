#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub candidates: Vec<Candidate>,
    pub usageMetadata: UsageMetadata,
    pub modelVersion: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
    pub finishReason: String,
    pub avgLogprobs: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsageMetadata {
    pub promptTokenCount: u32,
    pub candidatesTokenCount: u32,
    pub totalTokenCount: u32,
}