use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_syntax::node::ast;
use indent::indent_by;
use indoc::formatdoc;
use itertools::Itertools;

use super::{DeriveInfo, unsupported_for_extern_diagnostic};
use crate::plugins::derive::TypeVariantInfo;

/// Adds derive result for the `Destruct` trait.
pub fn handle_destruct(
    info: &DeriveInfo,
    derived: &ast::ExprPath,
    diagnostics: &mut Vec<PluginDiagnostic>,
) -> Option<String> {
    let full_typename = info.full_typename();
    let ty = &info.name;
    let header = info.format_impl_header("core::traits", "Destruct", &["core::traits::Destruct"]);
    let body = indent_by(
        8,
        match &info.specific_info {
            TypeVariantInfo::Enum(variants) => {
                formatdoc! {"
                    match self {{
                        {}
                    }}",
                    variants.iter().map(|variant| {
                        format!(
                            "{ty}::{}(x) => core::traits::Destruct::destruct(x),",
                            variant.name,
                        )
                    }).join("\n    ")
                }
            }
            TypeVariantInfo::Struct(members) => {
                format!(
                    "let {ty} {{ {} }} = self;{}",
                    members.iter().map(|member| &member.name).join(", "),
                    members
                        .iter()
                        .map(|member| format!(
                            "\ncore::traits::Destruct::destruct({});",
                            member.name
                        ))
                        .join(""),
                )
            }
            TypeVariantInfo::Extern => {
                diagnostics.push(unsupported_for_extern_diagnostic(derived));
                return None;
            }
        },
    );

    Some(formatdoc! {"
        {header} {{
            fn destruct(self: {full_typename}) nopanic {{
                {body}
            }}
        }}
    "})
}
