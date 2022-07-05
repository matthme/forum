use hdk::prelude::*;

mod entry_def_0;

use entry_def_0::EntryDef0;

entry_defs![
  EntryDef0::entry_def()
];

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
