use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Possibility {
    title: String,
    description: String,
    suggested_min: i32,
    suggested_max: i32,
    criteria: Vec<PossibilityCriteria>,
}

pub fn create(entry: Possibility) -> ZomeApiResult<Address> {
    let entry = Entry::App("Possibility".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Possibility> {
    hdk::utils::get_as_type(address)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "Possibility",
        description: "A template of a possible convergence of people around an idea (e.g. a Game Night) with criteria specified for limiting the convergence.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Possibility>| {
            Ok(())
        }
    )
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PossibilityCriteria {
    name: String,
    description: String,
}
