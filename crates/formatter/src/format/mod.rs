use mago_ast::*;
use mago_span::HasSpan;
use mago_span::Span;

use crate::comment::CommentFlags;
use crate::document::*;
use crate::format::assignment::print_assignment;
use crate::format::assignment::AssignmentLikeNode;
use crate::format::block::print_block_of_nodes;
use crate::format::call_node::print_call_like_node;
use crate::format::call_node::CallLikeNode;
use crate::format::class_like::print_class_like_body;
use crate::format::misc::print_attribute_list_sequence;
use crate::format::misc::print_colon_delimited_body;
use crate::format::misc::print_modifiers;
use crate::format::parameters::print_function_like_parameters;
use crate::format::statement::print_statement_sequence;
use crate::settings::*;
use crate::wrap;
use crate::Formatter;

pub mod array;
pub mod assignment;
pub mod binaryish;
pub mod block;
pub mod call;
pub mod call_arguments;
pub mod call_node;
pub mod class_like;
pub mod control_structure;
pub mod expression;
pub mod misc;
pub mod parameters;
pub mod statement;
pub mod string;

pub trait Format<'a> {
    #[must_use]
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a>;
}

impl<'a, T> Format<'a> for Box<T>
where
    T: Format<'a>,
{
    fn format(&'a self, p: &mut Formatter<'a>) -> Document<'a> {
        (**self).format(p)
    }
}

impl<'a, T> Format<'a> for &'a T
where
    T: Format<'a>,
{
    fn format(&'a self, p: &mut Formatter<'a>) -> Document<'a> {
        (**self).format(p)
    }
}

impl<'a> Format<'a> for Program {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.enter_node(Node::Program(self));
        let mut parts = vec![];
        if let Some(doc) = block::print_block_body(f, &self.statements) {
            parts.push(doc);
        }

        f.leave_node();

        if f.scripting_mode {
            parts.push(Document::Line(Line::hardline()));

            if let Some(last_span) = self.trivia.last_span().or_else(|| self.statements.last_span()) {
                let first_span = self.trivia.first_span().or_else(|| self.statements.first_span()).unwrap_or(last_span);

                if let Some(comments) = f.print_dangling_comments(first_span.join(last_span), false) {
                    parts.push(Document::Line(Line::hardline()));
                    parts.push(comments);
                }
            }
        }

        Document::Array(parts)
    }
}

impl<'a> Format<'a> for Statement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Statement, {
            match self {
                Statement::OpeningTag(t) => t.format(f),
                Statement::ClosingTag(t) => t.format(f),
                Statement::Inline(i) => i.format(f),
                Statement::Namespace(n) => n.format(f),
                Statement::Use(u) => u.format(f),
                Statement::Class(c) => c.format(f),
                Statement::Interface(i) => i.format(f),
                Statement::Trait(t) => t.format(f),
                Statement::Enum(e) => e.format(f),
                Statement::Block(b) => b.format(f),
                Statement::Constant(c) => c.format(f),
                Statement::Function(u) => u.format(f),
                Statement::Declare(d) => d.format(f),
                Statement::Goto(g) => g.format(f),
                Statement::Label(l) => l.format(f),
                Statement::Try(t) => t.format(f),
                Statement::Foreach(o) => o.format(f),
                Statement::For(o) => o.format(f),
                Statement::While(w) => w.format(f),
                Statement::DoWhile(d) => d.format(f),
                Statement::Continue(c) => c.format(f),
                Statement::Break(b) => b.format(f),
                Statement::Switch(s) => s.format(f),
                Statement::If(i) => i.format(f),
                Statement::Return(r) => r.format(f),
                Statement::Expression(e) => e.format(f),
                Statement::Echo(e) => e.format(f),
                Statement::Global(g) => g.format(f),
                Statement::Static(s) => s.format(f),
                Statement::HaltCompiler(h) => h.format(f),
                Statement::Unset(u) => u.format(f),
                Statement::Noop(_) => Document::String(";"),
            }
        })
    }
}

impl<'a> Format<'a> for OpeningTag {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, OpeningTag, {
            match &self {
                OpeningTag::Full(tag) => tag.format(f),
                OpeningTag::Short(tag) => tag.format(f),
                OpeningTag::Echo(tag) => tag.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for FullOpeningTag {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = true;

        wrap!(f, self, FullOpeningTag, {
            Document::String(match f.settings.keyword_case {
                CasingStyle::Lowercase => "<?php",
                CasingStyle::Uppercase => "<?PHP",
            })
        })
    }
}

impl<'a> Format<'a> for ShortOpeningTag {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = true;

        wrap!(f, self, ShortOpeningTag, { Document::String("<?") })
    }
}

impl<'a> Format<'a> for EchoOpeningTag {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = true;

        wrap!(f, self, EchoOpeningTag, { Document::String("<?=") })
    }
}

impl<'a> Format<'a> for ClosingTag {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = false;

        wrap!(f, self, ClosingTag, {
            let last_index = self.span.end.offset;
            // todo: put this behind a setting
            if f.skip_spaces_and_new_lines(Some(last_index), false).is_none() {
                f.scripting_mode = true;

                Document::empty()
            } else {
                Document::String("?>")
            }
        })
    }
}

impl<'a> Format<'a> for Inline {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = false;

        wrap!(f, self, Inline, { Document::String(f.lookup(&self.value)) })
    }
}

impl<'a> Format<'a> for Declare {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Declare, {
            let mut contents = vec![self.declare.format(f)];

            contents.push(Document::String("("));

