#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

mod possibility;
mod convergence;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_persistence_api::{
    cas::content::Address,
};
use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

define_zome! {
    entries: [
       possibility::definition(),
       convergence::definition()
    ]

    init: || { Ok(()) }

    validate_agent: | validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_possibility: {
            inputs: |entry: possibility::Possibility|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: possibility::create
        }
        get_possibility: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<possibility::Possibility>|,
            handler: possibility::get
        }
        create_convergence: {
            inputs: |entry: convergence::Convergence|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: convergence::create
        }
        get_convergence: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<convergence::Convergence>|,
            handler: convergence::get
        }
    ]

    traits: {
         hc_public [
            create_possibility,
            get_possibility,
            create_convergence,
            get_convergence
         ]
    }
}
