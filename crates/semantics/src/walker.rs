#![allow(clippy::too_many_arguments)]

use mago_ast::ast::*;
use mago_ast::*;
use mago_interner::StringIdentifier;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_span::Span;
use mago_walker::Walker;

use crate::consts::ANONYMOUS_CLASS_NAME;
use crate::consts::CALL_MAGIC_METHOD;
use crate::consts::CALL_STATIC_MAGIC_METHOD;
use crate::consts::CLONE_MAGIC_METHOD;
use crate::consts::CONSTRUCTOR_MAGIC_METHOD;
use crate::consts::DEBUG_INFO_MAGIC_METHOD;
use crate::consts::DECLARE_DIRECTIVES;
use crate::consts::DESTRUCTOR_MAGIC_METHOD;
use crate::consts::ENCODING_DECLARE_DIRECTIVE;
use crate::consts::GET_MAGIC_METHOD;
use crate::consts::INVOKE_MAGIC_METHOD;
use crate::consts::ISSET_MAGIC_METHOD;
use crate::consts::MAGIC_METHODS;
use crate::consts::RESERVED_KEYWORDS;
use crate::consts::SERIALIZE_MAGIC_METHOD;
use crate::consts::SET_MAGIC_METHOD;
use crate::consts::SET_STATE_MAGIC_METHOD;
use crate::consts::SLEEP_MAGIC_METHOD;
use crate::consts::SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED;
use crate::consts::STRICT_TYPES_DECLARE_DIRECTIVE;
use crate::consts::TICKS_DECLARE_DIRECTIVE;
use crate::consts::TO_STRING_MAGIC_METHOD;
use crate::consts::UNSERIALIZE_MAGIC_METHOD;
use crate::consts::UNSET_MAGIC_METHOD;
use crate::consts::WAKEUP_MAGIC_METHOD;
use crate::context::Context;

#[derive(Clone, Debug)]
pub struct SemanticsWalker;

