use oxc_ast::{ast::*, AstKind};

use crate::{
    comments::CommentFlags,
    ir::{Doc, DocBuilder},
    p_vec, Format, Prettier,
};

pub(super) fn should_hug_the_only_function_parameter(
    p: &mut Prettier<'_>,
    params: &FormalParameters<'_>,
) -> bool {
    if params.parameters_count() != 1 {
        return false;
    }
    let Some(parameter) = params.items.first() else { return false };

    let all_comment_flags = CommentFlags::Trailing
        | CommentFlags::Leading
        | CommentFlags::Dangling
        | CommentFlags::Block
        | CommentFlags::Line
        | CommentFlags::PrettierIgnore
        | CommentFlags::First
        | CommentFlags::Last;
    if p.has_comment(parameter.span, all_comment_flags) {
        return false;
    }

    match &parameter.pattern.kind {
        BindingPatternKind::ObjectPattern(_) | BindingPatternKind::ArrayPattern(_) => true,
        BindingPatternKind::BindingIdentifier(_) => {
            let Some(ts_type_annotation) = &parameter.pattern.type_annotation else { return false };
            matches!(
                ts_type_annotation.type_annotation,
                TSType::TSTypeLiteral(_) | TSType::TSMappedType(_)
            )
        }
        BindingPatternKind::AssignmentPattern(assignment_pattern) => {
            let left = &assignment_pattern.left.kind;
            if matches!(left, BindingPatternKind::ObjectPattern(_)) {
                return true;
            }

            if !matches!(left, BindingPatternKind::ArrayPattern(_)) {
                return false;
            }

            let right = &assignment_pattern.right;
            match right {
                Expression::Identifier(_) => true,
                Expression::ObjectExpression(obj_expr) => obj_expr.properties.len() == 0,
                Expression::ArrayExpression(arr_expr) => arr_expr.elements.len() == 0,
                _ => false,
            }
        }
    }
}

pub(super) fn print_function_parameters<'a>(
    p: &mut Prettier<'a>,
    params: &FormalParameters<'a>,
) -> Doc<'a> {
    let mut parts = p.vec();
    let is_arrow_function = matches!(p.parent_kind(), AstKind::ArrowFunctionExpression(_));
    let need_parens =
        !is_arrow_function || p.options.arrow_parens.is_always() || params.items.len() != 1;
    if need_parens {
        parts.push(p.text("("));
    }

    let should_hug_the_only_function_parameter = should_hug_the_only_function_parameter(p, params);

    let mut printed = p.vec();
    let len = params.items.len();
    let has_rest = params.rest.is_some();

    if let AstKind::Function(function) = p.parent_kind() {
        if let Some(this_param) = &function.this_param {
            parts.push(this_param.format(p));

            if params.items.len() > 0 {
                printed.push(p.text(","));

                if should_hug_the_only_function_parameter {
                    printed.push(p.space());
                } else if p.is_next_line_empty(this_param.span) {
                    printed.extend(p.hardline());
                    printed.extend(p.hardline());
                } else {
                    printed.push(p.line());
                }
            }
        }
    }

    for (i, param) in params.items.iter().enumerate() {
        if let Some(accessibility) = &param.accessibility {
            printed.push(p.text(accessibility.as_str()));
            printed.push(p.space());
        }

        if param.r#override {
            printed.push(p.text("override "));
        }

        if param.readonly {
            printed.push(p.text("readonly "));
        }

        printed.push(param.format(p));
        if i == len - 1 && !has_rest {
            break;
        }
        printed.push(p.text(","));
        if should_hug_the_only_function_parameter {
            printed.push(p.space());
        } else if p.is_next_line_empty(param.span) {
            printed.extend(p.hardline());
            printed.extend(p.hardline());
        } else {
            printed.push(p.line());
        }
    }
    if let Some(rest) = &params.rest {
        printed.push(rest.format(p));
    }

    if should_hug_the_only_function_parameter {
        let mut parts = p.vec();
        parts.push(p.text("("));
        parts.extend(printed);
        parts.push(p.text(")"));
        return p.array(parts);
    }

    let mut indented = p.vec();
    indented.push(p.softline());
    indented.extend(printed);
    let indented = p.indent(p_vec!(p, p.array(indented)));
    parts.push(indented);
    let skip_dangling_comma = params.rest.is_some()
        || matches!(p.parent_kind(), AstKind::Function(func) if func.this_param.is_some());
    parts.push(p.if_break(p.text(if skip_dangling_comma { "" } else { "," }), p.text(""), None));
    parts.push(p.softline());
    if need_parens {
        parts.push(p.text(")"));
    }

    if p.args.expand_first_arg {
        p.array(parts)
    } else {
        p.group(p.array(parts))
    }
}

pub(super) fn should_group_function_parameters(func: &Function) -> bool {
    let Some(return_type) = &func.return_type else {
        return false;
    };
    let type_parameters = func.type_parameters.as_ref().map(|x| &x.params);

    if let Some(type_parameter) = type_parameters {
        if type_parameter.len() > 1 {
            return false;
        }

        if let Some(type_parameter) = type_parameter.first() {
            if type_parameter.constraint.is_some() || type_parameter.default.is_some() {
                return false;
            }
        }
    }

    // TODO: need union `willBreak`
    func.params.parameters_count() == 1
        && (matches!(
            return_type.type_annotation,
            TSType::TSTypeLiteral(_) | TSType::TSMappedType(_)
        ))
}
