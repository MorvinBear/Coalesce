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
pub struct Template {
    title: String,
    description: String,
    suggested_min: i32,
    suggested_max: i32,
    roles: Vec<TemplateRole>,
}

pub fn create(entry: Template) -> ZomeApiResult<Address> {
    let entry = Entry::App("template".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Template> {
    hdk::utils::get_as_type(address)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "template",
        description: "An archetype for organize events of a specific nature",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Template>| {
            Ok(())
        }
    )
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TemplateRole {
    name: String,
    description: String,
}
