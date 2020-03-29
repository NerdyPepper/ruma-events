//! Types for the *m.direct* event.

use std::collections::HashMap;

use ruma_events_macros::ruma_event;
use ruma_identifiers::{RoomId, UserId};

ruma_event! {
    /// Informs the client about the rooms that are considered direct by a user.
    DirectEvent {
        kind: Event,
        event_type: "m.direct",
        content_type_alias: {
            /// The payload for `DirectEvent`.
            ///
            /// A mapping of `UserId`s to a list of `RoomId`s which are considered *direct* for that
            /// particular user.
            HashMap<UserId, Vec<RoomId>>
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ruma_identifiers::{RoomId, UserId};
    use serde_json::to_string;

    use super::{DirectEvent, DirectEventContent};
    use crate::EventResult;

    #[test]
    fn serialization() {
        let mut content: DirectEventContent = HashMap::new();
        let alice = UserId::new("ruma.io").unwrap();
        let room = vec![RoomId::new("ruma.io").unwrap()];

        content.insert(alice.clone(), room.clone());

        let event = DirectEvent { content };

        assert_eq!(
            to_string(&event).unwrap(),
            format!(
                r#"{{"type":"m.direct","content":{{"{}":["{}"]}}}}"#,
                alice.to_string(),
                room[0].to_string()
            )
        );
    }

    #[test]
    fn deserialization() {
        let alice = UserId::new("ruma.io").unwrap();
        let rooms = vec![
            RoomId::new("ruma.io").unwrap(),
            RoomId::new("ruma.io").unwrap(),
        ];

        let json_data = format!(
            r#"{{
            "type": "m.direct",
            "content": {{ "{}": ["{}", "{}"] }}
        }}"#,
            alice.to_string(),
            rooms[0].to_string(),
            rooms[1].to_string()
        );

        let event: DirectEvent = serde_json::from_str::<EventResult<_>>(&json_data)
            .unwrap()
            .into_result()
            .unwrap();
        let direct_rooms = event.content.get(&alice).unwrap();

        assert!(direct_rooms.contains(&rooms[0]));
        assert!(direct_rooms.contains(&rooms[1]));
    }
}
