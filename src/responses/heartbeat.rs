use responses::primitive::*;

//V0
#[derive(Debug,PartialEq)]
pub struct HeartbeatResponse {
  pub error_code: i16,
}

pub fn ser_heartbeat_response(response: HeartbeatResponse, output: &mut Vec<u8>) -> () {
  ser_i16(response.error_code, output);
}