use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CommandArgs {
    New,
    Drop {
        id: Uuid,
    },
    Subsc {
        broadcaster_id: Uuid,
        subscriber_id: Uuid,
    },
    Exit {
        broadcaster_id: Uuid,
        subscriber_id: Uuid,
    },
}