            let len = self.items.len();
            for (i, item) in self.items.iter().enumerate() {
                contents.push(item.format(f));
                if i != len - 1 {
                    contents.push(Document::String(", "));
                }
            }

            contents.push(Document::String(")"));
            contents.push(self.body.format(f));

            Document::Group(Group::new(contents).with_break(true))
        })
    }
}

impl<'a> Format<'a> for DeclareItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, DeclareItem, {
            Document::Array(vec![
                self.name.format(f),
                if f.settings.space_around_declare_equals { Document::space() } else { Document::empty() },
                Document::String("="),
                if f.settings.space_around_declare_equals { Document::space() } else { Document::empty() },
                self.value.format(f),
            ])
        })
    }
}

impl<'a> Format<'a> for DeclareBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, DeclareBody, {
            match self {
                DeclareBody::Statement(s) => {
                    let body = s.format(f);

                    misc::adjust_clause(f, s, body, false)
                }
                DeclareBody::ColonDelimited(b) => b.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for DeclareColonDelimitedBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, DeclareColonDelimitedBody, {
            print_colon_delimited_body(f, &self.colon, &self.statements, &self.end_declare, &self.terminator)
        })
    }
}

impl<'a> Format<'a> for Namespace {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Namespace, {
            let mut parts = vec![self.namespace.format(f)];

            if let Some(name) = &self.name {
                parts.push(Document::space());
                parts.push(name.format(f));
            }

            match &self.body {
                NamespaceBody::Implicit(namespace_implicit_body) => {
                    parts.push(namespace_implicit_body.terminator.format(f));
                    parts.push(Document::Line(Line::hardline()));
                    parts.push(Document::Line(Line::hardline()));

                    parts.extend(print_statement_sequence(f, &namespace_implicit_body.statements));
                }
                NamespaceBody::BraceDelimited(block) => {
                    parts.push(Document::space());
                    parts.push(block.format(f));
                }
            }

            Document::Array(parts)
        })
    }
}

impl<'a> Format<'a> for Identifier {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Identifier, {
            match self {
                Identifier::Local(i) => i.format(f),
                Identifier::Qualified(i) => i.format(f),
                Identifier::FullyQualified(i) => i.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for LocalIdentifier {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LocalIdentifier, { Document::String(f.lookup(&self.value)) })
    }
}

impl<'a> Format<'a> for QualifiedIdentifier {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, QualifiedIdentifier, { Document::String(f.lookup(&self.value)) })
    }
}

impl<'a> Format<'a> for FullyQualifiedIdentifier {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FullyQualifiedIdentifier, { Document::String(f.lookup(&self.value)) })
    }
}

