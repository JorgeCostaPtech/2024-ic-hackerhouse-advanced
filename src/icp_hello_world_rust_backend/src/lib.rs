use candid::Principal;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::call::call_raw;
use candid::{candid_method, encode_args, decode_args, CandidType, Deserialize};

use ic_cdk::{update, query};
use gpt_encoder::Encoder;
mod types;
mod repository;

use types::ChatMessage;
use repository::{
    add_chat_message, get_chat_messages, set_global_context, get_global_context,
    add_chat_response, get_chat_history,
};

const max_tokens:u8 = 10;  

#[query]
fn get_chat_messages_api() -> Vec<ChatMessage> {
    get_chat_messages()
}

#[update]
fn set_global_context_api(context: String) {
    set_global_context(context);
}


#[query]
fn get_global_context_api() -> String {
    get_global_context()
}


#[query]
fn get_chat_history_api() -> Vec<ChatMessage> {
    get_chat_history()
}


#[update]
async fn generate_text(input_text: String) -> Result<String, String> {

    let global_context = get_global_context();


    let combined_input = format!("{}\n{}", global_context, input_text);


    let input_ids_i64: Vec<i64> = Encoder::new()
        .encode(combined_input)
        .iter()
        .map(|&id| id as i64)
        .collect();


    let canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

    let args = encode_args((max_tokens, input_ids_i64))
        .map_err(|e| format!("Failed to encode arguments: {}", e))?;

    // Make the inter-canister call using call_raw
    let raw_response = call_raw(canister_id, "model_inference", &args, 0)
        .await
        .map_err(|(code, msg)| format!("Call failed: {} - {}", code as u8, msg))?;

    let (result,): (Result<Vec<i64>, String>,) = decode_args(&raw_response)
        .map_err(|e| format!("Failed to decode response: {}", e))?;

    let output_ids = result.map_err(|e| format!("model_inference returned error: {}", e))?;

    let output_ids_u64 = output_ids
        .iter()
        .map(|&id| {
            if id >= 0 {
                Ok(id as u64)
            } else {
                Err(format!("Invalid token ID {}", id))
            }
        })
        .collect::<Result<Vec<u64>, String>>()?;

    let generated_text = Encoder::new().decode(output_ids_u64.clone());

    let user = ic_cdk::caller();
    add_chat_message(user, input_text);


    let llm_principal = Principal::anonymous(); 
    add_chat_response(llm_principal, generated_text.clone());

    Ok(generated_text)
}