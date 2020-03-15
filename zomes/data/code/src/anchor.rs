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
pub struct Anchor {
    name: String
}

pub fn anchor(name: &str) -> ZomeApiResult<Address> {
    let anchor = Anchor { name: name.to_string() };
    let entry = Entry::App("Anchor".into(), anchor.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn address(name: &str) -> ZomeApiResult<Address> {
    let anchor = Anchor { name: name.to_string() };
    let entry = Entry::App("Anchor".into(), anchor.into());
    return hdk::entry_address(&entry);
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "Anchor",
        description: "Shared addresses to form directories.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Anchor>| {
            Ok(())
        },

        links: [ crate::possibility::listing_link() ]
    )
}