impl<'a> Format<'a> for Use {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Use, {
            Document::Group(Group::new(vec![
                self.r#use.format(f),
                Document::space(),
                match &self.items {
                    UseItems::Sequence(s) => s.format(f),
                    UseItems::TypedSequence(s) => s.format(f),
                    UseItems::TypedList(t) => t.format(f),
                    UseItems::MixedList(m) => m.format(f),
                },
                self.terminator.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for UseItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, UseItem, {
            let mut parts = vec![self.name.format(f)];

            if let Some(alias) = &self.alias {
                parts.push(Document::space());
                parts.push(alias.format(f));
            }

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for UseItemSequence {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, UseItemSequence, {
            Document::Group(Group::new(vec![
                Document::Indent(Document::join(
                    self.items.iter().map(|i| i.format(f)).collect(),
                    Separator::CommaLine,
                )),
                Document::Line(Line::softline()),
            ]))
        })
    }
}

impl<'a> Format<'a> for TypedUseItemList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TypedUseItemList, {
            let mut contents = vec![
                self.r#type.format(f),
                Document::space(),
                self.namespace.format(f),
                Document::String("\\"),
                Document::String("{"),
            ];

            if !self.items.is_empty() {
                let mut items = Document::join(self.items.iter().map(|i| i.format(f)).collect(), Separator::CommaLine);
                items.insert(0, Document::Line(Line::softline()));

                contents.push(Document::Indent(items));
            }

            if let Some(comments) = f.print_dangling_comments(self.left_brace.join(self.right_brace), true) {
                contents.push(comments);
            } else {
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(Document::String("}"));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for MixedUseItemList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MixedUseItemList, {
            let mut contents = vec![self.namespace.format(f), Document::String("\\"), Document::String("{")];

            if !self.items.is_empty() {
                let mut items = Document::join(self.items.iter().map(|i| i.format(f)).collect(), Separator::CommaLine);
                items.insert(0, Document::Line(Line::softline()));

                contents.push(Document::Indent(items));
            }

            if let Some(comments) = f.print_dangling_comments(self.left_brace.join(self.right_brace), true) {
                contents.push(comments);
            } else {
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(Document::String("}"));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for MaybeTypedUseItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MaybeTypedUseItem, {
            match &self.r#type {
                Some(t) => Document::Group(Group::new(vec![t.format(f), Document::space(), self.item.format(f)])),
                None => self.item.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for TypedUseItemSequence {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TypedUseItemSequence, {
            Document::Group(Group::new(vec![
                self.r#type.format(f),
                Document::space(),
                Document::Indent(Document::join(
                    self.items.iter().map(|i| i.format(f)).collect(),
                    Separator::CommaLine,
                )),
                Document::Line(Line::softline()),
            ]))
        })
    }
}

impl<'a> Format<'a> for UseItemAlias {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, UseItemAlias, {
            Document::Group(Group::new(vec![self.r#as.format(f), Document::space(), self.identifier.format(f)]))
        })
    }
}

impl<'a> Format<'a> for UseType {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, UseType, {
            match self {
                UseType::Function(keyword) => keyword.format(f),
                UseType::Const(keyword) => keyword.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for TraitUse {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUse, {
            let mut contents = vec![self.r#use.format(f), Document::space()];
            for (i, trait_name) in self.trait_names.iter().enumerate() {
                if i != 0 {
                    contents.push(Document::String(", "));
                }

                contents.push(trait_name.format(f));
            }

            contents.push(self.specification.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for TraitUseSpecification {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseSpecification, {
            match self {
                TraitUseSpecification::Abstract(s) => s.format(f),
                TraitUseSpecification::Concrete(s) => Document::Array(vec![Document::space(), s.format(f)]),
            }
        })
    }
}

impl<'a> Format<'a> for TraitUseAbstractSpecification {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseAbstractSpecification, { self.0.format(f) })
    }
}

impl<'a> Format<'a> for TraitUseConcreteSpecification {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseConcreteSpecification, {
            print_block_of_nodes(f, &self.left_brace, &self.adaptations, &self.right_brace, false)
        })
    }
}

impl<'a> Format<'a> for TraitUseAdaptation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseAdaptation, {
            match self {
                TraitUseAdaptation::Precedence(a) => a.format(f),
                TraitUseAdaptation::Alias(a) => a.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for TraitUseMethodReference {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseMethodReference, {
            match self {
                TraitUseMethodReference::Identifier(m) => m.format(f),
                TraitUseMethodReference::Absolute(m) => m.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for TraitUseAbsoluteMethodReference {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseAbsoluteMethodReference, {
            Document::Group(Group::new(vec![
                self.trait_name.format(f),
                Document::String("::"),
                self.method_name.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for TraitUsePrecedenceAdaptation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUsePrecedenceAdaptation, {
            let mut contents =
                vec![self.method_reference.format(f), Document::space(), self.insteadof.format(f), Document::space()];

            for (i, trait_name) in self.trait_names.iter().enumerate() {
                if i != 0 {
                    contents.push(Document::String(", "));
                }

                contents.push(trait_name.format(f));
            }

            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for TraitUseAliasAdaptation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TraitUseAliasAdaptation, {
            let mut parts = vec![self.method_reference.format(f), Document::space(), self.r#as.format(f)];

            if let Some(v) = &self.visibility {
                parts.push(Document::space());
                parts.push(v.format(f));
            }

            if let Some(a) = &self.alias {
                parts.push(Document::space());
                parts.push(a.format(f));
            }

            parts.push(self.terminator.format(f));

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for ClassLikeConstant {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeConstant, {
            let attributes = if let Some(attributes) = misc::print_attribute_list_sequence(f, &self.attributes, false) {
                attributes
            } else {
                Document::empty()
            };

            let constant = {
                let mut contents = print_modifiers(f, &self.modifiers);
                if !contents.is_empty() {
                    contents.push(Document::space());
                }

                contents.push(self.r#const.format(f));

                if let Some(h) = &self.hint {
                    contents.push(Document::space());
                    contents.push(h.format(f));
                }

                if self.items.len() == 1 {
                    contents.push(Document::space());
                    contents.push(self.items.as_slice()[0].format(f));
                } else if !self.items.is_empty() {
                    contents.push(Document::Indent(vec![Document::Line(Line::default())]));

                    contents.push(Document::Indent(Document::join(
                        self.items.iter().map(|v| v.format(f)).collect(),
                        Separator::CommaLine,
                    )));
                    contents.push(Document::Line(Line::softline()));
                }

                contents.push(self.terminator.format(f));

                Document::Group(Group::new(contents))
            };

            Document::Group(Group::new(vec![attributes, constant]))
        })
    }
}

impl<'a> Format<'a> for ClassLikeConstantItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeConstantItem, {
            let lhs = self.name.format(f);

            print_assignment(
                f,
                AssignmentLikeNode::ClassLikeConstantItem(self),
                lhs,
                Document::String("="),
                &self.value,
            )
        })
    }
}

impl<'a> Format<'a> for EnumCase {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EnumCase, {
            let mut parts = vec![];
            for attribute_list in self.attributes.iter() {
                parts.push(attribute_list.format(f));
                parts.push(Document::Line(Line::hardline()));
            }

            parts.push(self.case.format(f));
            parts.push(Document::space());
            parts.push(self.item.format(f));
            parts.push(self.terminator.format(f));

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for EnumCaseItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EnumCaseItem, {
            match self {
                EnumCaseItem::Unit(c) => c.format(f),
                EnumCaseItem::Backed(c) => c.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for EnumCaseUnitItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EnumCaseUnitItem, { self.name.format(f) })
    }
}

impl<'a> Format<'a> for EnumCaseBackedItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EnumCaseBackedItem, {
            let lhs = self.name.format(f);
            let operator = Document::String("=");

            print_assignment(f, AssignmentLikeNode::EnumCaseBackedItem(self), lhs, operator, &self.value)
        })
    }
}

impl<'a> Format<'a> for Property {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Property, {
            match self {
                Property::Plain(p) => p.format(f),
                Property::Hooked(p) => p.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for PlainProperty {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PlainProperty, {
            let attributes = misc::print_attribute_list_sequence(f, &self.attributes, false);
            let property = {
                let mut contents = print_modifiers(f, &self.modifiers);
                if let Some(var) = &self.var {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(var.format(f));
                }

                if let Some(h) = &self.hint {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(h.format(f));
                }

                if self.items.len() == 1 {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(self.items.as_slice()[0].format(f));
                } else if !self.items.is_empty() {
                    let mut items =
                        Document::join(self.items.iter().map(|v| v.format(f)).collect(), Separator::CommaLine);

                    if !contents.is_empty() {
                        items.insert(0, Document::Line(Line::default()));
                        contents.push(Document::Indent(items));
                        contents.push(Document::Line(Line::softline()));
                    } else {
                        // we don't have any modifiers, so we don't need to indent, or add a line
                        contents.extend(items);
                    }
                }

                contents.push(self.terminator.format(f));

                Document::Group(Group::new(contents))
            };

            if let Some(attributes) = attributes {
                Document::Group(Group::new(vec![attributes, property]))
            } else {
                property
            }
        })
    }
}

impl<'a> Format<'a> for HookedProperty {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, HookedProperty, {
            let attributes = misc::print_attribute_list_sequence(f, &self.attributes, false);

            let property = {
                let mut contents = print_modifiers(f, &self.modifiers);
                if let Some(var) = &self.var {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(var.format(f));
                }

                if let Some(h) = &self.hint {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(h.format(f));
                }

                if !contents.is_empty() {
                    contents.push(Document::space());
                }
                contents.push(self.item.format(f));
                contents.push(Document::space());
                contents.push(self.hooks.format(f));

                Document::Group(Group::new(contents))
            };

            if let Some(attributes) = attributes {
                Document::Group(Group::new(vec![attributes, property]))
            } else {
                property
            }
        })
    }
}

impl<'a> Format<'a> for PropertyItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyItem, {
            match self {
                PropertyItem::Abstract(p) => p.format(f),
                PropertyItem::Concrete(p) => p.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for PropertyAbstractItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyAbstractItem, { self.variable.format(f) })
    }
}

impl<'a> Format<'a> for PropertyConcreteItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyConcreteItem, {
            let lhs = self.variable.format(f);
            let operator = Document::String("=");

            print_assignment(f, AssignmentLikeNode::PropertyConcreteItem(self), lhs, operator, &self.value)
        })
    }
}

impl<'a> Format<'a> for Method {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Method, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.push(Document::Line(Line::hardline()));
            }

            let mut signature = print_modifiers(f, &self.modifiers);
            if !signature.is_empty() {
                signature.push(Document::space());
            }

            signature.push(self.function.format(f));
            signature.push(Document::space());
            if self.ampersand.is_some() {
                signature.push(Document::String("&"));
            }

            signature.push(self.name.format(f));
            signature.push(self.parameters.format(f));
            if let Some(return_type) = &self.return_type_hint {
                signature.push(return_type.format(f));
            }

            let signature_id = f.next_id();
            let signature_document = Document::Group(Group::new(signature).with_id(signature_id));

            let mut body = vec![];
            if let MethodBody::Concrete(_) = self.body {
                body.push(match f.settings.method_brace_style {
                    BraceStyle::SameLine => Document::space(),
                    BraceStyle::NextLine => Document::IfBreak(
                        IfBreak::new(Document::space(), Document::Line(Line::hardline())).with_id(signature_id),
                    ),
                });
            }

            body.push(self.body.format(f));

            Document::Group(Group::new(vec![
                Document::Group(Group::new(attributes)),
                signature_document,
                Document::Group(Group::new(body)),
            ]))
        })
    }
}

impl<'a> Format<'a> for MethodBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MethodBody, {
            match self {
                MethodBody::Abstract(b) => b.format(f),
                MethodBody::Concrete(b) => b.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for MethodAbstractBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MethodAbstractBody, { Document::String(";") })
    }
}

impl<'a> Format<'a> for Keyword {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Keyword, {
            let mut value = f.lookup(&self.value);

            value = match f.settings.keyword_case {
                CasingStyle::Lowercase => f.as_str(value.to_ascii_lowercase()),
                CasingStyle::Uppercase => f.as_str(value.to_ascii_uppercase()),
            };

            Document::String(value)
        })
    }
}

impl<'a> Format<'a> for Terminator {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Terminator, {
            match self {
                Terminator::Semicolon(_) | Terminator::TagPair(_, _) => Document::String(";"),
                Terminator::ClosingTag(t) => t.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ExpressionStatement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ExpressionStatement, {
            Document::Array(vec![self.expression.format(f), self.terminator.format(f)])
        })
    }
}

impl<'a> Format<'a> for Extends {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Extends, {
            Document::Group(Group::new(vec![
                self.extends.format(f),
                Document::Indent(vec![Document::Line(Line::default())]),
                Document::Indent(Document::join(
                    self.types.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )),
            ]))
        })
    }
}

impl<'a> Format<'a> for Implements {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Implements, {
            Document::Group(Group::new(vec![
                self.implements.format(f),
                Document::Indent(vec![Document::Line(Line::default())]),
                Document::Indent(Document::join(
                    self.types.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )),
            ]))
        })
    }
}

