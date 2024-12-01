use fennec_ast::*;
use fennec_span::HasSpan;

use crate::array;
use crate::binaryish::BinaryishOperator;
use crate::braced;
use crate::bracketed;
use crate::default_line;
use crate::document::Document;
use crate::document::Line;
use crate::empty_string;
use crate::format::binaryish;
use crate::format::binaryish::print_binaryish_expression;
use crate::format::call::collect_method_call_chain;
use crate::format::call::print_method_call_chain;
use crate::format::class_like::print_class_like_body;
use crate::format::delimited;
use crate::format::delimited::Delimiter;
use crate::format::misc::print_condition;
use crate::format::misc::print_modifiers;
use crate::format::misc::should_inline_expression;
use crate::format::sequence::TokenSeparatedSequenceFormatter;
use crate::format::Group;
use crate::format::IfBreak;
use crate::group;
use crate::hardline;
use crate::if_break;
use crate::indent;
use crate::indent_if_break;
use crate::settings::*;
use crate::space;
use crate::static_str;
use crate::token;
use crate::wrap;
use crate::Formatter;

use crate::format::Format;

impl<'a> Format<'a> for Expression {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        if let Expression::Parenthesized(parenthesized) = self {
            return parenthesized.expression.format(f);
        }

        wrap!(f, self, Expression, {
            match self {
                Expression::Referenced(referenced) => {
                    group!(token!(f, referenced.ampersand, "&"), referenced.expression.format(f))
                }
                Expression::Suppressed(suppressed) => {
                    group!(token!(f, suppressed.at, "@"), suppressed.expression.format(f))
                }
                Expression::Literal(literal) => literal.format(f),
                Expression::CompositeString(c) => c.format(f),
                Expression::ArithmeticOperation(op) => op.format(f),
                Expression::AssignmentOperation(op) => op.format(f),
                Expression::BitwiseOperation(op) => op.format(f),
                Expression::ComparisonOperation(op) => op.format(f),
                Expression::LogicalOperation(op) => op.format(f),
                Expression::CastOperation(op) => op.format(f),
                Expression::TernaryOperation(op) => op.format(f),
                Expression::CoalesceOperation(op) => op.format(f),
                Expression::ConcatOperation(op) => op.format(f),
                Expression::InstanceofOperation(op) => op.format(f),
                Expression::Array(array) => array.format(f),
                Expression::LegacyArray(legacy_array) => legacy_array.format(f),
                Expression::List(list) => list.format(f),
                Expression::ArrayAccess(a) => a.format(f),
                Expression::ArrayAppend(a) => a.format(f),
                Expression::AnonymousClass(c) => c.format(f),
                Expression::Closure(c) => c.format(f),
                Expression::ArrowFunction(a) => a.format(f),
                Expression::Variable(v) => v.format(f),
                Expression::Identifier(i) => i.format(f),
                Expression::Match(m) => m.format(f),
                Expression::Yield(y) => y.format(f),
                Expression::Construct(construct) => construct.format(f),
                Expression::Throw(t) => t.format(f),
                Expression::Clone(c) => c.format(f),
                Expression::Call(c) => {
                    if let Some(method_chain) = collect_method_call_chain(self) {
                        let chain_length = method_chain.calls.len();
                        if chain_length >= f.settings.method_chain_break_threshold {
                            // Chain is longer than threshold; format with line breaks
                            print_method_call_chain(&method_chain, f)
                        } else {
                            // Regular formatting
                            c.format(f)
                        }
                    } else {
                        c.format(f)
                    }
                }
                Expression::Access(a) => a.format(f),
                Expression::ClosureCreation(c) => c.format(f),
                Expression::Parent(k) => k.format(f),
                Expression::Static(k) => k.format(f),
                Expression::Self_(k) => k.format(f),
                Expression::Instantiation(i) => i.format(f),
                Expression::MagicConstant(c) => c.format(f),
                _ => unreachable!(),
            }
        })
    }
}

impl<'a> Format<'a> for Literal {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Literal, {
            match self {
                Literal::String(literal_string) => {
                    static_str!(f.lookup(&literal_string.value))
                }
                Literal::Integer(literal_integer) => {
                    static_str!(f.lookup(&literal_integer.raw))
                }
                Literal::Float(literal_float) => {
                    static_str!(f.lookup(&literal_float.raw))
                }
                Literal::True(keyword) => keyword.format(f),
                Literal::False(keyword) => keyword.format(f),
                Literal::Null(keyword) => keyword.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for Variable {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Variable, {
            match self {
                Variable::Direct(var) => var.format(f),
                Variable::Indirect(var) => var.format(f),
                Variable::Nested(var) => var.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for IndirectVariable {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, IndirectVariable, {
            group!(token!(f, self.dollar_left_brace, "${"), self.expression.format(f), token!(f, self.right_brace, "}"))
        })
    }
}

impl<'a> Format<'a> for DirectVariable {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, DirectVariable, { static_str!(f.lookup(&self.name)) })
    }
}

impl<'a> Format<'a> for NestedVariable {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, NestedVariable, { group!(token!(f, self.dollar, "$"), self.variable.format(f),) })
    }
}

