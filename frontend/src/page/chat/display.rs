
use shared::msg::chat_message::{ChatMessage, ChatMessageWithIndex};

use crate::prelude::*;

pub struct ChatDisplay {
    pub messages: MutableVec<ChatMessageWithIndex>,
    pub kind: ContractKind,
}

impl ChatDisplay {
    pub fn new(kind: ContractKind) -> Self {
        Self {
            kind,
            messages: MutableVec::new(),
        }
    }

    pub fn render(&self) -> Dom {
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("overflow-y", "auto")
                .style("flex", "1")
                .style("border-top", "1px solid black")
                .style("border-bottom", "1px solid black")
            }
        });
        static CONTENT:Lazy<String> = Lazy::new(|| {
            class! {
                .style("font-size", "1.5rem")
                .style("padding", "1rem")
            }
        });

        let kind = self.kind;
        html!("div", {
            .class(&*CONTAINER)
            .child(html!("div", {
                .class(&*CONTENT)
                .children_signal_vec(self.messages.signal_vec_cloned().map(move |msg| {
                    match kind {
                        ContractKind::Client => {
                            ClientMessage::new(msg).render()
                        }
                        ContractKind::Server => {
                            ServerMessage::new(msg).render()
                        }
                    }
                }))
            }))
        })
    }

    pub fn add_messages(&self, messages: Vec<ChatMessageWithIndex>) {
        self.messages.lock_mut().extend(messages);
    }
}

struct ClientMessage {
    msg: ChatMessageWithIndex,
}

impl ClientMessage {
    pub fn new(msg: ChatMessageWithIndex) -> Self {
        Self {
            msg
        }
    }

    pub fn render(&self) -> Dom {
        let ChatMessage {user, message, network_id} = &self.msg.msg;

        // TODO - delete / edit ?
        html!("div", {
            .text(&format!("({}) {}: {}", network_id, user, message))
        })
    }
}

struct ServerMessage {
    msg: ChatMessageWithIndex,
}

impl ServerMessage {
    pub fn new(msg: ChatMessageWithIndex) -> Self {
        Self {
            msg
        }
    }

    pub fn render(&self) -> Dom {
        let ChatMessage {user, message, network_id} = &self.msg.msg;

        html!("div", {
            .text(&format!("({}) {}: {}", network_id, user, message))
        })
    }
}