impl<'a> Format<'a> for ClassLikeMember {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeMember, {
            match self {
                ClassLikeMember::TraitUse(m) => m.format(f),
                ClassLikeMember::Constant(m) => m.format(f),
                ClassLikeMember::Property(m) => m.format(f),
                ClassLikeMember::EnumCase(m) => m.format(f),
                ClassLikeMember::Method(m) => m.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for Interface {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Interface, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.push(Document::Line(Line::hardline()));
            }

            let signature = vec![
                self.interface.format(f),
                Document::space(),
                self.name.format(f),
                if let Some(e) = &self.extends {
                    Document::Array(vec![Document::space(), e.format(f)])
                } else {
                    Document::empty()
                },
            ];

            let body = vec![
                match f.settings.classlike_brace_style {
                    BraceStyle::SameLine => Document::space(),
                    BraceStyle::NextLine => {
                        Document::Array(vec![Document::Line(Line::hardline()), Document::BreakParent])
                    }
                },
                print_class_like_body(f, &self.left_brace, &self.members, &self.right_brace),
            ];

            Document::Group(Group::new(vec![
                Document::Group(Group::new(attributes)),
                Document::Group(Group::new(signature)),
                Document::Group(Group::new(body)),
            ]))
        })
    }
}

impl<'a> Format<'a> for EnumBackingTypeHint {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EnumBackingTypeHint, {
            Document::Group(Group::new(vec![Document::String(":"), Document::space(), self.hint.format(f)]))
        })
    }
}