impl<'a> Format<'a> for Array {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Array, {
            let formatter =
                TokenSeparatedSequenceFormatter::new(",").with_trailing_separator(f.settings.trailing_comma);

            match f.settings.array_style {
                ArrayStyle::Short => {
                    let delimiter = Delimiter::Brackets(self.left_bracket, self.right_bracket);

                    formatter.format_with_delimiter(f, &self.elements, delimiter, f.settings.preserve_broken_arrays)
                }
                ArrayStyle::Long => {
                    let delimiter = Delimiter::Parentheses(self.left_bracket, self.right_bracket);

                    Document::Group(Group::new(vec![
                        Document::String("array"),
                        formatter.format_with_delimiter(
                            f,
                            &self.elements,
                            delimiter,
                            f.settings.preserve_broken_arrays,
                        ),
                    ]))
                }
            }
        })
    }
}

impl<'a> Format<'a> for LegacyArray {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LegacyArray, {
            let formatter =
                TokenSeparatedSequenceFormatter::new(",").with_trailing_separator(f.settings.trailing_comma);

            match f.settings.array_style {
                ArrayStyle::Short => {
                    let delimiter = Delimiter::Brackets(self.left_parenthesis, self.right_parenthesis);

                    formatter.format_with_delimiter(f, &self.elements, delimiter, f.settings.preserve_broken_arrays)
                }
                ArrayStyle::Long => {
                    let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);

                    Document::Group(Group::new(vec![
                        self.array.format(f),
                        formatter.format_with_delimiter(
                            f,
                            &self.elements,
                            delimiter,
                            f.settings.preserve_broken_arrays,
                        ),
                    ]))
                }
            }
        })
    }
}

impl<'a> Format<'a> for List {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, List, {
            let formatter =
                TokenSeparatedSequenceFormatter::new(",").with_trailing_separator(f.settings.trailing_comma);

            match f.settings.list_style {
                ListStyle::Short => {
                    let delimiter = Delimiter::Brackets(self.left_parenthesis, self.right_parenthesis);

                    formatter.format_with_delimiter(f, &self.elements, delimiter, f.settings.preserve_broken_arrays)
                }
                ListStyle::Long => {
                    let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);

                    Document::Group(Group::new(vec![
                        self.list.format(f),
                        formatter.format_with_delimiter(
                            f,
                            &self.elements,
                            delimiter,
                            f.settings.preserve_broken_arrays,
                        ),
                    ]))
                }
            }
        })
    }
}

impl<'a> Format<'a> for ArrayElement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArrayElement, {
            match self {
                ArrayElement::KeyValue(e) => e.format(f),
                ArrayElement::Value(e) => e.format(f),
                ArrayElement::Variadic(e) => e.format(f),
                ArrayElement::Missing(e) => e.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for KeyValueArrayElement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, KeyValueArrayElement, {
            group!(
                self.key.format(f),
                space!(),
                token!(f, self.double_arrow, "=>"),
                indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f))
            )
        })
    }
}

impl<'a> Format<'a> for ValueArrayElement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ValueArrayElement, { self.value.format(f) })
    }
}

impl<'a> Format<'a> for VariadicArrayElement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, VariadicArrayElement, { array!(token!(f, self.ellipsis, "..."), self.value.format(f)) })
    }
}

impl<'a> Format<'a> for MissingArrayElement {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MissingArrayElement, { empty_string!() })
    }
}

impl<'a> Format<'a> for Construct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Construct, {
            match self {
                Construct::Isset(c) => c.format(f),
                Construct::Empty(c) => c.format(f),
                Construct::Eval(c) => c.format(f),
                Construct::Include(c) => c.format(f),
                Construct::IncludeOnce(c) => c.format(f),
                Construct::Require(c) => c.format(f),
                Construct::RequireOnce(c) => c.format(f),
                Construct::Print(c) => c.format(f),
                Construct::Exit(c) => c.format(f),
                Construct::Die(c) => c.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for IssetConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, IssetConstruct, {
            let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);
            let formatter = TokenSeparatedSequenceFormatter::new(",").with_trailing_separator(false);

            // todo: add an setting to control preserve_broken_constructs
            Document::Group(Group::new(vec![
                self.isset.format(f),
                formatter.format_with_delimiter(f, &self.values, delimiter, false),
            ]))
        })
    }
}

