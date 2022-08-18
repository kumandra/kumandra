//! Helper for incoming object mappings requests.
//!
//! Handle (i.e. answer) incoming object mappings requests from a remote peer received via
//! `RequestResponsesBehaviour` with generic [`GenericRequestHandler`].

use crate::request_handlers::generic_request_handler::{GenericRequest, GenericRequestHandler};
use parity_scale_codec::{Decode, Encode};
use kumandra_core_primitives::objects::GlobalObject;
use kumandra_core_primitives::Sha256Hash;

/// Object-mapping protocol request.
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct ObjectMappingsRequest {
    /// Object hash (32-bytes)
    pub object_hash: Sha256Hash,
}

impl GenericRequest for ObjectMappingsRequest {
    const PROTOCOL_NAME: &'static str = "/kumandra/object-mappings/0.1.0";
    const LOG_TARGET: &'static str = "object-mappings-request-response-handler";
    type Response = ObjectMappingsResponse;
}

/// Object-mapping protocol request.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct ObjectMappingsResponse {
    /// Returned data.
    pub object_mapping: Option<GlobalObject>,
}

/// Create a new object-mappings request handler.
pub type ObjectMappingsRequestHandler = GenericRequestHandler<ObjectMappingsRequest>;