impl<'a> Format<'a> for Class {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Class, {
            let attributes = misc::print_attribute_list_sequence(f, &self.attributes, false);
            let mut signature = print_modifiers(f, &self.modifiers);
            if !signature.is_empty() {
                signature.push(Document::space());
            }

            signature.push(self.class.format(f));
            signature.push(Document::space());
            signature.push(self.name.format(f));

            if let Some(e) = &self.extends {
                signature.push(Document::space());
                signature.push(e.format(f));
            }

            if let Some(i) = &self.implements {
                signature.push(Document::space());
                signature.push(i.format(f));
            }

            let class = Document::Group(Group::new(vec![
                Document::Group(Group::new(signature)),
                Document::Group(Group::new(vec![
                    match f.settings.classlike_brace_style {
                        BraceStyle::SameLine => Document::space(),
                        BraceStyle::NextLine => {
                            Document::Array(vec![Document::Line(Line::hardline()), Document::BreakParent])
                        }
                    },
                    print_class_like_body(f, &self.left_brace, &self.members, &self.right_brace),
                ])),
            ]));

            if let Some(attributes) = attributes {
                Document::Group(Group::new(vec![attributes, class]))
            } else {
                class
            }
        })
    }
}

impl<'a> Format<'a> for Trait {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Trait, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.push(Document::Line(Line::hardline()));
            }

            let signature = vec![self.r#trait.format(f), Document::space(), self.name.format(f)];
            let body = vec![
                match f.settings.classlike_brace_style {
                    BraceStyle::SameLine => Document::space(),
                    BraceStyle::NextLine => {
                        Document::Array(vec![Document::Line(Line::hardline()), Document::BreakParent])
                    }
                },
                print_class_like_body(f, &self.left_brace, &self.members, &self.right_brace),
            ];

            Document::Group(Group::new(vec![
                Document::Group(Group::new(attributes)),
                Document::Group(Group::new(signature)),
                Document::Group(Group::new(body)),
            ]))
        })
    }
}

impl<'a> Format<'a> for Enum {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Enum, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.push(Document::Line(Line::hardline()));
            }

            let signature = vec![
                self.r#enum.format(f),
                Document::space(),
                self.name.format(f),
                if let Some(backing_type_hint) = &self.backing_type_hint {
                    // TODO: add an option to add a space before the colon
                    backing_type_hint.format(f)
                } else {
                    Document::empty()
                },
                if let Some(i) = &self.implements {
                    Document::Array(vec![Document::space(), i.format(f)])
                } else {
                    Document::empty()
                },
            ];

            let body = vec![
                match f.settings.classlike_brace_style {
                    BraceStyle::SameLine => Document::space(),
                    BraceStyle::NextLine => {
                        Document::Array(vec![Document::Line(Line::hardline()), Document::BreakParent])
                    }
                },
                print_class_like_body(f, &self.left_brace, &self.members, &self.right_brace),
            ];

            Document::Group(Group::new(vec![
                Document::Group(Group::new(attributes)),
                Document::Group(Group::new(signature)),
                Document::Group(Group::new(body)),
            ]))
        })
    }
}

impl<'a> Format<'a> for Return {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Return, {
            let mut parts = vec![];

            parts.push(self.r#return.format(f));
            if let Some(value) = &self.value {
                parts.push(Document::space());
                parts.push(value.format(f));
            }

            parts.push(self.terminator.format(f));

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for Block {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Block, { block::print_block(f, &self.left_brace, &self.statements, &self.right_brace) })
    }
}

impl<'a> Format<'a> for Echo {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Echo, {
            let mut contents = vec![self.echo.format(f), Document::Indent(vec![Document::Line(Line::default())])];

            if !self.values.is_empty() {
                contents.push(Document::Indent(Document::join(
                    self.values.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )));
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for ConstantItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ConstantItem, {
            let lhs = self.name.format(f);
            let operator = Document::String("=");

            print_assignment(f, AssignmentLikeNode::ConstantItem(self), lhs, operator, &self.value)
        })
    }
}

impl<'a> Format<'a> for Constant {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Constant, {
            let mut contents = vec![self.r#const.format(f)];

            if self.items.len() == 1 {
                contents.push(Document::space());
                contents.push(self.items.as_slice()[0].format(f));
            } else if !self.items.is_empty() {
                contents.push(Document::Indent(vec![Document::Line(Line::default())]));

                contents.push(Document::Indent(Document::join(
                    self.items.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )));
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for Attribute {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Attribute, { print_call_like_node(f, CallLikeNode::Attribute(self)) })
    }
}

impl<'a> Format<'a> for Hint {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Hint, {
            let k = |v: &str| match f.settings.keyword_case {
                CasingStyle::Lowercase => Document::String(f.as_str(v.to_ascii_lowercase())),
                CasingStyle::Uppercase => Document::String(f.as_str(v.to_ascii_uppercase())),
            };

