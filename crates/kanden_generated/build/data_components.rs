use std::collections::HashMap;

use heck::ToShoutySnakeCase;
use kanden_build_utils::rerun_if_changed;
use kanden_ident::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataComponentType {
    id: u32,
    name: String,
    ignore_swap_animation: bool,
    is_transient: bool,
}

#[derive(Deserialize)]
struct DataComponents {
    types: HashMap<Ident<String>, DataComponentType>,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    rerun_if_changed(["extracted/data_components.json"]);

    let data_components =
        serde_json::from_str::<DataComponents>(include_str!("../extracted/data_components.json"))?;

    let mut id_consts = TokenStream::new();

    let mut sorted_data_component_types = data_components.types.iter().collect::<Vec<_>>();
    sorted_data_component_types.sort_by(|(_, v1), (_, v2)| v1.id.cmp(&v2.id));
    for (key, value) in sorted_data_component_types {
        let name = key.path().to_shouty_snake_case();
        let name_ident = format_ident!("{}", name);
        let id = value.id as usize;

        let def = quote! {
            pub const #name_ident: usize = #id;
        };

        id_consts.extend([def]);
    }

    Ok(quote! {
        pub struct DataComponentId;

        impl DataComponentId {
            #id_consts
        }
    })
}
