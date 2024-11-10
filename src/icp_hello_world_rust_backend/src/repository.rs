use crate::types::ChatMessage;
use candid::Principal;
use ic_stable_structures::{
    cell::Cell as StableCell, memory_manager::{MemoryId, MemoryManager, VirtualMemory}, DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

const CHAT_MESSAGES_MEM_ID: MemoryId = MemoryId::new(20);
const GLOBAL_CONTEXT_MEM_ID: MemoryId = MemoryId::new(21);

type Memory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default(),
    ));

    static CHAT_MESSAGES: RefCell<StableBTreeMap<u64, ChatMessage, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(CHAT_MESSAGES_MEM_ID)),
        )
    );

    static GLOBAL_CONTEXT: RefCell<StableCell<String, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(GLOBAL_CONTEXT_MEM_ID)),
            String::new(),
        )
        .expect("Failed to initialize the global context cell"),
    );
}

pub fn add_chat_message(sender: Principal, content: String) {
    let timestamp = ic_cdk::api::time();

    let message = ChatMessage {
        sender,
        content,
        timestamp,
    };

    CHAT_MESSAGES.with(|chat_messages| {
        let mut chat_messages = chat_messages.borrow_mut();
        let message_id = timestamp;
        chat_messages.insert(message_id, message);
    });
}

pub fn add_chat_response(sender: Principal, content: String) {
    let timestamp = ic_cdk::api::time();

    let message = ChatMessage {
        sender,
        content,
        timestamp,
    };

    CHAT_MESSAGES.with(|chat_messages| {
        let mut chat_messages = chat_messages.borrow_mut();
        let message_id = timestamp + 1;
        chat_messages.insert(message_id, message);
    });
}

pub fn get_chat_messages() -> Vec<ChatMessage> {
    CHAT_MESSAGES.with(|chat_messages| {
        let chat_messages = chat_messages.borrow();
        chat_messages.iter().map(|(_, msg)| msg.clone()).collect()
    })
}

pub fn get_chat_history() -> Vec<ChatMessage> {
    CHAT_MESSAGES.with(|chat_messages| {
        let mut messages: Vec<ChatMessage> = chat_messages
            .borrow()
            .iter()
            .map(|(_, msg)| msg.clone())
            .collect();
        // Sort messages by timestamp
        messages.sort_by_key(|msg| msg.timestamp);
        messages
    })
}

pub fn set_global_context(context: String) {
    GLOBAL_CONTEXT.with(|global_context| {
        global_context
            .borrow_mut()
            .set(context)
            .expect("Failed to set global context");
    });
}

pub fn get_global_context() -> String {
    GLOBAL_CONTEXT.with(|global_context| global_context.borrow().get().clone())
} 