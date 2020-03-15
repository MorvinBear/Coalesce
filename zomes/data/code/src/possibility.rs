use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};
use hdk::entry_definition::ValidatingLinkDefinition;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Possibility {
    title: String,
    description: String,
    suggested_min: i32,
    suggested_max: i32,
    criteria: Vec<PossibilityCriteria>,
}

// PossibilityResult is used by list_possiblities
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PossibilityResult {
    address: Address,
    title: String
}

pub fn create(entry: Possibility) -> ZomeApiResult<Address> {
    let entry = Entry::App("Possibility".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;

    let anchor = crate::anchor::anchor("possibilities")?;

    hdk::link_entries(&anchor, &address, "listing", "")?;

    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Possibility> {
    hdk::utils::get_as_type(address)
}

pub fn list() -> ZomeApiResult<Vec<PossibilityResult>> {
    let anchor = crate::anchor::address("possibilities")?;
    let links = hdk::get_links(
        &anchor, LinkMatch::Exactly("listing"), LinkMatch::Any)?.addresses();

    // TODO see which listings are in local store to avoid DHT calls

    let mut results: Vec<PossibilityResult> = Vec::new();

    for address in links {
        let entry = get(address.clone());
        results.push(PossibilityResult { address, title: entry?.title })
    }

    Ok(results)
}

pub fn listing_link() -> ValidatingLinkDefinition {
    link!(
       direction: hdk::LinkDirection::To,
       other_type: "Possibility",
       link_type: "listing",
       validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
       },
        validation: | _validation_data: hdk::LinkValidationData | {
            Ok(())
        }
    )
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
