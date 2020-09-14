/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfCreatePartitionsRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfCreatePartitionsRequest {
    /// Each topic that we want to create new partitions inside.
    pub topics: Vec<CreatePartitionsTopic>,

    /// The time in ms to wait for the partitions to be created.
    pub timeout_ms: i32,

    /// If true, then validate the request, but don't actually increase the number of partitions.
    pub validate_only: bool,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct CreatePartitionsTopic {
    /// The topic name.
    pub name: String,

    /// The new partition count.
    pub count: i32,

    /// The new partition assignments.
    pub assignments: Option<Vec<CreatePartitionsAssignment>>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct CreatePartitionsAssignment {
    /// The assigned broker IDs.
    pub broker_ids: Vec<i32>,
}

// -----------------------------------
// KfCreatePartitionsResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfCreatePartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The partition creation results for each topic.
    pub results: Vec<CreatePartitionsTopicResult>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct CreatePartitionsTopicResult {
    /// The topic name.
    pub name: String,

    /// The result error, or zero if there was no error.
    pub error_code: ErrorCode,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

// -----------------------------------
// Implementation - KfCreatePartitionsRequest
// -----------------------------------

impl Request for KfCreatePartitionsRequest {
    const API_KEY: u16 = 37;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 1;
    const DEFAULT_API_VERSION: i16 = 1;

    type Response = KfCreatePartitionsResponse;
}