impl<'a> Format<'a> for EmptyConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EmptyConstruct, {
            let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);
            let formatter = |f: &mut Formatter<'a>| (Document::Group(Group::new(vec![self.value.format(f)])), false);

            group!(self.empty.format(f), delimited::format_delimited_group(f, delimiter, formatter, false))
        })
    }
}

impl<'a> Format<'a> for EvalConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, EvalConstruct, {
            let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);
            let formatter = |f: &mut Formatter<'a>| (Document::Group(Group::new(vec![self.value.format(f)])), false);

            group!(self.eval.format(f), delimited::format_delimited_group(f, delimiter, formatter, false))
        })
    }
}

impl<'a> Format<'a> for IncludeConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, IncludeConstruct, {
            group!(self.include.format(f), indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f)))
        })
    }
}

impl<'a> Format<'a> for IncludeOnceConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, IncludeOnceConstruct, {
            group!(
                self.include_once.format(f),
                indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f))
            )
        })
    }
}

impl<'a> Format<'a> for RequireConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, RequireConstruct, {
            group!(self.require.format(f), indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f)))
        })
    }
}

impl<'a> Format<'a> for RequireOnceConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, RequireOnceConstruct, {
            group!(
                self.require_once.format(f),
                indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f))
            )
        })
    }
}

impl<'a> Format<'a> for PrintConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PrintConstruct, {
            group!(self.print.format(f), indent_if_break!(if_break!(default_line!(), space!()), self.value.format(f)))
        })
    }
}

impl<'a> Format<'a> for ExitConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ExitConstruct, {
            // TODO: add support to check what syntax to use `exit` or `die`
            // and whether to use parentheses or not if there are no arguments
            match self.arguments {
                Some(ref arguments) => {
                    group![self.exit.format(f), arguments.format(f)]
                }
                None => self.exit.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for DieConstruct {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        // TODO: add support to check what syntax to use `exit` or `die`
        // and whether to use parentheses or not if there are no arguments
        wrap!(f, self, DieConstruct, {
            match self.arguments {
                Some(ref arguments) => {
                    array![self.die.format(f), arguments.format(f)]
                }
                None => self.die.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ArgumentList {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArgumentList, {
            let inline;
            if f.settings.inline_single_breaking_argument && 1 == self.arguments.len() {
                inline = if let Some(argument) = self.arguments.first() {
                    if f.has_surrounding_comments(argument.span()) {
                        false
                    } else {
                        should_inline_expression(f, argument.value())
                    }
                } else {
                    false
                };
            } else {
                inline = false;
            }

            let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);

            TokenSeparatedSequenceFormatter::new(",")
                .with_trailing_separator(f.settings.trailing_comma)
                .with_force_inline(inline)
                .format_with_delimiter(f, &self.arguments, delimiter, f.settings.preserve_broken_argument_lists)
        })
    }
}

impl<'a> Format<'a> for Argument {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Argument, {
            match self {
                Argument::Positional(a) => a.format(f),
                Argument::Named(a) => a.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for PositionalArgument {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PositionalArgument, {
            match self.ellipsis {
                Some(span) => Document::Group(Group::new(vec![token!(f, span, "..."), self.value.format(f)])),
                None => Document::Group(Group::new(vec![self.value.format(f)])),
            }
        })
    }
}

impl<'a> Format<'a> for NamedArgument {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, NamedArgument, {
            match self.ellipsis {
                Some(span) => Document::Group(Group::new(vec![
                    self.name.format(f),
                    token!(f, self.colon, ":"),
                    space!(),
                    token!(f, span, "..."),
                    self.value.format(f),
                ])),
                None => Document::Group(Group::new(vec![
                    self.name.format(f),
                    token!(f, self.colon, ":"),
                    space!(),
                    self.value.format(f),
                ])),
            }
        })
    }
}

impl<'a> Format<'a> for ConcatOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ConcatOperation, {
            binaryish::print_binaryish_expression(f, &self.lhs, BinaryishOperator::Concat(self.dot), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for ArithmeticOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArithmeticOperation, {
            match self {
                ArithmeticOperation::Prefix(o) => o.format(f),
                ArithmeticOperation::Infix(o) => o.format(f),
                ArithmeticOperation::Postfix(o) => o.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ArithmeticPrefixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArithmeticPrefixOperation, {
            let operator = match self.operator {
                ArithmeticPrefixOperator::Decrement(span) => token!(f, span, "--"),
                ArithmeticPrefixOperator::Increment(span) => token!(f, span, "++"),
                ArithmeticPrefixOperator::Minus(span) => token!(f, span, "-"),
                ArithmeticPrefixOperator::Plus(span) => token!(f, span, "+"),
            };

            group!(operator, self.value.format(f))
        })
    }
}

impl<'a> Format<'a> for ArithmeticInfixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArithmeticInfixOperation, {
            binaryish::print_binaryish_expression(f, &self.lhs, BinaryishOperator::from(self.operator), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for ArithmeticPostfixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArithmeticPostfixOperation, {
            let operator = match self.operator {
                ArithmeticPostfixOperator::Decrement(span) => token!(f, span, "--"),
                ArithmeticPostfixOperator::Increment(span) => token!(f, span, "++"),
            };

            array!(self.value.format(f), operator)
        })
    }
}

impl<'a> Format<'a> for AssignmentOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, AssignmentOperation, {
            let left = self.lhs.format(f);

            let operator = match self.operator {
                AssignmentOperator::Assign(span) => token!(f, span, "="),
                AssignmentOperator::Addition(span) => token!(f, span, "+="),
                AssignmentOperator::Subtraction(span) => token!(f, span, "-="),
                AssignmentOperator::Multiplication(span) => token!(f, span, "*="),
                AssignmentOperator::Division(span) => token!(f, span, "/="),
                AssignmentOperator::Modulo(span) => token!(f, span, "%="),
                AssignmentOperator::Exponentiation(span) => token!(f, span, "**="),
                AssignmentOperator::Concat(span) => token!(f, span, ".="),
                AssignmentOperator::BitwiseAnd(span) => token!(f, span, "&="),
                AssignmentOperator::BitwiseOr(span) => token!(f, span, "|="),
                AssignmentOperator::BitwiseXor(span) => token!(f, span, "^="),
                AssignmentOperator::LeftShift(span) => token!(f, span, "<<="),
                AssignmentOperator::RightShift(span) => token!(f, span, ">>="),
                AssignmentOperator::Coalesce(span) => token!(f, span, "??="),
            };

            if f.has_leading_multi_line_comments(self.rhs.span()) {
                Document::Group(Group::new(vec![
                    left,
                    Document::space(),
                    operator,
                    Document::Indent(vec![Document::Line(Line::hardline()), self.rhs.format(f)]),
                ]))
            } else {
                Document::Group(Group::new(vec![
                    left,
                    Document::space(),
                    operator,
                    Document::space(),
                    self.rhs.format(f),
                ]))
            }
        })
    }
}

