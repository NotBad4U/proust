use parser::primitive::*;

use nom::{HexDisplay,Needed,IResult,FileProducer, be_i16, be_i32, be_i64};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;

#[derive(PartialEq,Debug)]
pub struct HeartbeatRequest<'a> {
  group_id: KafkaString<'a>,
  generation_id: i32,
  member_id: KafkaString<'a>,
}

named!(pub heartbeat_request<HeartbeatRequest>,
  do_parse!(
    group_id: kafka_string >>
    generation_id: be_i32 >>
    member_id: kafka_string >>
    (
      HeartbeatRequest {
        group_id,
        generation_id,
        member_id,
      }
    )
  )
);

#[cfg(test)]
mod test {

  use super::*;
  use nom::*;
  use nom::IResult::*;

  #[test]
  fn it_should_parse_correct_heartbeat_request() {
    let input = &[
        0x00, 0x05,                   // group_id size
        0x68, 0x65, 0x6c, 0x6c, 0x6f, // group_id = hello
        0x00, 0x00, 0x00, 0x03,       // generation_id = 3
        0x00, 0x05,                   // member_id size
        0x77, 0x6f, 0x72, 0x6c, 0x64, // member_id = world
      ];

      let result = heartbeat_request(input);

      let expected = HeartbeatRequest {
        group_id: "hello",
        generation_id: 3,
        member_id: "world",
      };

      assert_eq!(result, Done(&[][..], expected))
  }
}