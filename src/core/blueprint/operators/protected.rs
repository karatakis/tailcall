use crate::core::blueprint::FieldDefinition;
use crate::core::config::{self, ConfigModule, Field};
use crate::core::ir::model::IR;
use crate::core::try_fold::TryFold;
use crate::core::valid::Valid;

pub fn update_protected<'a>(
    type_name: &'a str,
) -> TryFold<
    'a,
    (&'a ConfigModule, &'a Field, &'a config::Type, &'a str),
    FieldDefinition,
    miette::MietteDiagnostic,
> {
    TryFold::<
        (&ConfigModule, &Field, &config::Type, &'a str),
        FieldDefinition,
        miette::MietteDiagnostic,
    >::new(|(config, field, type_, _), mut b_field| {
        if field.protected.is_some() // check the field itself has marked as protected
                || type_.protected.is_some() // check the type that contains current field
                || config // check that output type of the field is protected
                    .find_type(field.type_of.name())
                    .and_then(|type_| type_.protected.as_ref())
                    .is_some()
        {
            if config.input_types().contains(type_name) {
                return Valid::fail(miette::diagnostic!("Input types can not be protected"));
            }

            if !config.extensions().has_auth() {
                return Valid::fail(
                        miette::diagnostic!("@protected operator is used but there is no @link definitions for auth providers"),
                    );
            }

            b_field.resolver = Some(IR::Protect(Box::new(
                b_field
                    .resolver
                    .unwrap_or(IR::ContextPath(vec![b_field.name.clone()])),
            )));
        }

        Valid::succeed(b_field)
    })
}
