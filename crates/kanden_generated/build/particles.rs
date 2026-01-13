use std::collections::HashMap;

use heck::ToShoutySnakeCase;
use kanden_build_utils::rerun_if_changed;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Deserialize)]
struct Particles {
    particle_type: HashMap<String, i32>,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    rerun_if_changed(["extracted/misc.json"]);

    let particles = serde_json::from_str::<Particles>(include_str!("../extracted/misc.json"))?;

    let mut id_consts = TokenStream::new();

    let mut sorted_particle_types = particles.particle_type.iter().collect::<Vec<_>>();
    sorted_particle_types.sort_by(|(_, v1), (_, v2)| v1.cmp(&v2));
    for (key, value) in sorted_particle_types {
        let name = key.to_shouty_snake_case();
        let name_ident = format_ident!("ID_{}", name);
        let id = value;

        let def = quote! {
            pub const #name_ident: i32 = #id;
        };

        id_consts.extend([def]);
    }

    Ok(quote! {
        #id_consts
    })
}
