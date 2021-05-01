use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CommandArgs {
    New,
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

#[deprecated]
#[derive(Debug, Copy, Clone)]
pub enum Mutation {
    // TODO: mutの列挙
}