impl<'a> Format<'a> for BitwiseOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, BitwiseOperation, {
            match self {
                BitwiseOperation::Prefix(o) => o.format(f),
                BitwiseOperation::Infix(o) => o.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for BitwisePrefixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, BitwisePrefixOperation, {
            let operator = match self.operator {
                BitwisePrefixOperator::Not(span) => token!(f, span, "~"),
            };

            group!(operator, self.value.format(f))
        })
    }
}

impl<'a> Format<'a> for BitwiseInfixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, BitwiseInfixOperation, {
            binaryish::print_binaryish_expression(f, &self.lhs, BinaryishOperator::from(self.operator), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for ClosureUseClauseVariable {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClosureUseClauseVariable, {
            if let Some(span) = self.ampersand {
                group!(token!(f, span, "&"), self.variable.format(f))
            } else {
                self.variable.format(f)
            }
        })
    }
}

impl<'a> Format<'a> for ClosureUseClause {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClosureUseClause, {
            let delimiter = Delimiter::Parentheses(self.left_parenthesis, self.right_parenthesis);
            let formatter =
                TokenSeparatedSequenceFormatter::new(",").with_trailing_separator(f.settings.trailing_comma);

            group!(
                self.r#use.format(f),
                {
                    if f.settings.space_after_closure_use {
                        space!()
                    } else {
                        empty_string!()
                    }
                },
                formatter.format_with_delimiter(f, &self.variables, delimiter, true)
            )
        })
    }
}

impl<'a> Format<'a> for Closure {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Closure, {
            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.extend(hardline!());
            }

            let mut signature = vec![];
            if let Some(s) = &self.r#static {
                signature.push(s.format(f));
                signature.push(space!());
            }

            signature.push(self.function.format(f));
            if f.settings.space_before_closure_params {
                signature.push(space!());
            }

            if let Some(span) = self.ampersand {
                signature.push(token!(f, span, "&"));
            }

            signature.push(self.parameters.format(f));
            if let Some(u) = &self.use_clause {
                signature.push(space!());
                signature.push(u.format(f));
            }

            if let Some(h) = &self.return_type_hint {
                signature.push(h.format(f));
            }

            let (signature_id, signature_document) = group!(f, @signature);

            let mut body = vec![];
            body.push(match f.settings.closure_brace_style {
                BraceStyle::SameLine => {
                    space!()
                }
                BraceStyle::NextLine => {
                    if_break!(space!(), Document::Line(Line::hardline()), Some(signature_id))
                }
            });
            body.push(self.body.format(f));

            group!(group!(@attributes), signature_document, group!(@body))
        })
    }
}