            match self {
                Hint::Identifier(identifier) => identifier.format(f),
                Hint::Parenthesized(parenthesized_hint) => {
                    let spacing = if f.settings.type_spacing > 0 {
                        Document::String(f.as_str(" ".repeat(f.settings.type_spacing)))
                    } else {
                        Document::empty()
                    };

                    Document::Group(Group::new(vec![
                        Document::String("("),
                        spacing.clone(),
                        parenthesized_hint.hint.format(f),
                        spacing,
                        Document::String(")"),
                    ]))
                }
                Hint::Nullable(nullable_hint) => {
                    let spacing = if f.settings.type_spacing > 0 {
                        Document::String(f.as_str(" ".repeat(f.settings.type_spacing)))
                    } else {
                        Document::empty()
                    };

                    // If the nullable type is nested inside another type hint,
                    // we cannot use `?` syntax.
                    let force_long_syntax = matches!(f.parent_node(), Node::Hint(_))
                        || (matches!(
                            nullable_hint.hint.as_ref(),
                            Hint::Nullable(_) | Hint::Union(_) | Hint::Intersection(_) | Hint::Parenthesized(_)
                        ));

                    if force_long_syntax {
                        return Document::Group(Group::new(vec![
                            k("null"),
                            spacing.clone(),
                            Document::String("|"),
                            spacing,
                            nullable_hint.hint.format(f),
                        ]));
                    }

                    match f.settings.null_type_hint {
                        NullTypeHint::NullPipe => Document::Group(Group::new(vec![
                            k("null"),
                            spacing.clone(),
                            Document::String("|"),
                            spacing,
                            nullable_hint.hint.format(f),
                        ])),
                        NullTypeHint::Question => Document::Group(Group::new(vec![
                            Document::String("?"),
                            spacing,
                            nullable_hint.hint.format(f),
                        ])),
                    }
                }
                Hint::Union(union_hint) => {
                    let spacing = if f.settings.type_spacing > 0 {
                        Document::String(f.as_str(" ".repeat(f.settings.type_spacing)))
                    } else {
                        Document::empty()
                    };

                    let force_long_syntax = matches!(f.parent_node(), Node::Hint(_))
                        || matches!(
                            union_hint.left.as_ref(),
                            Hint::Nullable(_) | Hint::Union(_) | Hint::Intersection(_) | Hint::Parenthesized(_)
                        )
                        || matches!(
                            union_hint.right.as_ref(),
                            Hint::Nullable(_) | Hint::Union(_) | Hint::Intersection(_) | Hint::Parenthesized(_)
                        );

                    if !force_long_syntax {
                        if let Hint::Null(_) = union_hint.left.as_ref() {
                            if f.settings.null_type_hint.is_question() {
                                return Document::Group(Group::new(vec![
                                    Document::String("?"),
                                    spacing,
                                    union_hint.right.format(f),
                                ]));
                            }
                        }

                        if let Hint::Null(_) = union_hint.right.as_ref() {
                            if f.settings.null_type_hint.is_question() {
                                return Document::Group(Group::new(vec![
                                    Document::String("?"),
                                    spacing,
                                    union_hint.left.format(f),
                                ]));
                            }
                        }
                    }

                    Document::Group(Group::new(vec![
                        union_hint.left.format(f),
                        spacing.clone(),
                        Document::String("|"),
                        spacing,
                        union_hint.right.format(f),
                    ]))
                }
                Hint::Intersection(intersection_hint) => {
                    let spacing = if f.settings.type_spacing > 0 {
                        Document::String(f.as_str(" ".repeat(f.settings.type_spacing)))
                    } else {
                        Document::empty()
                    };

                    Document::Group(Group::new(vec![
                        intersection_hint.left.format(f),
                        spacing.clone(),
                        Document::String("&"),
                        spacing,
                        intersection_hint.right.format(f),
                    ]))
                }
                Hint::Null(_) => k("null"),
                Hint::True(_) => k("true"),
                Hint::False(_) => k("false"),
                Hint::Array(_) => k("array"),
                Hint::Callable(_) => k("callable"),
                Hint::Static(_) => k("static"),
                Hint::Self_(_) => k("self"),
                Hint::Parent(_) => k("parent"),
                Hint::Void(_) => k("void"),
                Hint::Never(_) => k("never"),
                Hint::Float(_) => k("float"),
                Hint::Bool(_) => k("bool"),
                Hint::Integer(_) => k("int"),
                Hint::String(_) => k("string"),
                Hint::Object(_) => k("object"),
                Hint::Mixed(_) => k("mixed"),
                Hint::Iterable(_) => k("iterable"),
            }
        })
    }
}

impl<'a> Format<'a> for Modifier {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Modifier, {
            match self {
                Modifier::Static(keyword) => keyword.format(f),
                Modifier::Final(keyword) => keyword.format(f),
                Modifier::Abstract(keyword) => keyword.format(f),
                Modifier::Readonly(keyword) => keyword.format(f),
                Modifier::Public(keyword) => keyword.format(f),
                Modifier::Protected(keyword) => keyword.format(f),
                Modifier::Private(keyword) => keyword.format(f),
                Modifier::PrivateSet(keyword) => keyword.format(f),
                Modifier::ProtectedSet(keyword) => keyword.format(f),
                Modifier::PublicSet(keyword) => keyword.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for AttributeList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, AttributeList, {
            // Determine if there are comments between the `#[` and the first attribute (if any).
            let has_comments_before_first = || {
                if let Some(first_attr) = self.attributes.first() {
                    f.has_comment(
                        Span { start: self.hash_left_bracket.span().end, end: first_attr.span().start },
                        CommentFlags::all(),
                    )
                } else {
                    // If there are no attributes, then no comments "before the first attribute" can apply.
                    false
                }
            };

            // Determine if there are comments between the last attribute and the `]` (if any).
            let has_comments_after_last = || {
                if let Some(last_attr) = self.attributes.last() {
                    f.has_comment(
                        Span { start: last_attr.span().end, end: self.right_bracket.span().start },
                        CommentFlags::all(),
                    )
                } else {
                    // If there are no attributes, then no comments "after the last attribute" can apply.
                    false
                }
            };

            // Determine if the attribute list is empty and has comments inside the brackets.
            let is_empty_with_comments = || {
                self.attributes.is_empty()
                    && f.has_comment(self.hash_left_bracket.join(self.right_bracket), CommentFlags::all())
            };

            let should_break = self.attributes.len() > 3
                || has_comments_before_first()
                || has_comments_after_last()
                || is_empty_with_comments();

            let mut contents = vec![Document::String("#[")];
            let mut attributes = vec![];
            for attribute in self.attributes.iter() {
                attributes.push(Document::Group(Group::new(vec![attribute.format(f)])));
            }

            if should_break {
                let mut inner_conent = Document::join(attributes, Separator::CommaLine);
                inner_conent.insert(0, Document::Line(Line::softline()));
                if f.settings.trailing_comma {
                    inner_conent.push(Document::IfBreak(IfBreak::then(Document::String(","))));
                }

                contents.push(Document::Indent(inner_conent));
                if let Some(comments) = f.print_dangling_comments(self.hash_left_bracket.join(self.right_bracket), true)
                {
                    contents.push(comments);
                } else {
                    contents.push(Document::Line(Line::softline()));
                }
            } else {
                for (i, attribute) in attributes.into_iter().enumerate() {
                    if i != 0 {
                        contents.push(Document::String(", "));
                    }

                    contents.push(attribute);
                }
            }

            contents.push(Document::String("]"));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for PropertyHookAbstractBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHookAbstractBody, { Document::String(";") })
    }
}

impl<'a> Format<'a> for PropertyHookConcreteBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHookConcreteBody, {
            Document::Group(Group::new(vec![
                Document::space(),
                match self {
                    PropertyHookConcreteBody::Block(b) => b.format(f),
                    PropertyHookConcreteBody::Expression(b) => b.format(f),
                },
            ]))
        })
    }
}

