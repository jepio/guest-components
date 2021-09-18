// Copyright (c) 2021 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionRequest {
    pub command: String,
}

impl VersionRequest {
    pub fn new() -> VersionRequest {
        VersionRequest {
            command: String::from("version"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionResponse {
    pub status: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecryptionRequest {
    pub command: String,
    pub blobs: Vec<Blob>,
}

impl DecryptionRequest {
    pub fn new(blob: Blob) -> DecryptionRequest {
        DecryptionRequest {
            command: String::from("Decrypt"),
            blobs: vec![blob],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blob {
    pub kid: String,
    pub encrypted_data: String,
    pub algorithm: String,
    pub key_length: u32,
    pub iv: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecryptionResponse {
    pub status: String,
    pub data: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKekRequest {
    pub command: String,
    pub kids: Vec<String>,
}

impl GetKekRequest {
    pub fn new(key_id_list: Vec<String>) -> GetKekRequest {
        GetKekRequest {
            command: String::from("Get KEK"),
            kids: key_id_list,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKekResponse {
    pub status: String,
    pub data: Option<HashMap<String, String>>,
    pub error: Option<String>,
}
