use juniper::{FieldResult, ID};
use serde_json::{
  self,
  json,
};
use hdk::holochain_core_types::error::HolochainError;
use hdk::holochain_core_types::json::JsonString;
use hdk::error::ZomeApiResult;
use std::convert::TryFrom;
use crate::holochain_juniper::call_cached;
use crate::holochain_juniper::HID;
use crate::schema::post::Post;
use crate::Context;
use super::person::Person;

#[derive(Constructor, Clone)]
pub struct Comment {
    pub id: HID,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct CommentEntry {
    pub timestamp: String,
    pub text: String,
    pub base: String,
    pub creator: String,
}

impl Comment {
	pub fn retrieve_entry(&self) -> ZomeApiResult<CommentEntry> {
		let id: String = self.id.clone().into();
		let result = JsonString::from(call_cached("comments", "get_comment", json!({"address": id}).into())?);
		let comment_entry = CommentEntry::try_from(result)?;
		Ok(comment_entry)
	}
}

graphql_object!(Comment: Context |&self| {
    field id() -> ID {
    	self.id.clone().into()
    }

    field creator() -> FieldResult<Person> {
    	let id: String = self.retrieve_entry()?.creator;
    	Ok(Person{id: id.into()})
    }

    field createdAt() -> FieldResult<String> {
      Ok(self.retrieve_entry()?.timestamp)
    }

    field createdFrom() -> String {
    	"createdFrom".into()
    }

    field text() -> FieldResult<String> {
    	Ok(self.retrieve_entry()?.text)
    }

    field post() -> FieldResult<Post> {
    	let id: String = self.retrieve_entry()?.base;
    	Ok(Post{id: id.into()})
    }

    field attachments() -> FieldResult<Vec<Attachment>> {
      Ok(Vec::new())
    }
});



/*
type CommentQuerySet {
  total: Int
  hasMore: Boolean
  items: [Comment]
}
*/
#[derive(Constructor, Clone)]
pub struct CommentQuerySet {
    pub total: i32,
    pub items: Vec<Comment>,
}
graphql_object!(CommentQuerySet: Context |&self| {
  field total() -> i32 {
    self.total
  }

  field hasMore() -> bool {
    false
  }

  field items() -> Option<Vec<Option<Comment>>> {
    Some(self.items.iter().map(|item| Some(item.clone())).collect())
  }
});

#[derive(GraphQLObject)]
pub struct Attachment {
  id: ID,
  url: String,
}
