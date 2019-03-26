use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        json::{JsonString, default_try_from_json},
        error::HolochainError,
    }
};
use std::convert::TryFrom;
use juniper::{FieldError, FieldResult, Value, ID};
use serde_json::json;
use crate::holochain_juniper::call_cached;
use crate::identity;
use crate::Context;

mod person;
mod message_thread;
mod message;
mod me;

use person::{Person, PersonQuerySet};
use message::{Message};
use message_thread::{MessageThread};
use me::Me;


/*=====================================
=            Input Objects            =
=====================================*/

#[derive(GraphQLInputObject)]
struct MessageThreadInput {
    participant_ids: Option<Vec<Option<String>>>,
	created_at: Option<String>
}

#[derive(GraphQLInputObject)]
struct MessageInput {
    message_thread_id: Option<String>,
    text: Option<String>,
	created_at: Option<String>
}

/*=====  End of Input Objects  ======*/


/*
 * This is the macro for defining the query schema for the graphQL provider
 * Each field is something that can be queried. These take parameters to filter as needed
 */

pub struct Query;
graphql_object!(Query: Context |&self| {

    field apiVersion(&executor) -> FieldResult<String> {
        Ok("0.0.1".to_string())
    }

    field me(&executor) -> FieldResult<Me> {
    	Ok(Me)
    }

    field messageThread(&executor, id: Option<ID>) -> FieldResult<MessageThread> {
    	match id {
    		Some(id) => Ok(MessageThread{id: id.into()}),
    		None => Err(FieldError::new("Must call with an id parameter", Value::Null))
    	}
    }

    field people(
        &executor,
	    first: Option<i32>,
	    order: Option<String>,
	    sort_by: Option<String>,
	    offset: Option<i32>,
	    search: Option<String>,
	    autocomplete: Option<String>,
	    filter: Option<String>
	) -> FieldResult<PersonQuerySet> {
    	let people: Vec<Person> = identity::get_people()?
	    	.into_iter()
	    	.map(|id| {
        		Person {
	    			id: id.into(),
    			}
	    	})
    		.collect();
    	Ok(
    		PersonQuerySet{
    			total: people.len() as i32,
    			items: people,
    		}
    	)
	}

});

/*
 * This mutation object is what allows the consumer to change the data stored in the store
 * In holochain the store is the DHT. You also need to be sure you allow some pattern (such as links)
 * such that the values can be retrieved again later
 */

 #[derive(GraphQLObject)]
struct Success {
    success: bool,
    data: String,
}
impl Success {
    pub fn new(data: String) -> Self {
        Success{
            success: true,
            data,
        }
    }
}


pub struct Mutation;
graphql_object!(Mutation: Context |&self| {

    field createMessage(&executor, data: MessageInput) -> FieldResult<Message> {
    	// let id = post_message_to_thread(
     //        executor.context().cache.borrow_mut(),
    	// 	&data.message_thread_id.unwrap_or("".into()).into(),
    	// 	data.text.unwrap_or("".into()),
     //        data.created_at.unwrap_or("".into()),
    	// )?;
    	// Ok(Message{
    	// 	id: id.into(),
    	// })
        Ok(Message{
            id: "".to_string().into()
        })
    }

    field findOrCreateThread(&executor, data: MessageThreadInput) -> FieldResult<MessageThread> {
    	let participant_hylo_ids: Vec<String> = data.participant_ids.unwrap().into_iter().map(|elem| elem.unwrap()).collect();

        let participant_agent_ids = participant_hylo_ids
            .iter()
            .map(|hylo_id| {
                identity::agent_address_from_hylo_id(hylo_id.to_owned()).unwrap().to_string()
            }).collect::<Vec<String>>().clone();

        let result_value = call_cached("chat", "get_or_create_thread", json!({"participant_ids": participant_agent_ids}).into())?;
        hdk::debug(result_value.clone())?;
        return Ok(MessageThread {
            id: result_value.as_str().unwrap().to_string().into()
        })
    }

    field registerUser(id: Option<ID>, name: Option<String>, avatar_url: Option<String>) -> FieldResult<Success> {
    	let id = identity::register_user(
    		name.unwrap_or("?".into()),
    		avatar_url.unwrap_or("".into()),
    		id.unwrap_or(juniper::ID::new("")).to_string(),
    	)?;
    	Ok(Success::new(id.to_string()))
    }

});


// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;