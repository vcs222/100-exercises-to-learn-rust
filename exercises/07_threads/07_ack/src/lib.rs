use data::{Ticket, TicketDraft};
use std::sync::mpsc::{Receiver, Sender};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft: ticket,
                response_sender,
            }) => {
                response_sender
                    .send(store.add_ticket(TicketDraft {
                        title: ticket.title,
                        description: ticket.description,
                    }))
                    .unwrap();
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                if let Some(ticket) = store.get(id) {
                    println!("Ticket: {:?}", &ticket);
                    response_sender.send(Some(ticket.clone())).unwrap();
                } else {
                    println!("Ticket not found");
                    response_sender.send(None).unwrap();
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
