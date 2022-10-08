use std::sync::Arc;

use rcl_interfaces::srv::rmw::*;
use rosidl_runtime_rs::{Sequence, seq};

use crate::{rmw_request_id_t, Node, RclrsError, Service};

struct ParameterService {
    get_parameters_service: Arc<Service<GetParameters>>,
}

impl ParameterService {
    pub(crate) fn new(node: &mut Node) -> Result<Self, RclrsError> {
        let parameter_overrides = &node._parameter_map;
        let get_parameters_service = node.create_service(
            "get_parameters",
            |req_id: &rmw_request_id_t, req: GetParameters_Request| {
                GetParameters_Response {
                    values: seq![]
                }
            },
        )?;
        Ok(Self {
            get_parameters_service,
        })
    }
}