impl<'a> Format<'a> for ArrowFunction {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArrowFunction, {
            let mut parts = vec![];
            for attribute_list in self.attributes.iter() {
                parts.push(attribute_list.format(f));
                parts.extend(hardline!());
            }

            if let Some(s) = &self.r#static {
                parts.push(s.format(f));
                parts.push(space!());
            }

            parts.push(self.r#fn.format(f));
            if f.settings.space_before_arrow_function_params {
                parts.push(space!());
            }

            if let Some(span) = self.ampersand {
                parts.push(token!(f, span, "&"));
            }

            parts.push(self.parameters.format(f));
            if let Some(h) = &self.return_type_hint {
                parts.push(h.format(f));
            }

            parts.push(if_break!(indent!(default_line!()), space!()));
            parts.push(token!(f, self.arrow, "=>"));
            parts.push(space!());

            parts.push(self.expression.format(f));

            group!(@ parts)
        })
    }
}

impl<'a> Format<'a> for ClassLikeMemberSelector {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeMemberSelector, {
            match self {
                ClassLikeMemberSelector::Identifier(s) => s.format(f),
                ClassLikeMemberSelector::Variable(s) => s.format(f),
                ClassLikeMemberSelector::Expression(s) => s.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ClassLikeMemberExpressionSelector {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeMemberExpressionSelector, { braced!(self.expression.format(f)) })
    }
}

impl<'a> Format<'a> for ClassLikeConstantSelector {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassLikeConstantSelector, {
            match self {
                ClassLikeConstantSelector::Identifier(s) => s.format(f),
                ClassLikeConstantSelector::Expression(s) => s.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for Access {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Access, {
            match self {
                Access::Property(a) => a.format(f),
                Access::NullSafeProperty(a) => a.format(f),
                Access::StaticProperty(a) => a.format(f),
                Access::ClassConstant(a) => a.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for PropertyAccess {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, PropertyAccess, {
            Document::Group(Group::new(vec![
                self.object.format(f),
                token!(f, self.arrow, "->"),
                self.property.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for NullSafePropertyAccess {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, NullSafePropertyAccess, {
            Document::Group(Group::new(vec![
                self.object.format(f),
                token!(f, self.question_mark_arrow, "?->"),
                self.property.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for StaticPropertyAccess {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticPropertyAccess, {
            Document::Group(Group::new(vec![
                self.class.format(f),
                token!(f, self.double_colon, "::"),
                self.property.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for ClassConstantAccess {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClassConstantAccess, {
            Document::Group(Group::new(vec![
                self.class.format(f),
                token!(f, self.double_colon, "::"),
                self.constant.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for Call {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Call, {
            match self {
                Call::Function(c) => c.format(f),
                Call::Method(c) => c.format(f),
                Call::NullSafeMethod(c) => c.format(f),
                Call::StaticMethod(c) => c.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for FunctionCall {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionCall, { group!(self.function.format(f), self.arguments.format(f)) })
    }
}

impl<'a> Format<'a> for MethodCall {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MethodCall, {
            Document::Group(Group::new(vec![
                self.object.format(f),
                token!(f, self.arrow, "->"),
                self.method.format(f),
                self.arguments.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for NullSafeMethodCall {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, NullSafeMethodCall, {
            Document::Group(Group::new(vec![
                self.object.format(f),
                token!(f, self.question_mark_arrow, "?->"),
                self.method.format(f),
                self.arguments.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for StaticMethodCall {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticMethodCall, {
            Document::Group(Group::new(vec![
                self.class.format(f),
                token!(f, self.double_colon, "::"),
                self.method.format(f),
                self.arguments.format(f),
            ]))
        })
    }
}

impl<'a> Format<'a> for CastOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, CastOperation, { group!(self.operator.format(f), space!(), self.value.format(f)) })
    }
}

impl<'a> Format<'a> for CastOperator {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, CastOperator, {
            let op = |n: &str| match f.settings.keyword_case {
                CasingStyle::Lowercase => {
                    static_str!(f.as_str(format!("({})", n.to_lowercase())))
                }
                CasingStyle::Uppercase => {
                    static_str!(f.as_str(format!("({})", n.to_uppercase())))
                }
            };

            match self {
                CastOperator::Array(_, _) => op("array"),
                CastOperator::Bool(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("bool")
                    } else {
                        match f.settings.bool_cast {
                            BoolCastOperator::Bool => op("bool"),
                            BoolCastOperator::Boolean => op("boolean"),
                        }
                    }
                }
                CastOperator::Boolean(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("boolean")
                    } else {
                        match f.settings.bool_cast {
                            BoolCastOperator::Bool => op("bool"),
                            BoolCastOperator::Boolean => op("boolean"),
                        }
                    }
                }
                CastOperator::Double(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("float")
                    } else {
                        match f.settings.float_cast {
                            FloatCastOperator::Float => op("float"),
                            FloatCastOperator::Double => op("double"),
                            FloatCastOperator::Real => op("real"),
                        }
                    }
                }
                CastOperator::Real(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("real")
                    } else {
                        match f.settings.float_cast {
                            FloatCastOperator::Float => op("float"),
                            FloatCastOperator::Double => op("double"),
                            FloatCastOperator::Real => op("real"),
                        }
                    }
                }
                CastOperator::Float(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("float")
                    } else {
                        match f.settings.float_cast {
                            FloatCastOperator::Float => op("float"),
                            FloatCastOperator::Double => op("double"),
                            FloatCastOperator::Real => op("real"),
                        }
                    }
                }
                CastOperator::Int(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("int")
                    } else {
                        match f.settings.int_cast {
                            IntCastOperator::Int => op("int"),
                            IntCastOperator::Integer => op("integer"),
                        }
                    }
                }
                CastOperator::Integer(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("integer")
                    } else {
                        match f.settings.int_cast {
                            IntCastOperator::Int => op("int"),
                            IntCastOperator::Integer => op("integer"),
                        }
                    }
                }
                CastOperator::Object(_, _) => op("object"),
                CastOperator::Unset(_, _) => op("unset"),
                CastOperator::String(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("string")
                    } else {
                        match f.settings.string_cast {
                            StringCastOperator::String => op("string"),
                            StringCastOperator::Binary => op("binary"),
                        }
                    }
                }
                CastOperator::Binary(_, _) => {
                    if f.settings.leave_casts_as_is {
                        op("binary")
                    } else {
                        match f.settings.string_cast {
                            StringCastOperator::String => op("string"),
                            StringCastOperator::Binary => op("binary"),
                        }
                    }
                }
            }
        })
    }
}

impl<'a> Format<'a> for ComparisonOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ComparisonOperation, {
            binaryish::print_binaryish_expression(f, &self.lhs, BinaryishOperator::from(self.operator), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for LogicalOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LogicalOperation, {
            match self {
                LogicalOperation::Prefix(o) => o.format(f),
                LogicalOperation::Infix(o) => o.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for LogicalPrefixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LogicalPrefixOperation, {
            group!(
                match self.operator {
                    LogicalPrefixOperator::Not(span) => token!(f, span, "!"),
                },
                self.value.format(f),
            )
        })
    }
}

impl<'a> Format<'a> for LogicalInfixOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LogicalInfixOperation, {
            binaryish::print_binaryish_expression(f, &self.lhs, BinaryishOperator::from(self.operator), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for Throw {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Throw, { group!(self.throw.format(f), space!(), self.exception.format(f)) })
    }
}

impl<'a> Format<'a> for Instantiation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Instantiation, {
            group!(
                self.new.format(f),
                space!(),
                self.class.format(f),
                if let Some(arguments) = &self.arguments { arguments.format(f) } else { static_str!("()") }
            )
        })
    }
}

impl<'a> Format<'a> for ArrayAccess {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArrayAccess, { array!(self.array.format(f), bracketed!(self.index.format(f))) })
    }
}

impl<'a> Format<'a> for ArrayAppend {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ArrayAppend, {
            array!(self.array.format(f), token!(f, self.left_bracket, "["), token!(f, self.right_bracket, "]"))
        })
    }
}

impl<'a> Format<'a> for MatchArm {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MatchArm, {
            match self {
                MatchArm::Expression(a) => a.format(f),
                MatchArm::Default(a) => a.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for MatchDefaultArm {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MatchDefaultArm, {
            group!(
                self.default.format(f),
                if_break!(default_line!(), space!()),
                token!(f, self.arrow, "=>"),
                space!(),
                indent_if_break!(self.expression.format(f))
            )
        })
    }
}

impl<'a> Format<'a> for MatchExpressionArm {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MatchExpressionArm, {
            let len = self.conditions.len();
            let mut left = vec![];
            for (i, condition) in self.conditions.iter().enumerate() {
                left.push(condition.format(f));
                if i != (len - 1) {
                    left.push(static_str!(","));
                    left.push(if_break!(default_line!(), space!()));
                } else if f.settings.trailing_comma {
                    left.push(if_break!(static_str!(",")));
                }
            }

            left.push(indent_if_break!(if_break!(default_line!(), space!()), token!(f, self.arrow, "=>")));

            let right = vec![space!(), self.expression.format(f)];

            array!(group!(@left), group!(@right))
        })
    }
}

impl<'a> Format<'a> for Match {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Match, {
            let mut contents = vec![
                self.r#match.format(f),
                space!(),
                print_condition(f, self.left_parenthesis, &self.expression, self.right_parenthesis),
            ];

            match f.settings.control_brace_style {
                BraceStyle::SameLine => {
                    contents.push(Document::space());
                }
                BraceStyle::NextLine => {
                    contents.push(Document::IfBreak(IfBreak::new(Document::space(), Document::Line(Line::default()))));
                }
            };

            let delimiter = Delimiter::Braces(self.left_brace, self.right_brace);

            contents.push(
                TokenSeparatedSequenceFormatter::new(",")
                    .with_trailing_separator(f.settings.trailing_comma)
                    .with_force_break(true)
                    .format_with_delimiter(f, &self.arms, delimiter, false),
            );

            Document::Group(Group::new(contents))
        })
    }
}

impl<'a> Format<'a> for CoalesceOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, CoalesceOperation, {
            print_binaryish_expression(f, &self.lhs, BinaryishOperator::Coalesce(self.double_question_mark), &self.rhs)
        })
    }
}

impl<'a> Format<'a> for InstanceofOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, InstanceofOperation, {
            group!(self.lhs.format(f), space!(), self.instanceof.format(f), space!(), self.rhs.format(f))
        })
    }
}

impl<'a> Format<'a> for TernaryOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, TernaryOperation, {
            match self {
                TernaryOperation::Conditional(t) => t.format(f),
                TernaryOperation::Elvis(t) => t.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ConditionalTernaryOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ConditionalTernaryOperation, {
            match &self.then {
                Some(then) => {
                    group!(
                        self.condition.format(f),
                        indent_if_break!(
                            if_break!(default_line!(), space!()),
                            token!(f, self.question_mark, "?"),
                            space!()
                        ),
                        then.format(f),
                        indent_if_break!(if_break!(default_line!(), space!()), token!(f, self.colon, ":"), space!()),
                        self.r#else.format(f)
                    )
                }
                None => {
                    group!(
                        self.condition.format(f),
                        indent_if_break!(
                            if_break!(default_line!(), space!()),
                            token!(f, self.question_mark, "?"),
                            token!(f, self.colon, ":"),
                            space!()
                        ),
                        self.r#else.format(f)
                    )
                }
            }
        })
    }
}

impl<'a> Format<'a> for ElvisTernaryOperation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ElvisTernaryOperation, {
            group!(
                self.condition.format(f),
                indent_if_break!(
                    if_break!(default_line!(), space!()),
                    token!(f, self.question_mark_colon, "?:"),
                    space!()
                ),
                self.r#else.format(f)
            )
        })
    }
}

impl<'a> Format<'a> for CompositeString {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, CompositeString, {
            match self {
                CompositeString::ShellExecute(s) => s.format(f),
                CompositeString::Interpolated(s) => s.format(f),
                CompositeString::Document(s) => s.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for DocumentString {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, DocumentString, {
            let mut parts = vec![static_str!("<<<")];
            match self.kind {
                DocumentKind::Heredoc => {
                    parts.push(static_str!(f.lookup(&self.label)));
                }
                DocumentKind::Nowdoc => {
                    parts.push(static_str!("'"));
                    parts.push(static_str!(f.lookup(&self.label)));
                    parts.push(static_str!("'"));
                }
            }

            let indent = match self.indentation {
                DocumentIndentation::None => 0,
                DocumentIndentation::Whitespace(n) => n,
                DocumentIndentation::Tab(n) => n,
                DocumentIndentation::Mixed(t, w) => t + w,
            };

            parts.push(default_line!());
            for part in self.parts.iter() {
                let formatted = match part {
                    StringPart::Literal(l) => {
                        let lines = Formatter::split_lines(f.lookup(&l.value));
                        let mut parts = vec![];
                        for line in lines {
                            let line = Formatter::skip_leading_whitespace_up_to(line, indent);

                            parts.push(static_str!(line));
                            parts.push(default_line!());
                        }

                        array!(@parts)
                    }
                    _ => part.format(f),
                };

                parts.push(formatted);
            }
            parts.push(static_str!(f.lookup(&self.label)));
            parts.push(Document::BreakParent);

            group!(@parts)
        })
    }
}

impl<'a> Format<'a> for InterpolatedString {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, InterpolatedString, {
            let mut parts = vec![static_str!("\"")];

            for part in self.parts.iter() {
                parts.push(part.format(f));
            }

            parts.push(static_str!("\""));

            group!(@parts)
        })
    }
}

impl<'a> Format<'a> for ShellExecuteString {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ShellExecuteString, {
            let mut parts = vec![static_str!("`")];

