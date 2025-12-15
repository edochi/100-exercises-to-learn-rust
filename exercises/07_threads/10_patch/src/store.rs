use crate::data::{Status, Ticket, TicketDraft, TicketPatch};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    // pub fn update(&mut self, patch: TicketPatch) -> TicketId {
    //     let ticket = self.get_mut(patch.id).unwrap();
    //     if patch.title.is_some() {
    //         ticket.title = patch.title.unwrap();
    //     };
    //     if patch.description.is_some() {
    //         ticket.description = patch.description.unwrap();
    //     };
    //     if patch.status.is_some() {
    //         ticket.status = patch.status.unwrap();
    //     };
    //     patch.id
    // }
}
