use oxc_ast::{
    ast::{ObjectAssignmentTarget, ObjectExpression, ObjectPattern, TSTypeLiteral, WithClause},
    AstKind,
};
use oxc_span::Span;

use crate::{
    format::{function_parameters, misc},
    ir::{Doc, DocBuilder},
    p_vec, Format, Prettier,
};

#[derive(Debug, Clone, Copy)]
pub enum ObjectLike<'a, 'b> {
    Expression(&'b ObjectExpression<'a>),
    AssignmentTarget(&'b ObjectAssignmentTarget<'a>),
    Pattern(&'b ObjectPattern<'a>),
    WithClause(&'b WithClause<'a>),
    TSTypeLiteral(&'b TSTypeLiteral<'a>),
}

impl<'a, 'b> ObjectLike<'a, 'b> {
    fn len(&self) -> usize {
        match self {
            Self::Expression(expr) => expr.properties.len(),
            Self::AssignmentTarget(target) => target.properties.len(),
            Self::Pattern(object) => object.properties.len(),
            Self::WithClause(attributes) => attributes.with_entries.len(),
            Self::TSTypeLiteral(literal) => literal.members.len(),
        }
    }

    fn has_rest(&self) -> bool {
        match self {
            Self::Expression(expr) => false,
            Self::AssignmentTarget(target) => target.rest.is_some(),
            Self::Pattern(object) => object.rest.is_some(),
            Self::WithClause(_) | Self::TSTypeLiteral(_) => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Expression(object) => object.properties.is_empty(),
            Self::AssignmentTarget(object) => object.is_empty(),
            Self::Pattern(object) => object.is_empty(),
            Self::WithClause(attributes) => attributes.with_entries.is_empty(),
            Self::TSTypeLiteral(literal) => literal.members.is_empty(),
        }
    }

    fn is_object_pattern(&self) -> bool {
        matches!(self, Self::Pattern(_))
    }

    fn span(&self) -> Span {
        match self {
            Self::Expression(object) => object.span,
            Self::AssignmentTarget(object) => object.span,
            Self::Pattern(object) => object.span,
            Self::WithClause(attributes) => attributes.span,
            Self::TSTypeLiteral(literal) => literal.span,
        }
    }

    fn iter(&'b self, p: &'b mut Prettier<'a>) -> Box<dyn Iterator<Item = Doc<'a>> + 'b> {
        match self {
            Self::Expression(object) => {
                Box::new(object.properties.iter().map(|prop| prop.format(p)))
            }
            Self::AssignmentTarget(object) => {
                Box::new(object.properties.iter().map(|prop| prop.format(p)))
            }
            Self::Pattern(object) => Box::new(object.properties.iter().map(|prop| prop.format(p))),
            Self::WithClause(attributes) => {
                Box::new(attributes.with_entries.iter().map(|entry| entry.format(p)))
            }
            Self::TSTypeLiteral(literal) => {
                Box::new(literal.members.iter().map(|member| member.format(p)))
            }
        }
    }

    fn member_separator(self, p: &'b Prettier<'a>) -> &'static str {
        match self {
            Self::TSTypeLiteral(_) => {
                if p.semi().is_some() {
                    ";"
                } else {
                    ""
                }
            }
            _ => ",",
        }
    }
}

pub(super) fn print_object_properties<'a>(
    p: &mut Prettier<'a>,
    object: ObjectLike<'a, '_>,
) -> Doc<'a> {
    let left_brace = p.text("{");
    let right_brace = p.text("}");

    let should_break = false;
    let member_separator = object.member_separator(p);

    let content = if object.is_empty() {
        p.group(p.array(p_vec!(p, left_brace, p.softline(), right_brace)))
    } else {
        let mut parts = p.vec();
        parts.push(p.text("{"));

        let indent_parts = {
            let len = object.len();
            let has_rest = object.has_rest();
            let mut indent_parts = p.vec();

            indent_parts.push(if p.options.bracket_spacing { p.line() } else { p.softline() });

            let object_docs = object.iter(p).collect::<Vec<_>>();
            for (i, doc) in object_docs.into_iter().enumerate() {
                indent_parts.push(doc);
                if i == len - 1 && !has_rest {
                    break;
                }

                indent_parts.push(p.text(member_separator));
                indent_parts.push(p.line());
            }

            match object {
                ObjectLike::Expression(_)
                | ObjectLike::WithClause(_)
                | ObjectLike::TSTypeLiteral(_) => {}
                ObjectLike::AssignmentTarget(target) => {
                    if let Some(rest) = &target.rest {
                        indent_parts.push(rest.format(p));
                    }
                }
                ObjectLike::Pattern(object) => {
                    if let Some(rest) = &object.rest {
                        indent_parts.push(rest.format(p));
                    }
                }
            }
            indent_parts
        };
        parts.push(p.indent(indent_parts));
        if p.should_print_es5_comma()
            && match object {
                ObjectLike::Pattern(pattern) => pattern.rest.is_none(),
                _ => true,
            }
        {
            parts.push(p.if_break(p.text(member_separator), p.text(""), None));
        }
        parts.push(if p.options.bracket_spacing { p.line() } else { p.softline() });
        parts.push(p.text("}"));

        if matches!(p.current_kind(), AstKind::Program(_)) {
            let should_break =
                misc::has_new_line_in_range(p.source_text, object.span().start, object.span().end);
            return p.group_with_opts(p.array(parts), should_break, None);
        }

        let parent_kind = p.parent_kind();
        if (object.is_object_pattern() && should_hug_the_only_parameter(p, parent_kind))
            || (!should_break
                && object.is_object_pattern()
                && matches!(
                    parent_kind,
                    AstKind::AssignmentExpression(_) | AstKind::VariableDeclarator(_)
                ))
        {
            p.array(parts)
        } else {
            let should_break =
                misc::has_new_line_in_range(p.source_text, object.span().start, object.span().end);
            p.group_with_opts(p.array(parts), should_break, None)
        }
    };

    content
}

fn should_hug_the_only_parameter(p: &mut Prettier<'_>, kind: AstKind<'_>) -> bool {
    match kind {
        AstKind::FormalParameters(params) => {
            function_parameters::should_hug_the_only_function_parameter(p, params)
        }
        _ => false,
    }
}
