use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn always_return_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Should be an imperative statement about what is wrong")
        .with_help("Should be a command-like statement that tells the user how to fix the issue")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct AlwaysReturn;

// See <https://github.com/oxc-project/oxc/issues/6050> for documentation details.
declare_oxc_lint!(
    /// ### What it does
    ///
    /// Briefly describe the rule's purpose.
    ///
    /// ### Why is this bad?
    ///
    /// Explain why violating this rule is problematic.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    AlwaysReturn,
    promise,
    correctness,
);

impl Rule for AlwaysReturn {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("hey.then(x => x)", None),
        ("hey.then(x => ({}))", None),
        ("hey.then(x => { return; })", None),
        ("hey.then(x => { return x ? x.id : null })", None),
        ("hey.then(x => { return x * 10 })", None),
        ("hey.then(x => { process.exit(0); })", None),
        ("hey.then(x => { process.abort(); })", None),
        ("hey.then(function() { return 42; })", None),
        ("hey.then(function() { return new Promise(); })", None),
        (r#"hey.then(function() { return "x"; }).then(doSomethingWicked)"#, None),
        (r#"hey.then(x => x).then(function() { return "3" })"#, None),
        (r#"hey.then(function() { throw new Error("msg"); })"#, None),
        (r#"hey.then(function(x) { if (!x) { throw new Error("no x"); } return x; })"#, None),
        (r#"hey.then(function(x) { if (x) { return x; } throw new Error("no x"); })"#, None),
        (r#"hey.then(function(x) { if (x) { process.exit(0); } throw new Error("no x"); })"#, None),
        (r#"hey.then(function(x) { if (x) { process.abort(); } throw new Error("no x"); })"#, None),
        (r#"hey.then(x => { throw new Error("msg"); })"#, None),
        (r#"hey.then(x => { if (!x) { throw new Error("no x"); } return x; })"#, None),
        (r#"hey.then(x => { if (x) { return x; } throw new Error("no x"); })"#, None),
        ("hey.then(x => { var f = function() { }; return f; })", None),
        ("hey.then(x => { if (x) { return x; } else { return x; } })", None),
        (r#"hey.then(x => { return x; var y = "unreachable"; })"#, None),
        (r#"hey.then(x => { return x; return "unreachable"; })"#, None),
        ("hey.then(x => { return; }, err=>{ log(err); })", None),
        ("hey.then(x => { return x && x(); }, err=>{ log(err); })", None),
        ("hey.then(x => { return x.y || x(); }, err=>{ log(err); })", None),
        (
            r"
            hey.then(x => {
                return anotherFunc({
                    nested: {
                        one: x === 1 ? 1 : 0,
                        two: x === 2 ? 1 : 0
                    }
                })
            })
            ",
            None,
        ),
        (
            r"
            hey.then(({x, y}) => {
                if (y) {
                    throw new Error(x || y)
                }
                return x
            })
            ",
            None,
        ),
        (
            "hey.then(x => { console.log(x) })",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "if(foo) { hey.then(x => { console.log(x) }) }",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "void hey.then(x => { console.log(x) })",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            async function foo() {
              await hey.then(x => { console.log(x) })
            }
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "hey?.then(x => { console.log(x) })",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "foo = (hey.then(x => { console.log(x) }), 42)",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "(42, hey.then(x => { console.log(x) }))",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            hey
                .then(x => { console.log(x) })
                .catch(e => console.error(e))
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            hey
                .then(x => { console.log(x) })
                .catch(e => console.error(e))
                .finally(() => console.error('end'))
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            hey
                .then(x => { console.log(x) })
                .finally(() => console.error('end'))
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        ("hey.then(x => { globalThis = x })", None),
        ("hey.then(x => { globalThis[a] = x })", None),
        ("hey.then(x => { globalThis.a = x })", None),
        ("hey.then(x => { globalThis.a.n = x })", None),
        ("hey.then(x => { globalThis[12] = x })", None),
        (r#"hey.then(x => { globalThis['12']["test"] = x })"#, None),
        (
            r"hey.then(x => { window['x'] = x })",
            Some(serde_json::json!([{ "ignoreAssignmentVariable": ["globalThis", "window"] }])),
        ),
    ];

    let fail = vec![
        ("hey.then(x => {})", None),
        ("hey.then(function() { })", None),
        ("hey.then(function() { }).then(x)", None),
        ("hey.then(function() { }).then(function() { })", None),
        ("hey.then(function() { return; }).then(function() { })", None),
        ("hey.then(function() { doSomethingWicked(); })", None),
        ("hey.then(function() { if (x) { return x; } })", None),
        ("hey.then(function() { if (x) { return x; } else { }})", None),
        ("hey.then(function() { if (x) { } else { return x; }})", None),
        ("hey.then(function() { if (x) { process.chdir(); } else { return x; }})", None),
        ("hey.then(function() { if (x) { return you.then(function() { return x; }); } })", None),
        ("hey.then( x => { x ? x.id : null })", None),
        ("hey.then(function(x) { x ? x.id : null })", None),
        (
            r"
            (function() {
                return hey.then(x => {
                    anotherFunc({
                        nested: {
                            one: x === 1 ? 1 : 0,
                            two: x === 2 ? 1 : 0
                        }
                    })
                })
            })()
            ",
            None,
        ),
        (
            r"
            hey.then(({x, y}) => {
                if (y) {
                    throw new Error(x || y)
                }
            })
            ",
            None,
        ),
        (
            r"
            hey.then(({x, y}) => {
                if (y) {
                    return x
                }
            })
            ",
            None,
        ),
        (
            r"
            hey
                .then(function(x) { console.log(x) /* missing return here */ })
                .then(function(y) { console.log(y) /* no error here */ })
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "const foo = hey.then(function(x) {});",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            function foo() {
                return hey.then(function(x) {});
            }
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            r"
            async function foo() {
                return await hey.then(x => { console.log(x) })
            }
            ",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "const foo = hey?.then(x => { console.log(x) })",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        (
            "const foo = (42, hey.then(x => { console.log(x) }))",
            Some(serde_json::json!([{ "ignoreLastCallback": true }])),
        ),
        ("hey.then(x => { invalid = x })", None),
        (r"hey.then(x => { invalid['x'] = x })", None),
        (
            "hey.then(x => { windo[x] = x })",
            Some(serde_json::json!([{ "ignoreAssignmentVariable": ["window"] }])),
        ),
        (
            r"hey.then(x => { windo['x'] = x })",
            Some(serde_json::json!([{ "ignoreAssignmentVariable": ["window"] }])),
        ),
        (
            r"hey.then(x => { windows['x'] = x })",
            Some(serde_json::json!([{ "ignoreAssignmentVariable": ["window"] }])),
        ),
        (
            r"hey.then(x => { x() })",
            Some(serde_json::json!([{ "ignoreAssignmentVariable": ["window"] }])),
        ),
    ];

    Tester::new(AlwaysReturn::NAME, AlwaysReturn::PLUGIN, pass, fail).test_and_snapshot();
}
