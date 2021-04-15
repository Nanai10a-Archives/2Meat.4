use uuid::Uuid;

use crate::model::arg::Target;

pub struct Transferer {
    ids: Vec<(Uuid, Target)>,
}

impl Transferer {
    pub fn new() -> Self {
        Transferer { ids: Vec::new() }
    }

    pub fn contains(&self, id: Uuid) -> bool {
        self.which_is(id).is_ok()
    }

    pub fn which_is(&self, id: Uuid) -> anyhow::Result<Target> {
        let vec = self
            .ids
            .iter()
            .filter(|(uuid, _)| *uuid == id)
            .collect::<Vec<_>>();

        match vec.len() {
            0..1 => (),
            _ => todo!(),
        }

        match vec.first() {
            None => Err(anyhow::Error::msg("not found.")),
            Some(item) => Ok((**item).1),
        }
    }
}
