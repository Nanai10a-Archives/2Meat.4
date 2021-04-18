use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CommandArgs {
    New {
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
        broadcaster_id: Uuid,
        subscriber_id: Uuid,
    },
    Exit {
        broadcaster_id: Uuid,
        subscriber_id: Uuid,
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