impl<'a> Format<'a> for PropertyHookConcreteExpressionBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHookConcreteExpressionBody, {
            Document::Group(Group::new(vec![
                Document::String("=>"),
                Document::space(),
                self.expression.format(f),
                Document::String(";"),
            ]))
        })
    }
}

impl<'a> Format<'a> for PropertyHookBody {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHookBody, {
            match self {
                PropertyHookBody::Abstract(b) => b.format(f),
                PropertyHookBody::Concrete(b) => b.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for PropertyHook {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHook, {
            let attributes = misc::print_attribute_list_sequence(f, &self.attributes, false);
            let hook = {
                let mut contents = print_modifiers(f, &self.modifiers);
                if !contents.is_empty() {
                    contents.push(Document::space());
                }

                if self.ampersand.is_some() {
                    contents.push(Document::String("&"));
                }

                contents.push(self.name.format(f));
                if let Some(parameters) = &self.parameters {
                    contents.push(parameters.format(f));
                }

                contents.push(self.body.format(f));

                Document::Group(Group::new(contents))
            };

            if let Some(attributes) = attributes {
                Document::Group(Group::new(vec![attributes, hook]))
            } else {
                hook
            }
        })
    }
}

impl<'a> Format<'a> for PropertyHookList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyHookList, {
            let mut parts = vec![Document::String("{")];
            for hook in self.hooks.iter() {
                parts.push(Document::Indent(vec![Document::Line(Line::default()), hook.format(f)]));
            }

            parts.push(Document::Line(Line::default()));
            parts.push(Document::String("}"));

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for FunctionLikeParameterDefaultValue {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionLikeParameterDefaultValue, {
            Document::Group(Group::new(vec![Document::String("= "), self.value.format(f)]))
        })
    }
}

impl<'a> Format<'a> for FunctionLikeParameter {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionLikeParameter, {
            let attributes = print_attribute_list_sequence(f, &self.attributes, true);
            let parameter = {
                let mut contents = print_modifiers(f, &self.modifiers);
                if let Some(hint) = &self.hint {
                    if !contents.is_empty() {
                        contents.push(Document::space());
                    }

                    contents.push(hint.format(f));
                }

                if !contents.is_empty() {
                    contents.push(Document::space());
                }

                if self.ampersand.is_some() {
                    contents.push(Document::String("&"));
                }

                if self.ellipsis.is_some() {
                    contents.push(Document::String("..."));
                }

                contents.push(self.variable.format(f));
                if let Some(default_value) = &self.default_value {
                    contents.push(Document::space());
                    contents.push(default_value.format(f));
                }

                if let Some(hooks) = &self.hooks {
                    contents.push(Document::space());
                    contents.push(hooks.format(f));
                }

                Document::Group(Group::new(contents))
            };

