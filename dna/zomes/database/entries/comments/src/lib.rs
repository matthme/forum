use hdk::prelude::*;

mod comment;

use comment::EntryDef0;

entry_defs![
  EntryDef0::entry_def()
];

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
