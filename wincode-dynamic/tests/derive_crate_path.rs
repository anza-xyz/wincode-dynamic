//! The derive must reach wincode through the configured crate path rather than
//! assume that `wincode` resolves at the expansion site. This is what crates
//! re-exporting wincode-dynamic, without their users depending on wincode
//! directly, rely on.

use wincode_dynamic::{RootSchema, SchemaDynamic, SerializedSize};

// Shadows the `wincode` crate, so any bare `wincode::` path in the derive
// output fails to resolve.
#[allow(dead_code)]
mod wincode {}

/// A re-export of both crates, as a downstream crate would provide.
mod reexports {
    pub use {::wincode::*, ::wincode_dynamic::*};
}

#[derive(::wincode::SchemaRead, ::wincode::SchemaWrite, ::wincode_dynamic::SchemaDynamic)]
#[wincode(crate = "crate::reexports")]
struct Struct {
    value: u64,
}

#[derive(::wincode::SchemaRead, ::wincode::SchemaWrite, ::wincode_dynamic::SchemaDynamic)]
#[wincode(crate = "crate::reexports")]
enum Enum {
    Unit,
    Value { value: u32 },
}

#[test]
fn derive_resolves_wincode_through_crate_path() {
    assert_eq!(Struct::SERIALIZED_SIZE, SerializedSize::Static(8));
    assert!(matches!(Struct::schema(), RootSchema::Struct(_)));
    assert!(matches!(Enum::schema(), RootSchema::Enum { .. }));
}
