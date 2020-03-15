use hdk::{
    entry_definition::ValidatingEntryType,
    entry_definition::ValidatingLinkDefinition,
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
pub struct Instance {
    name: String,
    description: String
}

pub fn create(entry: Instance) -> ZomeApiResult<Address> {
    let entry = Entry::App("instant".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Instance> {
    hdk::utils::get_as_type(address)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "instance",
        description: "An attempt to organize a happening",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Instance>| {
            Ok(())
        },

        links: [ link_to_template() ]
    )
}

pub fn link_to_template() -> ValidatingLinkDefinition {
    link!(
       direction: hdk::LinkDirection::To,
       other_type: "template",
       link_type: "generatedFrom",
       validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
       },
        validation: | _validation_data: hdk::LinkValidationData | {
            Ok(())
        }
    )
}
