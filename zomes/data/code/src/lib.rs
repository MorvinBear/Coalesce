#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

mod template;
mod instance;

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
       template::definition(),
       instance::definition()
    ]

    init: || { Ok(()) }

    validate_agent: | validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_template: {
            inputs: |entry: template::Template|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: template::create
        }
        get_template: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<template::Template>|,
            handler: template::get
        }
        create_instance: {
            inputs: |entry: instance::Instance|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: instance::create
        }
        get_instance: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<instance::Instance>|,
            handler: instance::get
        }
    ]

    traits: {
        hc_public [create_my_entry, get_my_entry]
    }
}
