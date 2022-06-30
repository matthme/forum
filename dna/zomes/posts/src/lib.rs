use hdk::prelude::*;

mod post;

use post::Post;

entry_defs![
  Post::entry_def()
];

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