            for part in self.parts.iter() {
                parts.push(part.format(f));
            }

            parts.push(static_str!("`"));

            group!(@parts)
        })
    }
}

impl<'a> Format<'a> for StringPart {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StringPart, {
            match self {
                StringPart::Literal(s) => s.format(f),
                StringPart::Expression(s) => s.format(f),
                StringPart::BracedExpression(s) => s.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for LiteralStringPart {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, LiteralStringPart, { static_str!(f.lookup(&self.value)) })
    }
}

impl<'a> Format<'a> for BracedExpressionStringPart {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, BracedExpressionStringPart, {
            group!(token!(f, self.left_brace, "{"), self.expression.format(f), token!(f, self.right_brace, "}"))
        })
    }
}

impl<'a> Format<'a> for Yield {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Yield, {
            match self {
                Yield::Value(y) => y.format(f),
                Yield::Pair(y) => y.format(f),
                Yield::From(y) => y.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for YieldValue {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, YieldValue, {
            match &self.value {
                Some(v) => {
                    group!(self.r#yield.format(f), space!(), v.format(f))
                }
                None => self.r#yield.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for YieldPair {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, YieldPair, {
            group!(
                self.r#yield.format(f),
                space!(),
                self.key.format(f),
                space!(),
                token!(f, self.arrow, "=>"),
                space!(),
                self.value.format(f)
            )
        })
    }
}

impl<'a> Format<'a> for YieldFrom {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, YieldFrom, {
            group!(self.r#yield.format(f), space!(), self.from.format(f), space!(), self.iterator.format(f))
        })
    }
}

impl<'a> Format<'a> for Clone {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, Clone, { group!(self.clone.format(f), space!(), self.object.format(f)) })
    }
}

