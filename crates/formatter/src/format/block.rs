use fennec_ast::*;
use fennec_span::HasSpan;
use fennec_span::Span;

use crate::document::Document;
use crate::document::Group;
use crate::document::Line;
use crate::format::misc::print_token_with_indented_leading_comments;
use crate::format::sequence::SequenceFormatter;
use crate::format::statement;
use crate::format::Format;
use crate::token;
use crate::Formatter;

pub(super) fn print_block_of_nodes<'a, T: Format<'a> + HasSpan>(
    f: &mut Formatter<'a>,
    left_brace: Span,
    nodes: &'a Sequence<T>,
    right_brace: Span,
) -> Document<'a> {
    Document::Group(Group::new(vec![
        token!(f, left_brace, "{"),
        {
            if nodes.is_empty() {
                Document::empty()
            } else {
                Document::Indent(vec![
                    Document::Line(Line::hardline()),
                    SequenceFormatter::new().with_force_break(true).format(f, nodes),
                ])
            }
        },
        print_token_with_indented_leading_comments(f, right_brace, "}", true),
    ]))
}

pub(super) fn print_block<'a>(
    f: &mut Formatter<'a>,
    left_brace: &Span,
    stmts: &'a Sequence<Statement>,
    right_brace: &Span,
) -> Document<'a> {
    let mut contents = vec![];
    contents.push(token!(f, *left_brace, "{"));

    let has_body = stmts.iter().any(|stmt| !matches!(stmt, Statement::Noop(_)));
    let should_break;
    if has_body {
        let mut statements = statement::print_statement_sequence(f, stmts);
        statements.insert(0, Document::Line(Line::hardline()));
        contents.push(Document::Indent(statements));

        should_break = true;
    } else {
        let parent = f.parent_node();
        // in case the block is empty, we still want to add a new line
        // in some cases.
        should_break = match &parent {
            // functions, closures, and methods
            Node::Function(_) | Node::MethodBody(_) | Node::PropertyHookConcreteBody(_) => true,
            // try, catch, finally
            Node::Try(_) | Node::TryCatchClause(_) | Node::TryFinallyClause(_) => true,
            Node::Statement(_) => {
                let grand_parent = f.grandparent_node();

                match grand_parent {
                    // control structures
                    Some(
                        Node::ForBody(_)
                        | Node::WhileBody(_)
                        | Node::DoWhile(_)
                        | Node::If(_)
                        | Node::IfStatementBody(_)
                        | Node::IfStatementBodyElseClause(_)
                        | Node::IfStatementBodyElseIfClause(_)
                        | Node::ForeachBody(_),
                    ) => true,
                    _ => false,
                }
            }
            _ => false,
        };
    };

    contents.push(print_token_with_indented_leading_comments(f, *right_brace, "}", should_break));

    Document::Group(Group::new(contents).with_break(should_break))
}

pub(super) fn print_block_body<'a>(f: &mut Formatter<'a>, stmts: &'a Sequence<Statement>) -> Option<Document<'a>> {
    let has_body = stmts.iter().any(|stmt| !matches!(stmt, Statement::Noop(_)));

    if has_body {
        Some(Document::Array(statement::print_statement_sequence(f, stmts)))
    } else {
        None
    }
}
