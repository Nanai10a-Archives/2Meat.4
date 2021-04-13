use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CommandArgs {
    New {
        target: Target,
        place: Place,
    },
    #[deprecated]
    Mut {
        id: Uuid,
        mutation: Vec<Mutation>,
    },
    Drop {
        id: Uuid,
    },
    Subsc {
        receiver_id: Uuid,
        sender_id: Uuid,
    },
    Exit {
        receiver_id: Uuid,
        sender_id: Uuid,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Place {
    There,
    // TODO: FEATURE - Meta
}

#[deprecated]
#[derive(Debug, Copy, Clone)]
pub enum Mutation {
    // TODO: mutの列挙
}

#[derive(Debug, Copy, Clone)]
pub enum Target {
    Receiver,
    Sender,
}