impl<'a> Format<'a> for MagicConstant {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MagicConstant, {
            match &self {
                MagicConstant::Line(i) => i.format(f),
                MagicConstant::File(i) => i.format(f),
                MagicConstant::Directory(i) => i.format(f),
                MagicConstant::Trait(i) => i.format(f),
                MagicConstant::Method(i) => i.format(f),
                MagicConstant::Function(i) => i.format(f),
                MagicConstant::Property(i) => i.format(f),
                MagicConstant::Namespace(i) => i.format(f),
                MagicConstant::Class(i) => i.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for ClosureCreation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, ClosureCreation, {
            match &self {
                ClosureCreation::Function(c) => c.format(f),
                ClosureCreation::Method(c) => c.format(f),
                ClosureCreation::StaticMethod(c) => c.format(f),
            }
        })
    }
}

impl<'a> Format<'a> for FunctionClosureCreation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, FunctionClosureCreation, {
            group!(
                self.function.format(f),
                token!(f, self.left_parenthesis, "("),
                token!(f, self.ellipsis, "..."),
                token!(f, self.right_parenthesis, ")"),
            )
        })
    }
}

impl<'a> Format<'a> for MethodClosureCreation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, MethodClosureCreation, {
            group!(
                self.object.format(f),
                static_str!("->"),
                self.method.format(f),
                token!(f, self.left_parenthesis, "("),
                token!(f, self.ellipsis, "..."),
                token!(f, self.right_parenthesis, ")"),
            )
        })
    }
}

