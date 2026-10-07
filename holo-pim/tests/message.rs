use holo_pim::message::{MessageType, UnknownMessageType};

#[test]
fn message_types_round_trip() {
    for value in 0..=8 {
        let message_type = MessageType::try_from(value).unwrap();
        assert_eq!(message_type.as_u8(), value);
    }
}

#[test]
fn unknown_message_type_is_rejected() {
    assert_eq!(
        MessageType::try_from(9),
        Err(UnknownMessageType(9)),
    );
    assert_eq!(
        MessageType::try_from(u8::MAX),
        Err(UnknownMessageType(u8::MAX)),
    );
}

#[test]
fn message_type_display_is_stable() {
    assert_eq!(MessageType::JoinPrune.to_string(), "Join/Prune");
    assert_eq!(
        UnknownMessageType(9).to_string(),
        "unknown PIM message type: 9"
    );
}