impl SemanticsWalker {
    fn process_extends(
        &self,
        extends: &Extends,
        class_like_span: Span,
        class_like_kind: &str,
        class_like_name: &str,
        class_like_fqcn: &str,
        extension_limit: bool,
        context: &mut Context<'_>,
    ) {
        if extension_limit && extends.types.len() > 1 {
            context.report(
                Issue::error(format!(
                    "{} `{}` can only extend one other tyoe, found {}",
                    class_like_kind,
                    class_like_name,
                    extends.types.len()
                ))
                .with_annotation(Annotation::primary(extends.span()))
                .with_annotation(
                    Annotation::secondary(class_like_span)
                        .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                ),
            );
        }

        for extended_type in extends.types.iter() {
            let extended_fqcn = context.lookup_name(&extended_type.span().start);

            if extended_fqcn.eq_ignore_ascii_case(class_like_fqcn) {
                context.report(
                    Issue::error(format!("{} `{}` cannot extend itself", class_like_kind, class_like_name))
                        .with_annotation(Annotation::primary(extended_type.span()))
                        .with_annotation(
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                        ),
                );
            }
        }

        for extended_type in extends.types.iter() {
            let extended_name = context.interner.lookup(&extended_type.value());

            if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(extended_name))
                || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED
                    .iter()
                    .any(|keyword| keyword.eq_ignore_ascii_case(extended_name))
            {
                context.report(
                    Issue::error(format!(
                        "{} `{}` cannot extend reserved keyword `{}`",
                        class_like_kind, class_like_name, extended_name
                    ))
                    .with_annotation(Annotation::primary(extended_type.span()))
                    .with_annotation(
                        Annotation::secondary(class_like_span)
                            .with_message(format!("{} `{}` defined here", class_like_kind, class_like_name)),
                    ),
                );
            }
        }
    }

    fn process_implements(
        &self,
        implements: &Implements,
        class_like_span: Span,
        class_like_kind: &str,
        class_like_name: &str,
        class_like_fqcn: &str,
        check_for_self_implement: bool,
        context: &mut Context<'_>,
    ) {
        if check_for_self_implement {
            for implemented_type in implements.types.iter() {
                let implemented_fqcn = context.lookup_name(&implemented_type.span().start);

                if implemented_fqcn.eq_ignore_ascii_case(class_like_fqcn) {
                    context.report(
                        Issue::error(format!("{} `{}` cannot implement itself", class_like_kind, class_like_name))
                            .with_annotation(Annotation::primary(implemented_type.span()))
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                    );
                }
            }
        }

        for implemented_type in implements.types.iter() {
            let implemented_name = context.interner.lookup(&implemented_type.value());

            if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(implemented_name))
                || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED
                    .iter()
                    .any(|keyword| keyword.eq_ignore_ascii_case(implemented_name))
            {
                context.report(
                    Issue::error(format!(
                        "{} `{}` cannot implement reserved keyword `{}`",
                        class_like_kind, class_like_name, implemented_name
                    ))
                    .with_annotation(Annotation::primary(implemented_type.span()))
                    .with_annotation(
                        Annotation::secondary(class_like_span)
                            .with_message(format!("{} `{}` defined here", class_like_kind, class_like_name)),
                    ),
                );
            }
        }
    }

    fn process_property(
        &self,
        property: &Property,
        class_like_span: Span,
        class_like_kind: &str,
        class_like_name: &str,
        class_like_fqcn: &str,
        class_like_is_interface: bool,
        context: &mut Context<'_>,
    ) {
        let first_variable = property.first_variable();
        let first_variable_id = first_variable.name;
        let first_variable_name = context.interner.lookup(&first_variable_id);

        let modifiers = property.modifiers();
        let mut last_final: Option<Span> = None;
        let mut last_static: Option<Span> = None;
        let mut last_readonly: Option<Span> = None;
        let mut last_read_visibility: Option<Span> = None;
        let mut last_write_visibility: Option<Span> = None;

        for modifier in modifiers.iter() {
            match modifier {
                Modifier::Abstract(_) => {
                    context.report(
                        Issue::error(format!(
                            "property `{}:{}` cannot be declared abstract",
                            class_like_name, first_variable_name
                        ))
                        .with_annotation(Annotation::primary(modifier.span()))
                        .with_annotation(
                            Annotation::secondary(first_variable.span())
                                .with_message(format!("property `{}`", first_variable_name)),
                        )
                        .with_annotation(
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                        ),
                    );
                }
                Modifier::Static(_) => {
                    if let Some(last_readonly) = last_readonly {
                        context.report(
                            Issue::error(format!(
                                "readonly property `{}::{}` cannot be static",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(last_readonly))
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_static) = last_static {
                        context.report(
                            Issue::error(format!(
                                "property `{}::{}` has multiple `static` modifiers",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(
                                Annotation::secondary(last_static).with_message("previous `static` modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_visibility) = last_write_visibility {
                        context.report(
                            Issue::error(format!(
                                "static property `{}::{}` cannot have a write visibility modifier",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(last_visibility))
                            .with_annotation(Annotation::primary(modifier.span()).with_message("`static` modifier"))
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_static = Some(modifier.span());
                }
                Modifier::Readonly(_) => {
                    if let Some(last_static) = last_static {
                        context.report(
                            Issue::error(format!(
                                "static property `{}::{}` cannot be readonly",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(Annotation::primary(last_static).with_message("`static` modifier"))
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_readonly) = last_readonly {
                        context.report(
                            Issue::error(format!(
                                "property `{}::{}` has multiple `readonly` modifiers",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(
                                Annotation::secondary(last_readonly).with_message("previous `readonly` modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_readonly = Some(modifier.span());
                }
                Modifier::Final(_) => {
                    if let Some(last_final) = last_final {
                        context.report(
                            Issue::error("property has multiple `final` modifiers")
                                .with_annotation(Annotation::primary(modifier.span()))
                                .with_annotation(
                                    Annotation::primary(last_final).with_message("previous `final` modifier"),
                                )
                                .with_annotation(
                                    Annotation::secondary(first_variable.span())
                                        .with_message(format!("property `{}`", first_variable_name)),
                                )
                                .with_annotation(
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ),
                        );
                    }

                    last_final = Some(modifier.span());
                }
                Modifier::Private(_) | Modifier::Protected(_) | Modifier::Public(_) => {
                    if let Some(last_visibility) = last_read_visibility {
                        context.report(
                            Issue::error(format!(
                                "property `{}::{}` has multiple visibility modifiers",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(
                                Annotation::primary(last_visibility).with_message("previous visibility modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_read_visibility = Some(modifier.span());
                }
                Modifier::PrivateSet(_) | Modifier::ProtectedSet(_) | Modifier::PublicSet(_) => {
                    if let Some(last_visibility) = last_write_visibility {
                        context.report(
                            Issue::error(format!(
                                "property `{}::{}` has multiple write visibility modifiers",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(
                                Annotation::primary(last_visibility).with_message("previous write visibility modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_static) = last_static {
                        context.report(
                            Issue::error(format!(
                                "static property `{}::{}` cannot have a write visibility modifier",
                                class_like_name, first_variable_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(Annotation::primary(last_static).with_message("`static` modifier"))
                            .with_annotation(
                                Annotation::secondary(first_variable.span())
                                    .with_message(format!("property `{}`", first_variable_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_write_visibility = Some(modifier.span());
                }
            }
        }

        if let Some(var) = property.var() {
            if !modifiers.is_empty() {
                let first = modifiers.first().unwrap();
                let last = modifiers.last().unwrap();

                context.report(
                    Issue::error(format!(
                        "var property `{}::{}` cannot have modifiers",
                        class_like_name, first_variable_name
                    ))
                    .with_annotation(Annotation::primary(first.span().join(last.span())))
                    .with_annotation(Annotation::primary(var.span()).with_message("`var` is here"))
                    .with_annotation(
                        Annotation::secondary(first_variable.span())
                            .with_message(format!("property `{}`", first_variable_name)),
                    )
                    .with_annotation(
                        Annotation::secondary(class_like_span)
                            .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                    )
                    .with_help("remove either the `var` keyword, or the modifiers".to_string()),
                );
            }
        }

        if let Some(hint) = property.hint() {
            if hint.is_bottom() {
                let hint_name = context.lookup_hint(hint);
                // cant be used on properties
                context.report(
                    Issue::error(format!(
                        "property `{}::{}` cannot have type `{}`",
                        class_like_name, first_variable_name, hint_name
                    ))
                    .with_annotation(
                        Annotation::primary(hint.span())
                            .with_message(format!("type `{}` is not allowed on properties", hint_name)),
                    )
                    .with_annotation(
                        Annotation::secondary(first_variable.span())
                            .with_message(format!("property `{}`", first_variable_name)),
                    )
                    .with_annotation(
                        Annotation::secondary(class_like_span)
                            .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                    ),
                );
            }
        } else if let Some(readonly) = last_readonly {
            // readonly properties must have a type hint
            context.report(
                Issue::error(format!(
                    "readonly property `{}::{}` must have a type hint",
                    class_like_name, first_variable_name
                ))
                .with_annotation(Annotation::primary(readonly))
                .with_annotation(
                    Annotation::secondary(first_variable.span())
                        .with_message(format!("property `{}`", first_variable_name)),
                )
                .with_annotation(
                    Annotation::secondary(class_like_span)
                        .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                ),
            );
        }

        match &property {
            Property::Plain(plain_property) => {
                for item in plain_property.items.iter() {
                    if let PropertyItem::Concrete(property_concrete_item) = &item {
                        let item_name_id = property_concrete_item.variable.name;
                        let item_name = context.interner.lookup(&item_name_id);

                        if !property_concrete_item.value.is_constant(false) {
                            context.report(
                                Issue::error(format!(
                                    "property `{}::{}` value contains a non-constant expression",
                                    class_like_name, item_name
                                ))
                                .with_annotation(
                                    Annotation::primary(property_concrete_item.value.span())
                                        .with_message("non-constant expression"),
                                )
                                .with_annotation(
                                    Annotation::secondary(property_concrete_item.variable.span())
                                        .with_message(format!("property `{}` defined here", item_name)),
                                )
                                .with_annotation(
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ),
                            );
                        }

                        if let Some(readonly) = last_readonly {
                            context.report(
                                Issue::error(format!(
                                    "readonly property `{}::{}` cannot have a default value",
                                    class_like_name, item_name
                                ))
                                .with_annotation(
                                    Annotation::primary(property_concrete_item.value.span())
                                        .with_message("default value here"),
                                )
                                .with_annotation(Annotation::primary(readonly).with_message(format!(
                                    "property `{}::{}` is marked as readonly",
                                    class_like_name, item_name
                                )))
                                .with_annotation(
                                    Annotation::secondary(property_concrete_item.variable.span())
                                        .with_message(format!("property `{}` defined here", item_name)),
                                )
                                .with_annotation(
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ),
                            );
                        }
                    }
                }
            }
            Property::Hooked(hooked_property) => {
                let item_name_id = hooked_property.item.variable().name;
                let item_name = context.interner.lookup(&item_name_id);

                if let Some(readonly) = last_readonly {
                    context.report(
                        Issue::error(format!(
                            "hooked property `{}::{}` cannot be readonly",
                            class_like_name, item_name
                        ))
                        .with_annotation(Annotation::primary(hooked_property.hooks.span()))
                        .with_annotation(Annotation::primary(readonly).with_message(format!(
                            "property `{}::{}` is marked as readonly",
                            class_like_name, item_name
                        )))
                        .with_annotation(
                            Annotation::secondary(hooked_property.item.variable().span())
                                .with_message(format!("property `{}` defined here", item_name)),
                        )
                        .with_annotation(
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                        ),
                    );
                }

                if let Some(r#static) = last_static {
                    context.report(
                        Issue::error(format!("hooked property `{}::{}` cannot be static", class_like_name, item_name))
                            .with_annotation(Annotation::primary(hooked_property.hooks.span()))
                            .with_annotation(Annotation::primary(r#static).with_message(format!(
                                "property `{}::{}` is marked as static",
                                class_like_name, item_name
                            )))
                            .with_annotation(
                                Annotation::secondary(hooked_property.item.variable().span())
                                    .with_message(format!("property `{}` defined here", item_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                    );
                }

                let mut hook_names: Vec<(std::string::String, Span)> = vec![];
                for hook in hooked_property.hooks.hooks.iter() {
                    let name = context.interner.lookup(&hook.name.value);
                    let lowered_name = name.to_ascii_lowercase();

                    if !hook.modifiers.is_empty() {
                        let first = hook.modifiers.first().unwrap();
                        let last = hook.modifiers.last().unwrap();

                        context.report(
                            Issue::error(format!(
                                "hook `{}` for property `{}::{}` cannot have modifiers",
                                name, class_like_name, item_name
                            ))
                            .with_annotation(
                                Annotation::primary(first.span().join(last.span())).with_message("hook modifiers here"),
                            )
                            .with_annotation(
                                Annotation::secondary(hooked_property.item.variable().span())
                                    .with_message(format!("property `{}` defined here", item_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if !class_like_is_interface {
                        if let PropertyHookBody::Abstract(property_hook_abstract_body) = &hook.body {
                            context.report(
                                Issue::error(format!("non-abstract property hook `{}` must have a body", name))
                                    .with_annotation(Annotation::primary(property_hook_abstract_body.span()))
                                    .with_annotation(
                                        Annotation::secondary(hook.name.span())
                                            .with_message(format!("hook `{}` defined here", name)),
                                    )
                                    .with_annotation(
                                        Annotation::secondary(hooked_property.item.variable().span())
                                            .with_message(format!("property `{}` defined here", item_name)),
                                    )
                                    .with_annotation(Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    ))),
                            );
                        }
                    }

                    match lowered_name.as_str() {
                        "set" => {
                            if let Some(parameters) = &hook.parameters {
                                if parameters.parameters.len() != 1 {
                                    context.report(
                                        Issue::error(format!(
                                            "hook `{}` of property `{}::{}` must accept exactly one parameter, found {}",
                                            name, class_like_name, item_name, parameters.parameters.len()
                                        ))
                                        .with_annotation(Annotation::primary(parameters.span()))
                                        .with_annotation(
                                            Annotation::secondary(hook.name.span()).with_message(
                                                format!("hook `{}` defined here", name),
                                            ),
                                        )
                                        .with_annotation(
                                            Annotation::secondary(hooked_property.item.variable().span())
                                                .with_message(format!("property `{}` defined here", item_name)),
                                        )
                                        .with_annotation(
                                            Annotation::secondary(class_like_span)
                                                .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                                        ),
                                    );
                                } else {
                                    let first_parameter = parameters.parameters.first().unwrap();
                                    let first_parameter_name = context.interner.lookup(&first_parameter.variable.name);

                                    if first_parameter.hint.is_none() {
                                        context.report(
                                            Issue::error(format!(
                                                "parameter `{}` of hook `{}::{}::{}` must contain a type hint",
                                                first_parameter_name, class_like_name, item_name, name
                                            ))
                                            .with_annotation(Annotation::primary(first_parameter.variable.span()))
                                            .with_annotation(
                                                Annotation::secondary(hook.name.span())
                                                    .with_message(format!("hook `{}` defined here", name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hooked_property.item.variable().span())
                                                    .with_message(format!("property `{}` defined here", item_name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(class_like_span).with_message(format!(
                                                    "{} `{}` defined here",
                                                    class_like_kind, class_like_fqcn
                                                )),
                                            ),
                                        );
                                    }

                                    if let Some(ellipsis) = first_parameter.ellipsis {
                                        context.report(
                                            Issue::error(format!(
                                                "parameter `{}` of hook `{}::{}::{}` must not be variadic",
                                                first_parameter_name, class_like_name, item_name, name
                                            ))
                                            .with_annotation(Annotation::primary(ellipsis.span()))
                                            .with_annotation(
                                                Annotation::secondary(first_parameter.variable.span()).with_message(
                                                    format!("parameter `{}` defined here", first_parameter_name),
                                                ),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hook.name.span())
                                                    .with_message(format!("hook `{}` defined here", name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hooked_property.item.variable().span())
                                                    .with_message(format!("property `{}` defined here", item_name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(class_like_span).with_message(format!(
                                                    "{} `{}` defined here",
                                                    class_like_kind, class_like_fqcn
                                                )),
                                            ),
                                        );
                                    }

                                    if let Some(ampersand) = first_parameter.ampersand {
                                        context.report(
                                            Issue::error(format!(
                                                "parameter `{}` of hook `{}::{}::{}` must not be pass-by-reference",
                                                first_parameter_name, class_like_name, item_name, name
                                            ))
                                            .with_annotation(Annotation::primary(ampersand.span()))
                                            .with_annotation(
                                                Annotation::secondary(first_parameter.variable.span()).with_message(
                                                    format!("parameter `{}` defined here", first_parameter_name),
                                                ),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hook.name.span())
                                                    .with_message(format!("hook `{}` defined here", name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hooked_property.item.variable().span())
                                                    .with_message(format!("property `{}` defined here", item_name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(class_like_span).with_message(format!(
                                                    "{} `{}` defined here",
                                                    class_like_kind, class_like_fqcn
                                                )),
                                            ),
                                        );
                                    }

                                    if let Some(default_value) = &first_parameter.default_value {
                                        context.report(
                                            Issue::error(format!(
                                                "parameter `{}` of hook `{}::{}::{}` must not have a default value",
                                                first_parameter_name, class_like_name, item_name, name
                                            ))
                                            .with_annotation(Annotation::primary(default_value.span()))
                                            .with_annotation(
                                                Annotation::secondary(first_parameter.variable.span()).with_message(
                                                    format!("parameter `{}` defined here", first_parameter_name),
                                                ),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hook.name.span())
                                                    .with_message(format!("hook `{}` defined here", name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(hooked_property.item.variable().span())
                                                    .with_message(format!("property `{}` defined here", item_name)),
                                            )
                                            .with_annotation(
                                                Annotation::secondary(class_like_span).with_message(format!(
                                                    "{} `{}` defined here",
                                                    class_like_kind, class_like_fqcn
                                                )),
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                        "get" => {
                            if let Some(parameters) = &hook.parameters {
                                context.report(
                                    Issue::error(format!(
                                        "hook `{}` of property `{}::{}` must not have a parameters list",
                                        name, class_like_name, item_name
                                    ))
                                    .with_annotation(Annotation::primary(parameters.span()))
                                    .with_annotation(
                                        Annotation::secondary(hook.name.span())
                                            .with_message(format!("hook `{}` defined here", name)),
                                    )
                                    .with_annotation(
                                        Annotation::secondary(hooked_property.item.variable().span())
                                            .with_message(format!("property `{}` defined here", item_name)),
                                    )
                                    .with_annotation(
                                        Annotation::secondary(class_like_span).with_message(format!(
                                            "{} `{}` defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                    ),
                                );
                            }
                        }
                        _ => {
                            context.report(
                                Issue::error(format!(
                                    "hooked property `{}::{}` contains an unknwon hook `{}`, expected `set` or `get`",
                                    class_like_name, item_name, name
                                ))
                                .with_annotation(
                                    Annotation::primary(hook.name.span())
                                        .with_message(format!("hook `{}` defined here", name)),
                                )
                                .with_annotation(
                                    Annotation::secondary(hooked_property.item.variable().span())
                                        .with_message(format!("property `{}` defined here", item_name)),
                                )
                                .with_annotation(
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ),
                            );
                        }
                    }

                    if let Some((_, previous_span)) = hook_names.iter().find(|(previous, _)| previous.eq(&lowered_name))
                    {
                        context.report(
                            Issue::error(format!(
                                "hook `{}` has already been defined for property `{}::{}`",
                                name, class_like_name, item_name
                            ))
                            .with_annotation(Annotation::primary(hook.name.span()))
                            .with_annotation(
                                Annotation::secondary(*previous_span)
                                    .with_message(format!("previous definition of hook `{}`", previous_span)),
                            )
                            .with_annotation(
                                Annotation::secondary(hooked_property.item.variable().span())
                                    .with_message(format!("property `{}` defined here", item_name)),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    } else {
                        hook_names.push((lowered_name, hook.name.span()));
                    }
                }
            }
        };
    }

    fn process_method(
        &self,
        method: &Method,
        method_name: &str,
        class_like_span: Span,
        class_like_name: &str,
        class_like_fqcn: &str,
        class_like_kind: &str,
        class_like_is_interface: bool,
        context: &mut Context<'_>,
    ) {
        let mut last_static: Option<Span> = None;
        let mut last_final: Option<Span> = None;
        let mut last_abstract: Option<Span> = None;
        let mut last_visibility: Option<Span> = None;
        let mut is_public = true;
        for modifier in method.modifiers.iter() {
            match modifier {
                Modifier::Static(_) => {
                    if let Some(last_static) = last_static {
                        context.report(
                            Issue::error(format!(
                                "duplicate `static` modifier on method `{}::{}`",
                                class_like_name, method_name
                            ))
                            .with_annotation(
                                Annotation::primary(modifier.span()).with_message("duplicate `static` modifier"),
                            )
                            .with_annotation(
                                Annotation::primary(last_static).with_message("previous `static` modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_static = Some(modifier.span());
                }
                Modifier::Final(_) => {
                    if let Some(abstract_modifier) = last_abstract {
                        context.report(
                            Issue::error(format!(
                                "method `{}::{}` cannot be both `final` and `abstract`",
                                class_like_name, method_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()).with_message("`final` modifier"))
                            .with_annotation(Annotation::primary(abstract_modifier).with_message("`abstract` modifier"))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_final) = last_final {
                        context.report(
                            Issue::error(format!(
                                "duplicate `final` modifier on method `{}::{}`",
                                class_like_name, method_name
                            ))
                            .with_annotation(
                                Annotation::primary(modifier.span()).with_message("duplicate `final` modifier"),
                            )
                            .with_annotation(Annotation::primary(last_final).with_message("previous `final` modifier"))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_final = Some(modifier.span());
                }
                Modifier::Abstract(_) => {
                    if let Some(final_modifier) = last_final {
                        context.report(
                            Issue::error(format!(
                                "method `{}::{}` cannot be both `final` and `abstract`",
                                class_like_name, method_name
                            ))
                            .with_annotation(Annotation::primary(modifier.span()).with_message("`abstract` modifier"))
                            .with_annotation(Annotation::primary(final_modifier).with_message("`final` modifier"))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    if let Some(last_abstract) = last_abstract {
                        context.report(
                            Issue::error(format!(
                                "duplicate `abstract` modifier on method `{}::{}`",
                                class_like_name, method_name
                            ))
                            .with_annotation(
                                Annotation::primary(modifier.span()).with_message("duplicate `abstract` modifier"),
                            )
                            .with_annotation(
                                Annotation::primary(last_abstract).with_message("previous `abstract` modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }

                    last_abstract = Some(modifier.span());
                }
                Modifier::Readonly(_) => {
                    context.report(
                        Issue::error("`readonly` modifier is not allowed on methods".to_string())
                            .with_annotation(Annotation::primary(modifier.span()).with_message("`readonly` modifier"))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                    );
                }
                Modifier::Private(_) | Modifier::Protected(_) | Modifier::Public(_) => {
                    if let Some(last_visibility) = last_visibility {
                        context.report(
                            Issue::error(format!(
                                "duplicate visibility modifier on method `{}::{}`",
                                class_like_name, method_name
                            ))
                            .with_annotation(
                                Annotation::primary(modifier.span()).with_message("duplicate visibility modifier"),
                            )
                            .with_annotation(
                                Annotation::primary(last_visibility).with_message("previous visibility modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    } else {
                        if !matches!(modifier, Modifier::Public(_)) {
                            is_public = false;
                        }

                        last_visibility = Some(modifier.span());
                    }
                }
                Modifier::PrivateSet(k) | Modifier::ProtectedSet(k) | Modifier::PublicSet(k) => {
                    context.report(
                        Issue::error(format!(
                            "`{}` modifier is not allowed on methods",
                            context.interner.lookup(&k.value)
                        ))
                        .with_annotation(Annotation::primary(modifier.span()).with_message("`private(set)` modifier"))
                        .with_annotation(
                            Annotation::secondary(method.span())
                                .with_message(format!("method `{}::{}` defined here", class_like_name, method_name,)),
                        )
                        .with_annotation(
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                        ),
                    );
                }
            }
        }

        for (magic_method, parameter_count, must_be_public, must_be_static, can_have_return_type) in
            MAGIC_METHOD_SEMANTICS
        {
            if method_name.eq_ignore_ascii_case(magic_method) {
                if let Some(count) = parameter_count {
                    let mut found_count = 0;
                    let mut found_variadic = false;
                    for param in method.parameters.parameters.iter() {
                        found_count += 1;

                        if param.ellipsis.is_some() {
                            found_variadic = true;
                        }
                    }

                    if found_variadic || found_count.ne(count) {
                        let message = if found_variadic {
                            format!(
                                "magic method `{}::{}` must have exactly {} parameters, found more than {} due to variadic parameter",
                                class_like_name,
                                method_name,
                                count,
                                found_count
                            )
                        } else {
                            format!(
                                "magic method `{}::{}` must have exactly {} parameters, found {}",
                                class_like_name, method_name, count, found_count
                            )
                        };

                        context.report(
                            Issue::error(message)
                                .with_annotation(Annotation::primary(method.parameters.span()))
                                .with_annotation(Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )))
                                .with_annotation(Annotation::secondary(class_like_span).with_message(format!(
                                    "{} `{}` is defined here",
                                    class_like_kind, class_like_fqcn
                                ))),
                        );
                    }
                }

                if *must_be_public && !is_public {
                    context.report(
                        Issue::error(format!("magic method `{}::{}` must be public", class_like_name, method_name))
                            .with_annotation(
                                Annotation::primary(last_visibility.unwrap())
                                    .with_message("non-public visibility modifier"),
                            )
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                    );
                }

                match last_static.as_ref() {
                    Some(span) if !*must_be_static => {
                        context.report(
                            Issue::error(format!(
                                "magic method `{}::{}` cannot be static",
                                class_like_name, method_name
                            ))
                            .with_annotation(Annotation::primary(*span).with_message("`static` modifier"))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }
                    None if *must_be_static => {
                        context.report(
                            Issue::error(format!("magic method `{}::{}` must be static", class_like_name, method_name))
                                .with_annotation(Annotation::primary(method.name.span()))
                                .with_annotation(
                                    Annotation::secondary(class_like_span)
                                        .with_message(format!("{} `{}`", class_like_kind, class_like_fqcn)),
                                )
                                .with_annotation(Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                ))),
                        );
                    }
                    _ => {}
                }

                if !*can_have_return_type {
                    if let Some(hint) = &method.return_type_hint {
                        context.report(
                            Issue::error(format!(
                                "magic method `{}::{}` cannot have a return type hint",
                                class_like_name, method_name
                            ))
                            .with_annotation(Annotation::primary(hint.span()))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "method `{}::{}` defined here",
                                    class_like_name, method_name,
                                )),
                            )
                            .with_annotation(
                                Annotation::secondary(class_like_span)
                                    .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            ),
                        );
                    }
                }
            }
        }

        match &method.body {
            MethodBody::Abstract(method_abstract_body) => {
                if !class_like_is_interface && !method.modifiers.contains_abstract() {
                    context.report(
                        Issue::error(format!(
                            "non-abstract method `{}::{}` must have a concrete body",
                            class_like_name, method_name,
                        ))
                        .with_annotation(Annotation::primary(method_abstract_body.span()))
                        .with_annotations([
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            Annotation::secondary(method.span())
                                .with_message(format!("method `{}::{}` defined here", class_like_name, method_name)),
                        ]),
                    );
                }
            }
            MethodBody::Concrete(body) => {
                if let Some(abstract_modifier) = method.modifiers.get_abstract() {
                    context.report(
                        Issue::error(format!(
                            "method `{}::{}` is abstract and cannot have a concrete body",
                            class_like_name, method_name,
                        ))
                        .with_annotation(Annotation::primary(body.span()))
                        .with_annotations([
                            Annotation::primary(abstract_modifier.span()),
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            Annotation::secondary(method.span())
                                .with_message(format!("method `{}::{}` defined here", class_like_name, method_name)),
                        ]),
                    );
                } else if class_like_is_interface {
                    context.report(
                        Issue::error(format!(
                            "interface method `{}::{}` is implicitly abstract and cannot have a concrete body",
                            class_like_name, method_name,
                        ))
                        .with_annotation(Annotation::primary(body.span()))
                        .with_annotations([
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` is defined here", class_like_kind, class_like_fqcn)),
                            Annotation::secondary(method.span())
                                .with_message(format!("method `{}::{}` defined here", class_like_name, method_name)),
                        ]),
                    );
                }

                let hint = if let Some(return_hint) = &method.return_type_hint {
                    &return_hint.hint
                } else {
                    return;
                };

                let returns = mago_ast_utils::find_returns_in_block(body);

                match &hint {
                    Hint::Void(_) => {
                        for r#return in returns {
                            if let Some(val) = &r#return.value {
                                context.report(
                                    Issue::error(format!(
                                        "method `{}::{}` with return type of `void` must not return a value",
                                        class_like_name, method_name,
                                    ))
                                    .with_annotation(Annotation::primary(val.span()))
                                    .with_annotations([
                                        Annotation::secondary(class_like_span).with_message(format!(
                                            "{} `{}` is defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                        Annotation::secondary(method.span()).with_message(format!(
                                            "method `{}::{}` defined here",
                                            class_like_name, method_name,
                                        )),
                                    ])
                                    .with_help("remove the return type hint, or remove the return value"),
                                );
                            }
                        }
                    }
                    Hint::Never(_) => {
                        for r#return in returns {
                            context.report(
                                Issue::error(format!(
                                    "function `{}::{}` with return type of `never` must not return",
                                    class_like_name, method_name,
                                ))
                                .with_annotation(Annotation::primary(r#return.span()))
                                .with_annotations([
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` is defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                    Annotation::secondary(method.span()).with_message(format!(
                                        "method `{}::{}` defined here",
                                        class_like_name, method_name,
                                    )),
                                ])
                                .with_help("remove the return type hint, or remove the return statement"),
                            );
                        }
                    }
                    _ if !returns_generator(context, body, hint) => {
                        for r#return in returns {
                            if r#return.value.is_none() {
                                context.report(
                                    Issue::error(format!(
                                        "method `{}::{}` with return type must return a value",
                                        class_like_name, method_name,
                                    ))
                                    .with_annotation(Annotation::primary(r#return.span()))
                                    .with_annotations([
                                        Annotation::secondary(class_like_span).with_message(format!(
                                            "{} `{}` is defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                        Annotation::secondary(method.span()).with_message(format!(
                                            "method `{}::{}` defined here",
                                            class_like_name, method_name,
                                        )),
                                    ])
                                    .with_note("did you mean `return null;` instead of `return;`?")
                                    .with_help("add a return value to the statement"),
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        };
    }

    #[inline(always)]
    fn process_members(
        &self,
        members: &Sequence<ClassLikeMember>,
        class_like_span: Span,
        class_like_kind: &str,
        class_like_name: &str,
        class_like_fqcn: &str,
        context: &mut Context<'_>,
    ) {
        let mut method_names: Vec<(Span, StringIdentifier)> = vec![];
        let mut constant_names: Vec<(bool, std::string::String, Span)> = vec![];
        let mut property_names: Vec<(bool, StringIdentifier, Span)> = vec![];

        for member in members.iter() {
            match &member {
                ClassLikeMember::Property(property) => match &property {
                    Property::Plain(plain_property) => {
                        for item in plain_property.items.iter() {
                            let item_name_id = item.variable().name;
                            let item_name = context.interner.lookup(&item_name_id);

                            if let Some((is_promoted, _, span)) =
                                property_names.iter().find(|(_, name, _)| item_name_id.eq(name))
                            {
                                let message = if *is_promoted {
                                    format!(
                                        "property `{}::{}` has already been defined as a promoted property",
                                        class_like_name, item_name
                                    )
                                } else {
                                    format!("property `{}::{}` has already been defined", class_like_name, item_name)
                                };

                                context.report(
                                    Issue::error(message)
                                        .with_annotation(Annotation::primary(item.variable().span()))
                                        .with_annotations([
                                            Annotation::secondary(*span).with_message(format!(
                                                "property `{}::{}` previously defined here",
                                                class_like_name, item_name
                                            )),
                                            Annotation::secondary(class_like_span.span()).with_message(format!(
                                                "{} `{}` defined here",
                                                class_like_kind, class_like_fqcn
                                            )),
                                        ])
                                        .with_help("remove the duplicate property"),
                                );
                            } else {
                                property_names.push((false, item_name_id, item.variable().span()));
                            }
                        }
                    }
                    Property::Hooked(hooked_property) => {
                        let item_variable = hooked_property.item.variable();
                        let item_name_id = item_variable.name;
                        let item_name = context.interner.lookup(&item_name_id);

                        if let Some((is_promoted, _, span)) =
                            property_names.iter().find(|(_, name, _)| item_name_id.eq(name))
                        {
                            let message = if *is_promoted {
                                format!(
                                    "property `{}::{}` has already been defined as a promoted property",
                                    class_like_name, item_name
                                )
                            } else {
                                format!("property `{}::{}` has already been defined", class_like_name, item_name)
                            };

                            context.report(
                                Issue::error(message)
                                    .with_annotation(Annotation::primary(item_variable.span()))
                                    .with_annotations([
                                        Annotation::secondary(*span).with_message(format!(
                                            "property `{}::{}` previously defined here",
                                            class_like_name, item_name
                                        )),
                                        Annotation::secondary(class_like_span.span()).with_message(format!(
                                            "{} `{}` defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                    ])
                                    .with_help("remove the duplicate property"),
                            );
                        } else {
                            property_names.push((false, item_name_id, item_variable.span()));
                        }
                    }
                },
                ClassLikeMember::Method(method) => {
                    let method_name_id = method.name.value;
                    let method_name = context.interner.lookup(&method_name_id);
                    let method_name_lowered_id = context.interner.intern(method_name.to_ascii_lowercase());

                    if let Some((previous, _)) =
                        method_names.iter().find(|(_, previous_name)| method_name_lowered_id.eq(previous_name))
                    {
                        context.report(
                            Issue::error(format!(
                                "{} method `{}::{}` has already been defined",
                                class_like_kind, class_like_name, method_name
                            ))
                            .with_annotation(Annotation::primary(method.name.span()))
                            .with_annotations([
                                Annotation::secondary(*previous).with_message("previous definition"),
                                Annotation::secondary(class_like_span.span())
                                    .with_message(format!("{} `{}` defined here", class_like_kind, class_like_fqcn)),
                            ]),
                        );
                    } else {
                        method_names.push((method.name.span(), method_name_lowered_id));
                    }

                    if method_name.eq_ignore_ascii_case(CONSTRUCTOR_MAGIC_METHOD) {
                        for parameter in method.parameters.parameters.iter() {
                            if parameter.is_promoted_property() {
                                let item_name_id = parameter.variable.name;
                                let item_name = context.interner.lookup(&item_name_id);

                                if let Some((is_promoted, _, span)) =
                                    property_names.iter().find(|(_, name, _)| item_name_id.eq(name))
                                {
                                    let message = if !*is_promoted {
                                        format!(
                                            "promoted property `{}::{}` has already been defined as a property",
                                            class_like_name, item_name
                                        )
                                    } else {
                                        format!(
                                            "promoted property `{}::{}` has already been defined",
                                            class_like_name, item_name
                                        )
                                    };

                                    context.report(
                                        Issue::error(message)
                                            .with_annotation(Annotation::primary(parameter.variable.span()))
                                            .with_annotations([
                                                Annotation::secondary(*span).with_message(format!(
                                                    "property `{}::{}` previously defined here",
                                                    class_like_name, item_name
                                                )),
                                                Annotation::secondary(class_like_span.span()).with_message(format!(
                                                    "{} `{}` defined here",
                                                    class_like_kind, class_like_fqcn
                                                )),
                                            ])
                                            .with_help("remove the duplicate property"),
                                    );
                                } else {
                                    property_names.push((true, item_name_id, parameter.variable.span()));
                                }
                            }
                        }
                    }
                }
                ClassLikeMember::Constant(class_like_constant) => {
                    for item in class_like_constant.items.iter() {
                        let item_name = context.interner.lookup(&item.name.value);

                        if let Some((is_constant, name, span)) = constant_names.iter().find(|t| t.1.eq(&item_name)) {
                            if *is_constant {
                                context.report(
                                    Issue::error(format!(
                                        "{} constant `{}::{}` has already been defined",
                                        class_like_kind, class_like_name, name,
                                    ))
                                    .with_annotation(Annotation::primary(item.name.span()))
                                    .with_annotations([
                                        Annotation::secondary(*span).with_message(format!(
                                            "constant `{}::{}` previously defined here",
                                            class_like_name, name
                                        )),
                                        Annotation::secondary(class_like_span.span()).with_message(format!(
                                            "{} `{}` defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                    ]),
                                );
                            } else {
                                context.report(
                                    Issue::error(format!(
                                        "{} case `{}::{}` and constant `{}::{}` cannot have the same name",
                                        class_like_kind, class_like_name, name, class_like_name, name
                                    ))
                                    .with_annotation(Annotation::primary(item.name.span()))
                                    .with_annotations([
                                        Annotation::secondary(*span)
                                            .with_message(format!("case `{}::{}` defined here", class_like_name, name)),
                                        Annotation::secondary(class_like_span.span()).with_message(format!(
                                            "{} `{}` defined here",
                                            class_like_kind, class_like_fqcn
                                        )),
                                    ]),
                                );
                            }
                        } else {
                            constant_names.push((true, item_name.to_string(), item.name.span()));
                        }
                    }
                }
                ClassLikeMember::EnumCase(enum_case) => {
                    let case_name = context.interner.lookup(&enum_case.item.name().value);

                    if let Some((is_constant, name, span)) = constant_names.iter().find(|t| t.1.eq(&case_name)) {
                        if *is_constant {
                            context.report(
                                Issue::error(format!(
                                    "{} case `{}::{}` and constant `{}::{}` cannot have the same name",
                                    class_like_kind, class_like_name, name, class_like_name, name
                                ))
                                .with_annotation(Annotation::primary(enum_case.item.name().span()))
                                .with_annotations([
                                    Annotation::secondary(*span)
                                        .with_message(format!("constant `{}::{}` defined here", class_like_name, name)),
                                    Annotation::secondary(class_like_span.span()).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ]),
                            );
                        } else {
                            context.report(
                                Issue::error(format!(
                                    "{} case `{}::{}` has already been defined",
                                    class_like_kind, class_like_name, name,
                                ))
                                .with_annotation(Annotation::primary(enum_case.item.name().span()))
                                .with_annotations([
                                    Annotation::secondary(*span).with_message(format!(
                                        "case `{}::{}` previously defined here",
                                        class_like_name, name
                                    )),
                                    Annotation::secondary(class_like_span.span()).with_message(format!(
                                        "{} `{}` defined here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ]),
                            );
                        }

                        continue;
                    } else {
                        constant_names.push((false, case_name.to_string(), enum_case.item.name().span()));
                    }
                }
                _ => {}
            }
        }
    }

    fn process_class_like_constant(
        &self,
        class_like_constant: &ClassLikeConstant,
        class_like_span: Span,
        class_like_kind: &str,
        class_like_name: &str,
        class_like_fqcn: &str,
        context: &mut Context,
    ) {
        let first_item = class_like_constant.first_item();
        let first_item_name = context.interner.lookup(&first_item.name.value);

        let mut last_final: Option<Span> = None;
        let mut last_visibility: Option<Span> = None;
        for modifier in class_like_constant.modifiers.iter() {
            match modifier {
                Modifier::Readonly(k)
                | Modifier::Static(k)
                | Modifier::Abstract(k)
                | Modifier::PrivateSet(k)
                | Modifier::ProtectedSet(k)
                | Modifier::PublicSet(k) => {
                    context.report(
                        Issue::error(format!(
                            "`{}` modifier is not allowed on constants",
                            context.interner.lookup(&k.value),
                        ))
                        .with_annotation(Annotation::primary(modifier.span()))
                        .with_annotations([
                            Annotation::secondary(first_item.span()).with_message(format!(
                                "{} constant `{}::{}` is declared here",
                                class_like_kind, class_like_name, first_item_name
                            )),
                            Annotation::secondary(class_like_span)
                                .with_message(format!("{} `{}` is declared here", class_like_kind, class_like_fqcn)),
                        ]),
                    );
                }
                Modifier::Final(_) => {
                    if let Some(last_final) = last_final {
                        context.report(
                            Issue::error("duplicate `final` modifier on constant")
                                .with_annotation(Annotation::primary(modifier.span()))
                                .with_annotations([
                                    Annotation::secondary(last_final).with_message("previous `final` modifier"),
                                    Annotation::secondary(first_item.span()).with_message(format!(
                                        "{} constant `{}::{}` is declared here",
                                        class_like_kind, class_like_name, first_item_name
                                    )),
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` is declared here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ]),
                        );
                    }

                    last_final = Some(modifier.span());
                }
                Modifier::Private(_) | Modifier::Protected(_) | Modifier::Public(_) => {
                    if let Some(last_visibility) = last_visibility {
                        context.report(
                            Issue::error("duplicate visibility modifier on constant")
                                .with_annotation(Annotation::primary(modifier.span()))
                                .with_annotations([
                                    Annotation::secondary(last_visibility).with_message("previous visibility modifier"),
                                    Annotation::secondary(first_item.span()).with_message(format!(
                                        "{} constant `{}::{}` is declared here",
                                        class_like_kind, class_like_name, first_item_name
                                    )),
                                    Annotation::secondary(class_like_span).with_message(format!(
                                        "{} `{}` is declared here",
                                        class_like_kind, class_like_fqcn
                                    )),
                                ]),
                        );
                    }

                    last_visibility = Some(modifier.span());
                }
            }
        }

        for item in class_like_constant.items.iter() {
            let item_name = context.interner.lookup(&item.name.value);

            if !item.value.is_constant(false) {
                context.report(
                    Issue::error(format!(
                        "constant `{}::{}` value contains a non-constant expression",
                        class_like_name, item_name
                    ))
                    .with_annotation(Annotation::primary(item.value.span()))
                    .with_annotations([
                        Annotation::secondary(item.name.span()).with_message(format!(
                            "{} constant `{}::{}` is declared here",
                            class_like_kind, class_like_name, item_name
                        )),
                        Annotation::secondary(class_like_span)
                            .with_message(format!("{} `{}` is declared here", class_like_kind, class_like_fqcn)),
                    ]),
                );
            }
        }
    }

    fn process_promoted_properties_outside_constructor(
        &self,
        parameter_list: &FunctionLikeParameterList,
        context: &mut Context<'_>,
    ) {
        for parameter in parameter_list.parameters.iter() {
            if parameter.is_promoted_property() {
                context.report(
                    Issue::error("promoted properties are not allowed outside of constructors")
                        .with_annotation(Annotation::primary(parameter.span())),
                );
            }
        }
    }
}

impl Walker<Context<'_>> for SemanticsWalker {
    fn walk_in_statement(&self, statement: &Statement, context: &mut Context<'_>) {
        context.push_ancestor(statement.span());
    }

    fn walk_in_expression(&self, expression: &Expression, context: &mut Context<'_>) {
        context.push_ancestor(expression.span());
    }

    fn walk_out_statement(&self, _statement: &Statement, context: &mut Context<'_>) {
        context.pop_ancestor();
    }

    fn walk_out_expression(&self, _expression: &Expression, context: &mut Context<'_>) {
        context.pop_ancestor();
    }

    fn walk_in_program(&self, program: &Program, context: &mut Context<'_>) {
        let mut index = 0;
        let mut before = vec![];

        for statement in program.statements.iter() {
            if !matches!(
                statement,
                Statement::Declare(_)
                    | Statement::OpeningTag(_)
                    | Statement::Inline(Inline { kind: InlineKind::Shebang, .. })
            ) {
                index += 1;

                before.push(statement.span());

                continue;
            }

            if index == 0 {
                continue;
            }

            if let Statement::Declare(declare) = statement {
                for item in declare.items.iter() {
                    let name = context.interner.lookup(&item.name.value);

                    if name.eq_ignore_ascii_case(STRICT_TYPES_DECLARE_DIRECTIVE) {
                        context.report(
                            Issue::error("strict type declaration must be the first statement in the file")
                                .with_annotation(Annotation::primary(declare.span()))
                                .with_annotations(before.iter().map(|span| {
                                    Annotation::secondary(*span)
                                        .with_message("this statement should come after the strict type declaration")
                                })),
                        );
                    }
                }
            }
        }

        let mut index = 0;
        let mut before = vec![];

        for statement in program.statements.iter() {
            if !matches!(
                statement,
                Statement::Declare(_)
                    | Statement::Namespace(_)
                    | Statement::OpeningTag(_)
                    | Statement::Inline(Inline { kind: InlineKind::Shebang, .. })
            ) {
                index += 1;

                before.push(statement.span());

                continue;
            }

            if index == 0 {
                continue;
            }

            if let Statement::Namespace(namespace) = statement {
                context.report(
                    Issue::error("namespace must be the first statement in the file")
                        .with_annotation(Annotation::primary(namespace.span()))
                        .with_annotations(before.iter().map(|span| {
                            Annotation::secondary(*span)
                                .with_message("this statement should come after the namespace statement")
                        })),
                );
            }
        }

        let program = context.program();
        let namespaces = program.filter_map(|node| if let Node::Namespace(ns) = node { Some(*ns) } else { None });

        let mut last_unbraced = None;
        let mut last_braced = None;

        for namespace in namespaces {
            let mut namespace_span = namespace.namespace.span();
            if let Some(name) = &namespace.name {
                namespace_span = namespace_span.join(name.span());
            }

            match &namespace.body {
                NamespaceBody::Implicit(body) => {
                    if namespace.name.is_none() {
                        context.report(
                            Issue::error("unbraced namespace must be named")
                                .with_annotation(Annotation::primary(namespace.span().join(body.terminator.span())))
                                .with_annotation(Annotation::secondary(body.span())),
                        );
                    }

                    last_unbraced = Some((namespace_span, body.span()));

                    if let Some((last_namespace_span, last_body_span)) = last_braced {
                        context.report(
                            Issue::error(
                                "cannot mix unbraced namespace declarations with braced namespace declarations",
                            )
                            .with_annotation(Annotation::primary(namespace_span))
                            .with_annotations([
                                Annotation::primary(last_namespace_span),
                                Annotation::secondary(last_body_span).with_message("braced namespace declaration"),
                                Annotation::secondary(body.span()).with_message("unbraced namespace declaration"),
                            ]),
                        );
                    }
                }
                NamespaceBody::BraceDelimited(body) => {
                    last_braced = Some((namespace_span, body.span()));

                    if let Some((last_namespace_span, last_body_span)) = last_unbraced {
                        context.report(
                            Issue::error(
                                "cannot mix braced namespace declarations with unbraced namespace declarations",
                            )
                            .with_annotation(Annotation::primary(namespace_span))
                            .with_annotations([
                                Annotation::primary(last_namespace_span),
                                Annotation::secondary(last_body_span).with_message("unbraced namespace declaration"),
                                Annotation::secondary(body.span()).with_message("braced namespace declaration"),
                            ]),
                        );
                    }
                }
            }
        }
    }

    fn walk_in_short_opening_tag(&self, short_opening_tag: &ShortOpeningTag, context: &mut Context<'_>) {
        context.report(
            Issue::error("short opening tag `<?` is no longer supported")
                .with_annotation(Annotation::primary(short_opening_tag.span()))
                .with_help("use the full opening tag `<?php` instead."),
        );
    }

    fn walk_in_declare(&self, declare: &Declare, context: &mut Context<'_>) {
        for item in declare.items.iter() {
            let name = context.interner.lookup(&item.name.value);

            match name.to_ascii_lowercase().as_str() {
                STRICT_TYPES_DECLARE_DIRECTIVE => {
                    let value = match &item.value {
                        Expression::Literal(Literal::Integer(LiteralInteger { value, .. })) => *value,
                        _ => None,
                    };

                    if !matches!(value, Some(0) | Some(1)) {
                        context.report(
                            Issue::error(format!("`{}` declare directive must be set to either `0` or `1`", name))
                                .with_annotation(Annotation::primary(item.value.span())),
                        );
                    }

                    if context.get_ancestors_len() > 2 {
                        // get the span of the parent, and label it.
                        let parent = context.get_ancestor(context.get_ancestors_len() - 2);

                        context.report(
                            Issue::error("strict types declaration must be at the top level")
                                .with_annotation(Annotation::primary(declare.span()))
                                .with_annotation(
                                    Annotation::secondary(parent)
                                        .with_message("this statement should come after the strict type declaration"),
                                ),
                        );
                    }
                }
                TICKS_DECLARE_DIRECTIVE => {
                    if !matches!(item.value, Expression::Literal(Literal::Integer(_))) {
                        context.report(
                            Issue::error(format!("`{}` declare directive must be set to a literal integer", name))
                                .with_annotation(Annotation::primary(item.value.span())),
                        );
                    }
                }
                ENCODING_DECLARE_DIRECTIVE => {
                    if !matches!(item.value, Expression::Literal(Literal::String(_))) {
                        context.report(
                            Issue::error(format!("`{}` declare directive must be set to a literal integer", name))
                                .with_annotation(Annotation::primary(item.value.span())),
                        );
                    }
                }
                _ => {
                    context.report(
                        Issue::error(format!(
                            "`{}` is not a supported declare directive, supported directives are: `{}`",
                            name,
                            DECLARE_DIRECTIVES.join("`, `")
                        ))
                        .with_annotation(Annotation::primary(item.name.span())),
                    );
                }
            }
        }
    }

    fn walk_in_namespace(&self, namespace: &Namespace, context: &mut Context<'_>) {
        if context.get_ancestors_len() > 2 {
            // get the span of the parent, and label it.
            let parent = context.get_ancestor(context.get_ancestors_len() - 2);

            context.report(
                Issue::error("namespace declaration must be at the top level")
                    .with_annotation(Annotation::primary(namespace.span()))
                    .with_annotation(
                        Annotation::secondary(parent)
                            .with_message("this statement should come after the namespace declaration"),
                    ),
            );
        }
    }

    fn walk_in_hint(&self, hint: &Hint, context: &mut Context<'_>) {
        match hint {
            Hint::Parenthesized(parenthesized_hint) => {
                if !parenthesized_hint.hint.is_parenthesizable() {
                    let val = context.lookup_hint(&parenthesized_hint.hint);

                    context.report(
                        Issue::error(format!("type `{}` cannot be parenthesized", val))
                            .with_annotation(Annotation::primary(parenthesized_hint.hint.span()))
                            .with_annotation(Annotation::secondary(parenthesized_hint.span())),
                    );
                }
            }
            Hint::Nullable(nullable_hint) => {
                if nullable_hint.hint.is_standalone() || nullable_hint.hint.is_complex() {
                    let val = context.lookup_hint(&nullable_hint.hint);

                    context.report(
                        Issue::error(format!("type `{}` cannot be nullable", val))
                            .with_annotation(Annotation::primary(nullable_hint.hint.span()))
                            .with_annotation(Annotation::secondary(nullable_hint.span())),
                    );
                }
            }
            Hint::Union(union_hint) => {
                if !union_hint.left.is_unionable() {
                    let val = context.lookup_hint(&union_hint.left);

                    context.report(
                        Issue::error(format!("type `{}` cannot be part of a union", val))
                            .with_annotation(Annotation::primary(union_hint.left.span()))
                            .with_annotation(Annotation::secondary(union_hint.pipe)),
                    );
                }

                if !union_hint.right.is_unionable() {
                    let val = context.lookup_hint(&union_hint.right);

                    context.report(
                        Issue::error(format!("type `{}` cannot be part of a union", val))
                            .with_annotation(Annotation::primary(union_hint.right.span()))
                            .with_annotation(Annotation::secondary(union_hint.pipe)),
                    );
                }
            }
            Hint::Intersection(intersection_hint) => {
                if !intersection_hint.left.is_intersectable() {
                    let val = context.lookup_hint(&intersection_hint.left);
                    context.report(
                        Issue::error(format!("type `{}` cannot be part of an intersection", val))
                            .with_annotation(Annotation::primary(intersection_hint.left.span()))
                            .with_annotation(Annotation::secondary(intersection_hint.ampersand)),
                    );
                }

                if !intersection_hint.right.is_intersectable() {
                    let val = context.lookup_hint(&intersection_hint.right);

                    context.report(
                        Issue::error(format!("type `{}` cannot be part of an intersection", val))
                            .with_annotation(Annotation::primary(intersection_hint.right.span()))
                            .with_annotation(Annotation::secondary(intersection_hint.ampersand)),
                    );
                }
            }
            _ => {}
        }
    }

    fn walk_in_try(&self, r#try: &Try, context: &mut Context<'_>) {
        if r#try.catch_clauses.is_empty() && r#try.finally_clause.is_none() {
            context.report(
                Issue::error("cannot use `try` without a `catch` or `finally`")
                    .with_annotations([
                        Annotation::primary(r#try.r#try.span()),
                        Annotation::secondary(r#try.block.span()),
                    ])
                    .with_note("each `try` must have at least one corresponding `catch` or `finally` clause.")
                    .with_help("add either a `catch` or `finally` clause")
                    .with_link("https://www.php.net/manual/en/language.exceptions.php"),
            );
        }
    }

    fn walk_in_property_hook(&self, property_hook: &PropertyHook, context: &mut Context<'_>) {
        if let Some(parameter_list) = &property_hook.parameters {
            self.process_promoted_properties_outside_constructor(parameter_list, context);
        }
    }

    fn walk_in_method(&self, method: &Method, context: &mut Context<'_>) {
        let name = context.interner.lookup(&method.name.value);
        if name != "__construct" {
            self.process_promoted_properties_outside_constructor(&method.parameters, context);

            return;
        }

        if let Some(abstract_modifier) = method.modifiers.get_abstract() {
            for parameter in method.parameters.parameters.iter() {
                if parameter.is_promoted_property() {
                    context.report(
                        Issue::error("promoted properties are not allowed in abstract constructors")
                            .with_annotation(Annotation::primary(parameter.span()))
                            .with_annotation(
                                Annotation::secondary(abstract_modifier.span())
                                    .with_message("this constructor is abstract"),
                            ),
                    );
                }
            }
        }
    }

    fn walk_in_class(&self, class: &Class, context: &mut Context<'_>) {
        let class_name = context.interner.lookup(&class.name.value);
        let class_fqcn = context.lookup_name(&class.name.span.start);

        if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(class_name))
            || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED
                .iter()
                .any(|keyword| keyword.eq_ignore_ascii_case(class_name))
        {
            context.report(
                Issue::error(format!("class `{}` name cannot be a reserved keyword", class_name))
                    .with_annotation(Annotation::primary(class.name.span()))
                    .with_annotation(
                        Annotation::secondary(class.span())
                            .with_message(format!("class `{}` defined here", class_fqcn)),
                    ),
            );
        }

        let mut last_final = None;
        let mut last_abstract = None;
        let mut last_readonly = None;

        for modifier in class.modifiers.iter() {
            match &modifier {
                Modifier::Static(_) | Modifier::PrivateSet(_) | Modifier::ProtectedSet(_) | Modifier::PublicSet(_) => {
                    context.report(
                        Issue::error(format!(
                            "class `{}` cannot have `{}` modifier",
                            class_name,
                            modifier.as_str(context.interner)
                        ))
                        .with_annotation(Annotation::primary(modifier.span()))
                        .with_annotation(
                            Annotation::secondary(class.span())
                                .with_message(format!("class `{}` defined here", class_fqcn)),
                        )
                        .with_help("remove the `static` modifier"),
                    );
                }
                Modifier::Public(keyword) | Modifier::Protected(keyword) | Modifier::Private(keyword) => {
                    let visibility_name = context.interner.lookup(&keyword.value);

                    context.report(
                        Issue::error(format!(
                            "class `{}` cannot have `{}` visibility modifier",
                            class_name, visibility_name
                        ))
                        .with_annotation(Annotation::primary(keyword.span()))
                        .with_annotation(
                            Annotation::secondary(class.span())
                                .with_message(format!("class `{}` defined here", class_fqcn)),
                        )
                        .with_help(format!("remove the `{}` modifier", visibility_name)),
                    );
                }
                Modifier::Final(keyword) => {
                    if let Some(span) = last_abstract {
                        context.report(
                            Issue::error(format!("abstract class `{}` cannot have `final` modifier", class_name))
                                .with_annotation(Annotation::primary(keyword.span()))
                                .with_annotations([
                                    Annotation::secondary(span).with_message("previous `abstract` modifier"),
                                    Annotation::secondary(class.span())
                                        .with_message(format!("class `{}` defined here", class_fqcn)),
                                ])
                                .with_help("remove the `final` modifier"),
                        );
                    }

                    if let Some(span) = last_final {
                        context.report(
                            Issue::error(format!("class `{}` cannot have multiple `final` modifiers", class_name))
                                .with_annotation(Annotation::primary(keyword.span()))
                                .with_annotations([
                                    Annotation::secondary(span).with_message("previous `final` modifier"),
                                    Annotation::secondary(class.span())
                                        .with_message(format!("class `{}` defined here", class_fqcn)),
                                ])
                                .with_help("remove the duplicate `final` modifier"),
                        );
                    }

                    last_final = Some(keyword.span);
                }
                Modifier::Abstract(keyword) => {
                    if let Some(span) = last_final {
                        context.report(
                            Issue::error(format!("final class `{}` cannot have `abstract` modifier", class_name))
                                .with_annotation(Annotation::primary(keyword.span()))
                                .with_annotations([
                                    Annotation::secondary(span).with_message("previous `final` modifier"),
                                    Annotation::secondary(class.span())
                                        .with_message(format!("class `{}` defined here", class_fqcn)),
                                ])
                                .with_help("remove the `abstract` modifier"),
                        );
                    }

                    if let Some(span) = last_abstract {
                        context.report(
                            Issue::error(format!("class `{}` cannot have multiple `abstract` modifiers", class_name))
                                .with_annotation(Annotation::primary(keyword.span()))
                                .with_annotations([
                                    Annotation::secondary(span).with_message("previous `abstract` modifier"),
                                    Annotation::secondary(class.span())
                                        .with_message(format!("class `{}` defined here", class_fqcn)),
                                ])
                                .with_help("remove the duplicate `abstract` modifier"),
                        );
                    }

                    last_abstract = Some(keyword.span);
                }
                Modifier::Readonly(keyword) => {
                    if let Some(span) = last_readonly {
                        context.report(
                            Issue::error(format!("class `{}` cannot have multiple `readonly` modifiers", class_name))
                                .with_annotation(Annotation::primary(keyword.span()))
                                .with_annotations([
                                    Annotation::secondary(span).with_message("previous `readonly` modifier"),
                                    Annotation::secondary(class.span())
                                        .with_message(format!("class `{}` defined here", class_fqcn)),
                                ])
                                .with_help("remove the duplicate `readonly` modifier"),
                        );
                    }

                    last_readonly = Some(keyword.span);
                }
            }
        }

        if let Some(extends) = &class.extends {
            self.process_extends(extends, class.span(), "class", class_name, class_fqcn, true, context);
        }

        if let Some(implements) = &class.implements {
            self.process_implements(implements, class.span(), "class", class_name, class_fqcn, true, context);
        }

        self.process_members(&class.members, class.span(), "class", class_name, class_fqcn, context);

        for memeber in class.members.iter() {
            match &memeber {
                ClassLikeMember::EnumCase(case) => {
                    context.report(
                        Issue::error(format!("class `{}` cannot contain enum cases", class_name))
                            .with_annotation(Annotation::primary(case.span()))
                            .with_annotation(
                                Annotation::secondary(class.span())
                                    .with_message(format!("class `{}` defined here", class_fqcn)),
                            ),
                    );
                }
                ClassLikeMember::Method(method) => {
                    let method_name = context.interner.lookup(&method.name.value);

                    if !class.modifiers.contains_abstract() && method.modifiers.contains_abstract() {
                        context.report(
                            Issue::error(format!(
                                "class `{}` has an abstract method `{}`, and therefore must be declared abstract",
                                class_name, method_name
                            ))
                            .with_annotation(Annotation::primary(class.name.span()))
                            .with_annotation(
                                Annotation::secondary(method.span()).with_message(format!(
                                    "abstract method `{}::{}` is defined here",
                                    class_name, method_name
                                )),
                            ),
                        );
                    }

                    self.process_method(
                        method,
                        method_name,
                        class.span(),
                        class_name,
                        class_fqcn,
                        "class",
                        false,
                        context,
                    );
                }
                ClassLikeMember::Property(property) => {
                    self.process_property(property, class.span(), "class", class_name, class_fqcn, false, context);
                }
                ClassLikeMember::Constant(constant) => {
                    self.process_class_like_constant(constant, class.span(), "class", class_name, class_fqcn, context);
                }
                _ => {}
            }
        }
    }

    fn walk_in_interface(&self, interface: &Interface, context: &mut Context<'_>) {
        let interface_name = context.interner.lookup(&interface.name.value);
        let interface_fqcn = context.lookup_name(&interface.name.span.start);

        if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(interface_name))
            || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED
                .iter()
                .any(|keyword| keyword.eq_ignore_ascii_case(interface_name))
        {
            context.report(
                Issue::error(format!("interface `{}` name cannot be a reserved keyword", interface_name))
                    .with_annotation(Annotation::primary(interface.name.span()))
                    .with_annotation(
                        Annotation::secondary(interface.span())
                            .with_message(format!("interface `{}` defined here", interface_fqcn)),
                    ),
            );
        }

        if let Some(extends) = &interface.extends {
            self.process_extends(
                extends,
                interface.span(),
                "interface",
                interface_name,
                interface_fqcn,
                false,
                context,
            );
        }

        self.process_members(
            &interface.members,
            interface.span(),
            "interface",
            interface_name,
            interface_fqcn,
            context,
        );

        for memeber in interface.members.iter() {
            match &memeber {
                ClassLikeMember::TraitUse(trait_use) => {
                    context.report(
                        Issue::error(format!("interface `{}` cannot use traits", interface_name))
                            .with_annotation(Annotation::primary(trait_use.span()))
                            .with_annotation(
                                Annotation::secondary(interface.span())
                                    .with_message(format!("interface `{}` defined here", interface_fqcn)),
                            ),
                    );
                }
                ClassLikeMember::EnumCase(case) => {
                    context.report(
                        Issue::error(format!("interface `{}` cannot contain enum cases", interface_name))
                            .with_annotation(Annotation::primary(case.span()))
                            .with_annotation(
                                Annotation::secondary(interface.span())
                                    .with_message(format!("interface `{}` defined here", interface_fqcn)),
                            ),
                    );
                }
                ClassLikeMember::Method(method) => {
                    let method_name_id = method.name.value;
                    let method_name = context.interner.lookup(&method_name_id);

                    let mut visibilities = vec![];
                    for modifier in method.modifiers.iter() {
                        if matches!(modifier, Modifier::Private(_) | Modifier::Protected(_)) {
                            visibilities.push(modifier);
                        }
                    }

                    for visibility in visibilities {
                        let visibility_name = visibility.as_str(context.interner);

                        context.report(
                            Issue::error(format!(
                                "interface method `{}::{}` cannot have `{}` modifier",
                                interface_name, method_name, visibility_name
                            ))
                            .with_annotation(Annotation::primary(visibility.span()))
                            .with_annotation(
                                Annotation::secondary(interface.span())
                                    .with_message(format!("`{}` defined here", interface_fqcn)),
                            )
                            .with_help(format!("remove the `{}` modifier", visibility_name)),
                        );
                    }

                    if let MethodBody::Concrete(body) = &method.body {
                        context.report(
                            Issue::error(format!(
                                "interface method `{}::{}` cannot have a body",
                                interface_name, method_name
                            ))
                            .with_annotations([
                                Annotation::primary(body.span()),
                                Annotation::primary(method.name.span()),
                                Annotation::secondary(interface.span())
                                    .with_message(format!("`interface {}` defined here", interface_fqcn)),
                            ])
                            .with_help("replace the method body with a `;`"),
                        );
                    }

                    if let Some(abstract_modifier) = method.modifiers.get_abstract() {
                        context.report(
                            Issue::error(format!(
                                "interface method `{}::{}` must not be abstract",
                                interface_name, method_name
                            ))
                            .with_annotation(Annotation::primary(abstract_modifier.span()))
                            .with_annotations([
                                Annotation::secondary(interface.span())
                                    .with_message(format!("interface `{}` is defined here", interface_fqcn)),
                                Annotation::secondary(method.span())
                                    .with_message(format!("method `{}::{}` defined here", interface_name, method_name)),
                            ]),
                        );
                    }

                    self.process_method(
                        method,
                        method_name,
                        interface.span(),
                        interface_name,
                        interface_fqcn,
                        "interface",
                        true,
                        context,
                    );
                }
                ClassLikeMember::Property(property) => {
                    match &property {
                        Property::Plain(plain_property) => {
                            context.report(
                                Issue::error(format!(
                                    "interface `{}` cannot have non-hooked properties",
                                    interface_name
                                ))
                                .with_annotation(Annotation::primary(plain_property.span()))
                                .with_annotation(
                                    Annotation::secondary(interface.span())
                                        .with_message(format!("`{}` defined here", interface_fqcn)),
                                ),
                            );
                        }
                        Property::Hooked(hooked_property) => {
                            let property_name_id = hooked_property.item.variable().name;
                            let property_name = context.interner.lookup(&property_name_id);

                            let mut found_public = false;
                            let mut non_public_read_visibilities = vec![];
                            let mut write_visibilities = vec![];
                            for modifier in hooked_property.modifiers.iter() {
                                if matches!(modifier, Modifier::Public(_)) {
                                    found_public = true;
                                }

                                if matches!(modifier, Modifier::Private(_) | Modifier::Protected(_)) {
                                    non_public_read_visibilities.push(modifier);
                                }

                                if matches!(modifier, Modifier::PrivateSet(_)) {
                                    write_visibilities.push(modifier);
                                }
                            }

                            for visibility in write_visibilities {
                                let visibility_name = visibility.as_str(context.interner);

                                context.report(
                                    Issue::error(format!(
                                        "interface virtual property `{}::{}` must not specify asymmetric visibility",
                                        interface_name, property_name,
                                    ))
                                    .with_annotation(Annotation::primary(visibility.span()))
                                    .with_annotation(
                                        Annotation::secondary(interface.span())
                                            .with_message(format!("`{}` defined here", interface_fqcn)),
                                    )
                                    .with_help(format!("remove the `{}` modifier", visibility_name)),
                                );
                            }

                            for visibility in non_public_read_visibilities {
                                let visibility_name = visibility.as_str(context.interner);
                                context.report(
                                    Issue::error(format!(
                                        "interface virtual property `{}::{}` cannot have `{}` modifier",
                                        interface_name, property_name, visibility_name,
                                    ))
                                    .with_annotation(Annotation::primary(visibility.span()))
                                    .with_annotation(
                                        Annotation::secondary(interface.span())
                                            .with_message(format!("`{}` defined here", interface_fqcn)),
                                    )
                                    .with_help(format!("remove the `{}` modifier", visibility_name)),
                                );
                            }

                            if !found_public {
                                context.report(
                                    Issue::error(format!(
                                        "interface property `{}::{}` must be declared public",
                                        interface_name, property_name
                                    ))
                                    .with_annotation(Annotation::primary(hooked_property.item.variable().span()))
                                    .with_annotation(
                                        Annotation::secondary(interface.span())
                                            .with_message(format!("`{}` defined here", interface_fqcn)),
                                    )
                                    .with_help("add the `public` visibility modifier to the property"),
                                );
                            }

                            if let Some(abstract_modifier) = hooked_property.modifiers.get_abstract() {
                                context.report(
                                    Issue::error(format!(
                                        "interface property `{}::{}` cannot be abstract",
                                        interface_name, property_name
                                    ))
                                    .with_annotation(Annotation::primary(abstract_modifier.span()))
                                    .with_annotations([
                                        Annotation::secondary(hooked_property.item.variable().span()),
                                        Annotation::secondary(interface.span()).with_message(format!("`{}` defined here", interface_fqcn)),
                                    ])
                                    .with_note(
                                        "all interface properties are implicitly abstract, and cannot be explicitly abstract.",
                                    ),
                                );
                            }

                            if let PropertyItem::Concrete(item) = &hooked_property.item {
                                context.report(
                                    Issue::error(format!(
                                        "interface hooked property `{}::{}` cannot have a default value",
                                        interface_name, property_name
                                    ))
                                    .with_annotation(Annotation::primary(item.span()))
                                    .with_annotation(
                                        Annotation::secondary(interface.span())
                                            .with_message(format!("`{}` defined here", interface_fqcn)),
                                    )
                                    .with_note(
                                        "interface properties are virtual properties which cannot contain a default value",
                                    ),
                                );
                            }

                            for hook in hooked_property.hooks.hooks.iter() {
                                if let PropertyHookBody::Concrete(property_hook_concrete_body) = &hook.body {
                                    context.report(
                                        Issue::error(format!(
                                            "interface hooked property `{}::{}` must be abstract",
                                            interface_name, property_name
                                        ))
                                        .with_annotation(Annotation::primary(property_hook_concrete_body.span()))
                                        .with_annotations([
                                            Annotation::primary(hooked_property.item.variable().span()),
                                            Annotation::secondary(interface.span())
                                                .with_message(format!("`{}` defined here", interface_fqcn)),
                                        ])
                                        .with_note("abstract hooked properties do not contain a body"),
                                    );
                                }
                            }
                        }
                    };

                    self.process_property(
                        property,
                        interface.span(),
                        "interface",
                        interface_name,
                        interface_fqcn,
                        true,
                        context,
                    );
                }
                ClassLikeMember::Constant(class_like_constant) => {
                    let mut non_public_read_visibility = vec![];
                    for modifier in class_like_constant.modifiers.iter() {
                        if matches!(modifier, Modifier::Private(_) | Modifier::Protected(_)) {
                            non_public_read_visibility.push(modifier);
                        }
                    }

                    for visibility in non_public_read_visibility.iter() {
                        let visibility_name = visibility.as_str(context.interner);

                        context.report(
                            Issue::error(format!(
                                "interface constant cannot have `{}` visibility modifier",
                                visibility_name,
                            ))
                            .with_annotation(Annotation::primary(visibility.span()))
                            .with_annotation(
                                Annotation::secondary(interface.span())
                                    .with_message(format!("`{}` defined here", interface_fqcn)),
                            )
                            .with_help(format!("remove the `{}` modifier", visibility_name)),
                        );
                    }

                    self.process_class_like_constant(
                        class_like_constant,
                        interface.span(),
                        "interface",
                        interface_name,
                        interface_fqcn,
                        context,
                    );
                }
            }
        }
    }

    fn walk_in_trait(&self, r#trait: &Trait, context: &mut Context<'_>) {
        let class_like_name = context.interner.lookup(&r#trait.name.value);
        let class_like_fqcn = context.lookup_name(&r#trait.name.span.start);

        if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(class_like_name))
            || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED
                .iter()
                .any(|keyword| keyword.eq_ignore_ascii_case(class_like_name))
        {
            context.report(
                Issue::error(format!("trait `{}` name cannot be a reserved keyword", class_like_name))
                    .with_annotation(Annotation::primary(r#trait.name.span()))
                    .with_annotation(
                        Annotation::secondary(r#trait.span())
                            .with_message(format!("trait `{}` defined here", class_like_fqcn)),
                    ),
            );
        }

        self.process_members(&r#trait.members, r#trait.span(), class_like_name, class_like_fqcn, "trait", context);

        for member in r#trait.members.iter() {
            match &member {
                ClassLikeMember::EnumCase(case) => {
                    context.report(
                        Issue::error(format!("trait `{}` cannot contain enum cases", class_like_name))
                            .with_annotation(Annotation::primary(case.span()))
                            .with_annotation(
                                Annotation::secondary(r#trait.span())
                                    .with_message(format!("trait `{}` defined here", class_like_fqcn)),
                            ),
                    );
                }
                ClassLikeMember::Method(method) => {
                    let method_name = context.interner.lookup(&method.name.value);

                    self.process_method(
                        method,
                        method_name,
                        r#trait.span(),
                        class_like_name,
                        class_like_fqcn,
                        "trait",
                        false,
                        context,
                    );
                }
                ClassLikeMember::Property(property) => {
                    self.process_property(
                        property,
                        r#trait.span(),
                        "trait",
                        class_like_name,
                        class_like_fqcn,
                        false,
                        context,
                    );
                }
                ClassLikeMember::Constant(class_like_constant) => {
                    self.process_class_like_constant(
                        class_like_constant,
                        r#trait.span(),
                        "trait",
                        class_like_name,
                        class_like_fqcn,
                        context,
                    );
                }
                _ => {}
            }
        }
    }

    fn walk_in_enum(&self, r#enum: &Enum, context: &mut Context<'_>) {
        let enum_name = context.interner.lookup(&r#enum.name.value);
        let enum_fqcn = context.lookup_name(&r#enum.name.span.start);
        let enum_is_backed = r#enum.backing_type_hint.is_some();

        if RESERVED_KEYWORDS.iter().any(|keyword| keyword.eq_ignore_ascii_case(enum_name))
            || SOFT_RESERVED_KEYWORDS_MINUS_SYMBOL_ALLOWED.iter().any(|keyword| keyword.eq_ignore_ascii_case(enum_name))
        {
            context.report(
                Issue::error(format!("enum `{}` name cannot be a reserved keyword", enum_name))
                    .with_annotation(Annotation::primary(r#enum.name.span()))
                    .with_annotation(
                        Annotation::secondary(r#enum.span()).with_message(format!("enum `{}` defined here", enum_fqcn)),
                    ),
            );
        }

        if let Some(EnumBackingTypeHint { hint, .. }) = &r#enum.backing_type_hint {
            if !matches!(hint, Hint::String(_) | Hint::Integer(_)) {
                let key = context.lookup_hint(hint);

                context.report(
                    Issue::error(format!(
                        "enum `{}` backing type must be either `string` or `int`, found `{}`.",
                        enum_name, key
                    ))
                    .with_annotation(Annotation::primary(hint.span()))
                    .with_annotation(
                        Annotation::secondary(r#enum.name.span())
                            .with_message(format!("enum `{}` defined here", enum_fqcn)),
                    ),
                );
            }
        }

        if let Some(implements) = &r#enum.implements {
            self.process_implements(implements, r#enum.span(), "enum", enum_name, enum_fqcn, true, context);
        }

        self.process_members(&r#enum.members, r#enum.span(), enum_name, enum_fqcn, "enum", context);

        for memeber in r#enum.members.iter() {
            match &memeber {
                ClassLikeMember::EnumCase(case) => {
                    let item_name_id = case.item.name().value;
                    let item_name = context.interner.lookup(&item_name_id);

                    match &case.item {
                        EnumCaseItem::Unit(_) => {
                            if enum_is_backed {
                                context.report(
                                    Issue::error(format!(
                                        "case `{}` of backed enum `{}` must have a value",
                                        item_name, enum_name
                                    ))
                                    .with_annotation(Annotation::primary(case.span()))
                                    .with_annotation(
                                        Annotation::secondary(r#enum.span())
                                            .with_message(format!("enum `{}` defined here", enum_fqcn)),
                                    ),
                                );
                            }
                        }
                        EnumCaseItem::Backed(item) => {
                            if !enum_is_backed {
                                context.report(
                                    Issue::error(format!(
                                        "case `{}` of unbacked enum `{}` must not have a value",
                                        item_name, enum_name
                                    ))
                                    .with_annotation(Annotation::primary(item.equals.span().join(item.value.span())))
                                    .with_annotations([
                                        Annotation::secondary(item.name.span())
                                            .with_message(format!("case `{}::{}` defined here", enum_name, item_name)),
                                        Annotation::secondary(r#enum.span())
                                            .with_message(format!("enum `{}` defined here", enum_fqcn)),
                                    ]),
                                );
                            }
                        }
                    }
                }
                ClassLikeMember::Method(method) => {
                    let method_name_id = method.name.value;
                    let method_name = context.interner.lookup(&method_name_id);

                    if let Some(magic_method) =
                        MAGIC_METHODS.iter().find(|magic_method| magic_method.eq_ignore_ascii_case(method_name))
                    {
                        context.report(
                            Issue::error(format!(
                                "enum `{}` cannot contain magic method `{}`",
                                enum_name, magic_method
                            ))
                            .with_annotation(Annotation::primary(method.name.span))
                            .with_annotation(Annotation::secondary(r#enum.name.span())),
                        );
                    }

                    if let Some(abstract_modifier) = method.modifiers.get_abstract() {
                        context.report(
                            Issue::error(format!("enum method `{}::{}` must not be abstract", enum_name, method_name))
                                .with_annotation(Annotation::primary(abstract_modifier.span()))
                                .with_annotations([
                                    Annotation::secondary(r#enum.span())
                                        .with_message(format!("enum `{}` is defined here", enum_fqcn)),
                                    Annotation::secondary(method.span())
                                        .with_message(format!("method `{}::{}` defined here", enum_name, method_name)),
                                ]),
                        );
                    }

                    self.process_method(
                        method,
                        method_name,
                        r#enum.span(),
                        enum_name,
                        enum_fqcn,
                        "enum",
                        false,
                        context,
                    );
                }
                ClassLikeMember::Property(property) => {
                    context.report(
                        Issue::error(format!("enum `{}` cannot have properties", enum_name))
                            .with_annotation(Annotation::primary(property.span()))
                            .with_annotation(Annotation::secondary(r#enum.span())),
                    );

                    self.process_property(property, r#enum.span(), "enum", enum_name, enum_fqcn, false, context);
                }
                ClassLikeMember::Constant(class_like_constant) => {
                    self.process_class_like_constant(
                        class_like_constant,
                        r#enum.span(),
                        "enum",
                        enum_name,
                        enum_fqcn,
                        context,
                    );
                }
                _ => {}
            }
        }
    }

    fn walk_in_anonymous_class(&self, anonymous_class: &AnonymousClass, context: &mut Context<'_>) {
        let mut last_final = None;
        let mut last_readonly = None;

        for modifier in anonymous_class.modifiers.iter() {
            match &modifier {
                Modifier::Static(_)
                | Modifier::Abstract(_)
                | Modifier::PrivateSet(_)
                | Modifier::ProtectedSet(_)
                | Modifier::PublicSet(_)
                | Modifier::Public(_)
                | Modifier::Protected(_)
                | Modifier::Private(_) => {
                    let modifier_name = modifier.as_str(context.interner);

                    context.report(
                        Issue::error(format!(
                            "class `{}` cannot have `{}` modifier",
                            ANONYMOUS_CLASS_NAME, modifier_name
                        ))
                        .with_annotation(Annotation::primary(modifier.span()))
                        .with_annotation(
                            Annotation::secondary(anonymous_class.span())
                                .with_message(format!("class `{}` defined here", ANONYMOUS_CLASS_NAME)),
                        )
                        .with_help(format!("help: remove the `{}` modifier", modifier_name)),
                    );
                }
                Modifier::Final(keyword) => {
                    if let Some(span) = last_final {
                        context.report(
                            Issue::error(format!(
                                "class `{}` cannot have multiple `final` modifiers",
                                ANONYMOUS_CLASS_NAME
                            ))
                            .with_annotation(Annotation::primary(keyword.span()))
                            .with_annotation(Annotation::secondary(span).with_message("previous `final` modifier"))
                            .with_annotation(
                                Annotation::secondary(anonymous_class.span())
                                    .with_message(format!("class `{}` defined here", ANONYMOUS_CLASS_NAME)),
                            )
                            .with_help("help: remove the duplicate `final` modifier"),
                        );
                    }

                    last_final = Some(keyword.span);
                }
                Modifier::Readonly(keyword) => {
                    if let Some(span) = last_readonly {
                        context.report(
                            Issue::error(format!(
                                "class `{}` cannot have multiple `readonly` modifiers",
                                ANONYMOUS_CLASS_NAME
                            ))
                            .with_annotations([
                                Annotation::primary(keyword.span),
                                Annotation::secondary(span).with_message("previous `readonly` modifier"),
                                Annotation::secondary(anonymous_class.span())
                                    .with_message(format!("class `{}` defined here", ANONYMOUS_CLASS_NAME)),
                            ])
                            .with_help("help: remove the duplicate `readonly` modifier"),
                        );
                    }

                    last_readonly = Some(keyword.span);
                }
            }
        }

        if let Some(extends) = &anonymous_class.extends {
            self.process_extends(
                extends,
                anonymous_class.span(),
                "class",
                ANONYMOUS_CLASS_NAME,
                ANONYMOUS_CLASS_NAME,
                true,
                context,
            );
        }

        if let Some(implements) = &anonymous_class.implements {
            self.process_implements(
                implements,
                anonymous_class.span(),
                "class",
                ANONYMOUS_CLASS_NAME,
                ANONYMOUS_CLASS_NAME,
                false,
                context,
            );
        }

        self.process_members(
            &anonymous_class.members,
            anonymous_class.span(),
            "class",
            ANONYMOUS_CLASS_NAME,
            ANONYMOUS_CLASS_NAME,
            context,
        );

        for member in anonymous_class.members.iter() {
            match &member {
                ClassLikeMember::EnumCase(case) => {
                    context.report(
                        Issue::error(format!("class `{}` cannot contain enum cases", ANONYMOUS_CLASS_NAME))
                            .with_annotations([
                                Annotation::primary(case.span()),
                                Annotation::secondary(anonymous_class.span())
                                    .with_message(format!("class `{}` defined here", ANONYMOUS_CLASS_NAME)),
                            ]),
                    );
                }
                ClassLikeMember::Method(method) => {
                    let method_name = context.interner.lookup(&method.name.value);

                    if let Some(abstract_modifier) = method.modifiers.get_abstract() {
                        context.report(
                            Issue::error(format!("anonymous class method `{}` must not be abstract", method_name))
                                .with_annotations([
                                    Annotation::primary(abstract_modifier.span()),
                                    Annotation::secondary(anonymous_class.span())
                                        .with_message(format!("class `{}` defined here", ANONYMOUS_CLASS_NAME)),
                                    Annotation::secondary(method.span())
                                        .with_message(format!("method `{}` defined here", method_name)),
                                ]),
                        );
                    }

                    self.process_method(
                        method,
                        method_name,
                        anonymous_class.span(),
                        ANONYMOUS_CLASS_NAME,
                        ANONYMOUS_CLASS_NAME,
                        "class",
                        false,
                        context,
                    );
                }
                ClassLikeMember::Property(property) => {
                    self.process_property(
                        property,
                        anonymous_class.span(),
                        "class",
                        ANONYMOUS_CLASS_NAME,
                        ANONYMOUS_CLASS_NAME,
                        false,
                        context,
                    );
                }
                ClassLikeMember::Constant(class_like_constant) => {
                    self.process_class_like_constant(
                        class_like_constant,
                        anonymous_class.span(),
                        "class",
                        ANONYMOUS_CLASS_NAME,
                        ANONYMOUS_CLASS_NAME,
                        context,
                    );
                }
                _ => {}
            }
        }
    }

    fn walk_in_function(&self, function: &Function, context: &mut Context<'_>) {
        self.process_promoted_properties_outside_constructor(&function.parameters, context);

        let name = context.interner.lookup(&function.name.value);
        let fqfn = context.lookup_name(&function.name.span.start);

        let hint = if let Some(return_hint) = &function.return_type_hint {
            &return_hint.hint
        } else {
            return;
        };

        let returns = mago_ast_utils::find_returns_in_block(&function.body);

        match &hint {
            Hint::Void(_) => {
                for r#return in returns {
                    if let Some(val) = &r#return.value {
                        context.report(
                            Issue::error(format!(
                                "function `{}` with return type of `void` must not return a value",
                                name
                            ))
                            .with_annotation(Annotation::primary(val.span()))
                            .with_annotation(
                                Annotation::secondary(function.span())
                                    .with_message(format!("function `{}` defined here", fqfn)),
                            )
                            .with_help("help: remove the return type hint, or remove the return value"),
                        );
                    }
                }
            }
            Hint::Never(_) => {
                for r#return in returns {
                    context.report(
                        Issue::error(format!("function `{}` with return type of `never` must not return", name))
                            .with_annotation(Annotation::primary(r#return.span()))
                            .with_annotation(
                                Annotation::secondary(function.span())
                                    .with_message(format!("function `{}` defined here", fqfn)),
                            )
                            .with_help("remove the return type hint, or remove the return statement"),
                    );
                }
            }
            _ if !returns_generator(context, &function.body, hint) => {
                for r#return in returns {
                    if r#return.value.is_none() {
                        context.report(
                            Issue::error(format!("function `{}` with return type must return a value", name))
                                .with_annotation(Annotation::primary(r#return.span()))
                                .with_annotation(
                                    Annotation::secondary(function.span())
                                        .with_message(format!("function `{}` defined here", fqfn)),
                                )
                                .with_note("did you mean `return null;` instead of `return;`?")
                                .with_help("add a return value to the statement"),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn walk_in_attribute(&self, attribute: &Attribute, context: &mut Context<'_>) {
        let name = context.interner.lookup(&attribute.name.value());

        if let Some(list) = &attribute.arguments {
            for argument in list.arguments.iter() {
                let (ellipsis, value) = match &argument {
                    Argument::Positional(positional_argument) => {
                        (positional_argument.ellipsis, &positional_argument.value)
                    }
                    Argument::Named(named_argument) => (named_argument.ellipsis, &named_argument.value),
                };

                if let Some(ellipsis) = ellipsis {
                    context.report(
                        Issue::error("cannot use argument unpacking in attribute arguments")
                            .with_annotation(Annotation::primary(ellipsis.span()))
                            .with_annotation(
                                Annotation::secondary(attribute.name.span())
                                    .with_message(format!("attribute `{}` defined here", name)),
                            )
                            .with_note("unpacking arguments is not allowed in attribute arguments"),
                    );
                }

                if !value.is_constant(true) {
                    context.report(
                        Issue::error(format!("attribute `{}` argument contains a non-constant expression", name))
                            .with_annotations([
                                Annotation::primary(value.span()),
                                Annotation::secondary(attribute.name.span())
                                    .with_message(format!("attribute `{}` defined here", name)),
                            ])
                            .with_note("attribute arguments must be constant expressions"),
                    );
                }
            }
        }
    }

    fn walk_in_goto(&self, goto: &Goto, context: &mut Context<'_>) {
        let all_labels =
            context.program().filter_map(|node| if let Node::Label(label) = node { Some(*label) } else { None });

        if all_labels.iter().any(|l| l.name.value == goto.label.value) {
            return;
        }

        // If we reach this point, the label was not found
        // try to see if there is a label with the same name but different case
        // if so, suggest the correct label
        let going_to = context.interner.lookup(&goto.label.value);
        let mut suggestions = vec![];
        for label in all_labels {
            let label_name = context.interner.lookup(&label.name.value);
            if label_name.eq_ignore_ascii_case(going_to) {
                suggestions.push((label_name, label.name.span));
            }
        }

        let mut issue =
            Issue::error(format!("undefined goto label `{}`", going_to))
                .with_annotation(Annotation::primary(goto.label.span))
                .with_annotations(suggestions.iter().map(|(name, span)| {
                    Annotation::secondary(*span).with_message(format!("did you mean `{}`?", name))
                }));

        if 1 == suggestions.len() {
            issue =
                issue.with_note(format!("goto label `{}` not found, did you mean `{}`?", going_to, suggestions[0].0));
        } else if !suggestions.is_empty() {
            let names = suggestions.iter().map(|(name, _)| format!("`{}`", name)).collect::<Vec<_>>().join(", ");

            issue = issue.with_note(format!("goto label `{}` not found, did you mean one of: {}?", going_to, names));
        }

        context.report(issue);
    }

    fn walk_in_argument_list(&self, argument_list: &ArgumentList, context: &mut Context<'_>) {
        let mut last_named_argument: Option<Span> = None;
        let mut last_unpacking: Option<Span> = None;

        for argument in argument_list.arguments.iter() {
            match &argument {
                Argument::Positional(positional_argument) => {
                    if let Some(ellipsis) = positional_argument.ellipsis {
                        if let Some(last_named_argument) = last_named_argument {
                            context.report(
                                Issue::error("cannot use argument unpacking after a named argument")
                                    .with_annotation(Annotation::primary(ellipsis.span()))
                                    .with_annotation(
                                        Annotation::secondary(last_named_argument).with_message("named argument here"),
                                    )
                                    .with_note("unpacking arguments must come before named arguments"),
                            );
                        }

                        last_unpacking = Some(ellipsis.span());
                    } else {
                        if let Some(named_argument) = last_named_argument {
                            context.report(
                                Issue::error("cannot use positional argument after a named argument")
                                    .with_annotation(Annotation::primary(positional_argument.span()))
                                    .with_annotation(
                                        Annotation::secondary(named_argument).with_message("named argument here"),
                                    )
                                    .with_note("positional arguments must come before named arguments"),
                            );
                        }

                        if let Some(unpacking) = last_unpacking {
                            context.report(
                                Issue::error("cannot use positional argument after argument unpacking")
                                    .with_annotation(Annotation::primary(positional_argument.span()))
                                    .with_annotation(
                                        Annotation::secondary(unpacking).with_message("argument unpacking here"),
                                    )
                                    .with_note("positional arguments must come before unpacking arguments"),
                            );
                        }
                    }
                }
                Argument::Named(named_argument) => {
                    if let Some(ellipsis) = named_argument.ellipsis {
                        context.report(
                            Issue::error("cannot use argument unpacking in named arguments")
                                .with_annotation(Annotation::primary(ellipsis.span()))
                                .with_annotation(
                                    Annotation::secondary(named_argument.span()).with_message("named argument here"),
                                )
                                .with_note("unpacking arguments is not allowed in named arguments"),
                        );
                    }

                    last_named_argument = Some(named_argument.span());
                }
            }
        }
    }

    fn walk_in_closure(&self, closure: &Closure, context: &mut Context<'_>) {
        self.process_promoted_properties_outside_constructor(&closure.parameters, context);

        let hint = if let Some(return_hint) = &closure.return_type_hint {
            &return_hint.hint
        } else {
            return;
        };

        let returns = mago_ast_utils::find_returns_in_block(&closure.body);

        match &hint {
            Hint::Void(_) => {
                for r#return in returns {
                    if let Some(val) = &r#return.value {
                        context.report(
                            Issue::error("closure with return type of `void` must not return a value")
                                .with_annotation(Annotation::primary(val.span()))
                                .with_annotation(
                                    Annotation::secondary(closure.function.span).with_message("closure defined here"),
                                )
                                .with_help("remove the return type hint, or remove the return value"),
                        );
                    }
                }
            }
            Hint::Never(_) => {
                for r#return in returns {
                    context.report(
                        Issue::error("closure with return type of `never` must not return")
                            .with_annotation(Annotation::primary(r#return.span()))
                            .with_annotation(
                                Annotation::secondary(closure.function.span).with_message("closure defined here"),
                            )
                            .with_help("remove the return type hint, or remove the return statement"),
                    );
                }
            }
            _ if !returns_generator(context, &closure.body, hint) => {
                for r#return in returns {
                    if r#return.value.is_none() {
                        context.report(
                            Issue::error("closure with return type must return a value")
                                .with_annotation(Annotation::primary(r#return.span()))
                                .with_annotation(
                                    Annotation::secondary(closure.function.span).with_message("closure defined here"),
                                )
                                .with_note("did you mean `return null;` instead of `return;`?")
                                .with_help("add a return value to the statement"),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn walk_in_arrow_function(&self, arrow_function: &ArrowFunction, context: &mut Context<'_>) {
        self.process_promoted_properties_outside_constructor(&arrow_function.parameters, context);

        if let Some(return_hint) = &arrow_function.return_type_hint {
            // while technically valid, it is not possible to return `void` from an arrow function
            // because the return value is always inferred from the body, even if the body does
            // not return a value, in the case it throws or exits the process.
            //
            // see: https://3v4l.org/VgoiO
            if let Hint::Void(_) = &return_hint.hint {
                context.report(
                    Issue::error("arrow function cannot have a return type of `void`")
                        .with_annotation(Annotation::primary(return_hint.hint.span()))
                        .with_annotation(Annotation::secondary(arrow_function.r#fn.span))
                        .with_help("remove the return type hint, or use a different type"),
                );
            }
        }
    }

    fn walk_in_function_like_parameter_list(
        &self,
        function_like_parameter_list: &FunctionLikeParameterList,

        context: &mut Context<'_>,
    ) {
        let mut last_variadic = None;
        let mut parameters_seen = vec![];
        for parameter in function_like_parameter_list.parameters.iter() {
            let name = context.interner.lookup(&parameter.variable.name);
            if let Some(prev_span) =
                parameters_seen.iter().find_map(|(n, s)| if parameter.variable.name.eq(n) { Some(s) } else { None })
            {
                context.report(
                    Issue::error(format!("parameter `{}` is already defined", name))
                        .with_annotation(Annotation::primary(parameter.variable.span()))
                        .with_annotation(Annotation::secondary(*prev_span).with_message("previously defined here")),
                );
            } else if !parameter.is_promoted_property() {
                parameters_seen.push((parameter.variable.name, parameter.variable.span()));
            }

            let mut last_readonly = None;
            let mut last_read_visibility = None;
            let mut last_write_visibility = None;
            for modifier in parameter.modifiers.iter() {
                match &modifier {
                    Modifier::Static(keyword) | Modifier::Final(keyword) | Modifier::Abstract(keyword) => {
                        context.report(
                            Issue::error(format!(
                                "parameter `{}` cannot have modifier `{}`",
                                name,
                                context.interner.lookup(&keyword.value)
                            ))
                            .with_annotation(Annotation::primary(modifier.span()))
                            .with_annotation(
                                Annotation::secondary(parameter.variable.span)
                                    .with_message(format!("parameter `{}` defined here", name)),
                            )
                            .with_help("remove the modifier from the parameter"),
                        );
                    }
                    Modifier::Readonly(_) => {
                        if let Some(s) = last_readonly {
                            context.report(
                                Issue::error(format!("parameter `{}` cannot have multiple `readonly` modifiers", name))
                                    .with_annotation(Annotation::primary(modifier.span()))
                                    .with_annotation(
                                        Annotation::secondary(s).with_message("previous `readonly` modifier"),
                                    )
                                    .with_help("remove the duplicate `readonly` modifier"),
                            );
                        } else {
                            last_readonly = Some(modifier.span());
                        }
                    }
                    Modifier::Public(_) | Modifier::Protected(_) | Modifier::Private(_) => {
                        if let Some(s) = last_read_visibility {
                            context.report(
                                Issue::error(format!("parameter `{}` cannot have multiple visibility modifiers", name))
                                    .with_annotation(Annotation::primary(modifier.span()))
                                    .with_annotation(
                                        Annotation::secondary(s).with_message("previous visibility modifier"),
                                    )
                                    .with_help("remove the duplicate visibility modifier"),
                            );
                        } else {
                            last_read_visibility = Some(modifier.span());
                        }
                    }
                    Modifier::PrivateSet(_) | Modifier::ProtectedSet(_) | Modifier::PublicSet(_) => {
                        if let Some(s) = last_write_visibility {
                            context.report(
                                Issue::error(format!(
                                    "parameter `{}` cannot have multiple write visibility modifiers",
                                    name
                                ))
                                .with_annotation(Annotation::primary(modifier.span()))
                                .with_annotation(Annotation::secondary(s).with_message("previous visibility modifier"))
                                .with_help("remove the duplicate visibility modifier"),
                            );
                        } else {
                            last_write_visibility = Some(modifier.span());
                        }
                    }
                }
            }

            if let Some((n, s)) = last_variadic {
                context.report(
                    Issue::error(format!(
                        "parameter `{}` is defined after variadic parameter `{}`",
                        name,
                        context.interner.lookup(&n)
                    ))
                    .with_annotation(Annotation::primary(parameter.variable.span()))
                    .with_annotation(Annotation::secondary(s).with_message("variadic parameter defined here"))
                    .with_help("move all parameters after the variadic parameter to the end of the parameter list"),
                );
            }

            if let Some(ellipsis) = parameter.ellipsis {
                if let Some(default) = &parameter.default_value {
                    context.report(
                        Issue::error(format!("variadic parameter `{}` cannot have a default value", name))
                            .with_annotation(Annotation::primary(default.span()))
                            .with_annotation(
                                Annotation::secondary(ellipsis.join(parameter.variable.span))
                                    .with_message(format!("parameter `{}` is variadic", name)),
                            )
                            .with_help("remove the default value from the variadic parameter"),
                    );
                }

                last_variadic = Some((parameter.variable.name, parameter.span()));
                continue;
            }

            if let Some(hint) = &parameter.hint {
                if hint.is_bottom() {
                    let hint_name = context.lookup_hint(hint);

                    context.report(
                        Issue::error(format!("bottom type `{}` cannot be used as a parameter type", hint_name))
                            .with_annotation(Annotation::primary(hint.span()))
                            .with_annotation(
                                Annotation::secondary(parameter.variable.span())
                                    .with_message(format!("parameter `{}` defined here", name)),
                            ),
                    );
                }
            }
        }
    }

    fn walk_in_match(&self, r#match: &Match, context: &mut Context<'_>) {
        let mut last_default: Option<Span> = None;

        for arm in r#match.arms.iter() {
            if let MatchArm::Default(default_arm) = &arm {
                if let Some(previous) = last_default {
                    context.report(
                        Issue::error("match expression may only contain one default arm")
                            .with_annotation(Annotation::primary(default_arm.span()))
                            .with_annotation(
                                Annotation::secondary(previous).with_message("previous default arm defined here"),
                            )
                            .with_annotation(Annotation::secondary(r#match.span()))
                            .with_help("remove this default case"),
                    );
                } else {
                    last_default = Some(default_arm.default.span);
                }
            }
        }
    }

    fn walk_in_switch(&self, switch: &Switch, context: &mut Context<'_>) {
        let mut last_default: Option<Span> = None;

        let cases = match &switch.body {
            SwitchBody::BraceDelimited(switch_brace_delimited_body) => &switch_brace_delimited_body.cases,
            SwitchBody::ColonDelimited(switch_colon_delimited_body) => &switch_colon_delimited_body.cases,
        };

        for case in cases.iter() {
            if let SwitchCase::Default(default_case) = &case {
                if let Some(previous) = last_default {
                    context.report(
                        Issue::error("switch statement may only contain one default case")
                            .with_annotation(Annotation::primary(default_case.span()))
                            .with_annotation(
                                Annotation::secondary(previous).with_message("previous default case defined here"),
                            )
                            .with_annotation(Annotation::secondary(switch.span()))
                            .with_help("remove this default case"),
                    );
                } else {
                    last_default = Some(default_case.default.span);
                }
            }
        }
    }
}

/// Defines the semantics of magic methods.
///
/// The tuple contains the following elements:
///
/// 1. The name of the magic method.
/// 2. The number of arguments the magic method accepts, or none if it can accept any number of arguments.
/// 3. Whether the magic method has to be public.
/// 4. Whether the magic method has to be static.
/// 5. Whether the magic method can contain a return type.
const MAGIC_METHOD_SEMANTICS: &[(&str, Option<usize>, bool, bool, bool)] = &[
    (CONSTRUCTOR_MAGIC_METHOD, None, false, false, false),
    (DESTRUCTOR_MAGIC_METHOD, None, false, false, false),
    (CLONE_MAGIC_METHOD, None, false, false, true),
    (CALL_MAGIC_METHOD, Some(2), true, false, true),
    (CALL_STATIC_MAGIC_METHOD, Some(2), true, true, true),
    (GET_MAGIC_METHOD, Some(1), true, false, true),
    (SET_MAGIC_METHOD, Some(2), true, false, true),
    (ISSET_MAGIC_METHOD, Some(1), true, false, true),
    (UNSET_MAGIC_METHOD, Some(1), true, false, true),
    (SLEEP_MAGIC_METHOD, Some(0), true, false, true),
    (WAKEUP_MAGIC_METHOD, Some(0), true, false, true),
    (SERIALIZE_MAGIC_METHOD, Some(0), true, false, true),
    (UNSERIALIZE_MAGIC_METHOD, Some(1), true, false, true),
    (TO_STRING_MAGIC_METHOD, Some(0), true, false, true),
    (INVOKE_MAGIC_METHOD, None, true, false, true),
    (SET_STATE_MAGIC_METHOD, Some(1), true, true, true),
    (DEBUG_INFO_MAGIC_METHOD, Some(0), true, false, true),
];

fn returns_generator<'ast>(context: &mut Context<'_>, block: &'ast Block, hint: &'ast Hint) -> bool {
    if hint_contains_generator(context, hint) {
        return true;
    }

    mago_ast_utils::block_has_yield(block)
}

fn hint_contains_generator(context: &mut Context<'_>, hint: &Hint) -> bool {
    match hint {
        Hint::Identifier(identifier) => {
            let symbol = context.lookup_name(&identifier.span().start);

            "Generator" == symbol
        }
        Hint::Parenthesized(parenthesized_hint) => hint_contains_generator(context, &parenthesized_hint.hint),
        Hint::Nullable(nullable_hint) => hint_contains_generator(context, &nullable_hint.hint),
        Hint::Union(union_hint) => {
            hint_contains_generator(context, &union_hint.left) || hint_contains_generator(context, &union_hint.right)
        }
        Hint::Intersection(intersection_hint) => {
            hint_contains_generator(context, &intersection_hint.left)
                || hint_contains_generator(context, &intersection_hint.right)
        }
        _ => false,
    }
}