impl<'a> Format<'a> for StaticMethodClosureCreation {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, StaticMethodClosureCreation, {
            group!(
                self.class.format(f),
                static_str!("::"),
                self.method.format(f),
                token!(f, self.left_parenthesis, "("),
                token!(f, self.ellipsis, "..."),
                token!(f, self.right_parenthesis, ")"),
            )
        })
    }
}

impl<'a> Format<'a> for AnonymousClass {
    fn format(&'a self, f: &mut Formatter<'a>) -> Document<'a> {
        wrap!(f, self, AnonymousClass, {
            let mut initialization = vec![];
            initialization.push(self.new.format(f));
            initialization.push(if self.attributes.is_empty() { space!() } else { indent!(default_line!()) });

            let mut attributes = vec![];
            for attribute_list in self.attributes.iter() {
                attributes.push(attribute_list.format(f));
                attributes.extend(hardline!());
            }

            let mut signature = vec![];
            signature.push(self.new.format(f));
            signature.push(space!());
            signature.push(print_modifiers(f, &self.modifiers));
            signature.push(self.class.format(f));

            if let Some(arguments) = &self.arguments {
                signature.push(arguments.format(f));
            }

            if let Some(extends) = &self.extends {
                signature.push(space!());
                signature.push(extends.format(f));
            }

            if let Some(implements) = &self.implements {
                signature.push(space!());
                signature.push(implements.format(f));
            }

            let (signature_id, signature_document) = group!(f, @signature);

            let mut body = vec![];
            body.push(match f.settings.classlike_brace_style {
                BraceStyle::SameLine => {
                    space!()
                }
                BraceStyle::NextLine => {
                    if_break!(space!(), array!(@hardline!()), Some(signature_id))
                }
            });
            body.push(print_class_like_body(f, &self.left_brace, &self.members, &self.right_brace));

            group!(group!(@attributes), signature_document, group!(@body), Document::BreakParent)
        })
    }
}