            if let Some(attributes) = attributes {
                Document::Group(Group::new(vec![attributes, parameter]))
            } else {
                parameter
            }
        })
    }
}

impl<'a> Format<'a> for FunctionLikeParameterList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionLikeParameterList, { print_function_like_parameters(f, self) })
    }
}

impl<'a> Format<'a> for FunctionLikeReturnTypeHint {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionLikeReturnTypeHint, {
            Document::Group(Group::new(vec![Document::String(":"), Document::space(), self.hint.format(f)]))
        })
    }
}

impl<'a> Format<'a> for Function {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Function, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.push(Document::Line(Line::hardline()));
            }

            let mut signature = vec![];
            signature.push(self.function.format(f));
            signature.push(Document::space());
            if self.ampersand.is_some() {
                signature.push(Document::String("&"));
            }

            signature.push(self.name.format(f));
            signature.push(self.parameters.format(f));
            if let Some(return_type) = &self.return_type_hint {
                signature.push(return_type.format(f));
            }

            let signature_id = f.next_id();
            let signature_document = Document::Group(Group::new(signature).with_id(signature_id));

            Document::Group(Group::new(vec![
                Document::Group(Group::new(attributes)),
                signature_document,
                Document::Group(Group::new(vec![
                    match f.settings.function_brace_style {
                        BraceStyle::SameLine => Document::space(),
                        BraceStyle::NextLine => Document::IfBreak(
                            IfBreak::new(
                                Document::space(),
                                Document::Array(vec![Document::Line(Line::hardline()), Document::BreakParent]),
                            )
                            .with_id(signature_id),
                        ),
                    },
                    self.body.format(f),
                ])),
            ]))
        })
    }
}

impl<'a> Format<'a> for Try {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Try, {
            let mut parts = vec![self.r#try.format(f), Document::space(), self.block.format(f)];

            for clause in self.catch_clauses.iter() {
                parts.push(Document::space());
                parts.push(clause.format(f));
            }

            if let Some(clause) = &self.finally_clause {
                parts.push(Document::space());
                parts.push(clause.format(f));
            }

            Document::Group(Group::new(parts))
        })
    }
}

impl<'a> Format<'a> for TryCatchClause {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TryCatchClause, {
            let mut context = vec![self.hint.format(f)];
            if let Some(variable) = &self.variable {
                context.push(Document::space());
                context.push(variable.format(f));
            }

            Document::Group(Group::new(vec![
                self.catch.format(f),
                Document::space(),
                Document::String("("),
                Document::Group(Group::new(context)),
                Document::String(")"),
                Document::space(),
                self.block.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for TryFinallyClause {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TryFinallyClause, {
            Document::Group(Group::new(vec![self.finally.format(f), Document::space(), self.block.format(f)]))
        })
    }
}

impl<'a> Format<'a> for Global {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Global, {
            let mut contents = vec![self.global.format(f)];

            if self.variables.len() == 1 {
                contents.push(Document::space());
                contents.push(self.variables.as_slice()[0].format(f));
            } else if !self.variables.is_empty() {
                contents.push(Document::Indent(vec![Document::Line(Line::default())]));

                contents.push(Document::Indent(Document::join(
                    self.variables.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )));
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for StaticAbstractItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticAbstractItem, { self.variable.format(f) })
    }
}

impl<'a> Format<'a> for StaticConcreteItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticConcreteItem, {
            Document::Group(Group::new(vec![
                self.variable.format(f),
                Document::space(),
                Document::String("="),
                Document::space(),
                self.value.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for StaticItem {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticItem, {
            match self {
                StaticItem::Abstract(i) => i.format(f),
                StaticItem::Concrete(i) => i.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for Static {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Static, {
            let mut contents = vec![self.r#static.format(f)];

            if self.items.len() == 1 {
                contents.push(Document::space());
                contents.push(self.items.as_slice()[0].format(f));
            } else if !self.items.is_empty() {
                contents.push(Document::Indent(vec![Document::Line(Line::default())]));

                contents.push(Document::Indent(Document::join(
                    self.items.iter().map(|v| v.format(f)).collect(),
                    Separator::CommaLine,
                )));
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for Unset {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Unset, {
            let mut contents = vec![self.unset.format(f), Document::String("(")];

            if !self.values.is_empty() {
                let mut values =
                    Document::join(self.values.iter().map(|v| v.format(f)).collect(), Separator::CommaLine);

                if f.settings.trailing_comma {
                    values.push(Document::IfBreak(IfBreak::then(Document::String(","))));
                }

                values.insert(0, Document::Line(Line::softline()));

                contents.push(Document::Indent(values));
                contents.push(Document::Line(Line::softline()));
            }

            contents.push(Document::String(")"));
            contents.push(self.terminator.format(f));

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for Goto {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Goto, {
            Document::Group(Group::new(vec![
                self.goto.format(f),
                Document::space(),
                self.label.format(f),
                self.terminator.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for Label {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Label, { Document::Group(Group::new(vec![self.name.format(f), Document::String(":")])) })
    }
}

impl<'a> Format<'a> for HaltCompiler {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        f.scripting_mode = false;

        wrap!(f, self, HaltCompiler, {
            Document::Group(Group::new(vec![
                self.halt_compiler.format(f),
                Document::String("("),
                Document::String(")"),
                self.terminator.format(f),
            ]))
        })
    }
}
