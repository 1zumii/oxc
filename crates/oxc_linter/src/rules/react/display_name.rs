use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use serde_json::Value;

use crate::{
    context::{ContextHost, LintContext},
    rule::Rule,
    AstNode,
};

fn display_name_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Should be an imperative statement about what is wrong")
        .with_help("Should be a command-like statement that tells the user how to fix the issue")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct DisplayName {
    // When true the rule will ignore the name set by the transpiler and require a displayName property in this case.
    ignore_transpiler_name: bool,
    // displayName allows you to name your context object.
    // This name is used in the React dev tools for the context's Provider and Consumer.
    // When true this rule will warn on context objects without a displayName.
    check_context_objects: bool,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow missing displayName in a React component definition
    ///
    /// ### Why is this bad?
    ///
    /// DisplayName allows you to name your component. This name is used by React in debugging messages.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```jsx
    /// var Hello = createReactClass({
    ///   render: function() {
    ///     return <div>Hello {this.props.name}</div>;
    ///   }
    /// });
    ///
    /// const Hello = React.memo(({ a }) => {
    ///   return <>{a}</>
    /// })
    ///
    /// export default ({ a }) => {
    ///   return <>{a}</>
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```jsx
    /// var Hello = createReactClass({
    ///   displayName: 'Hello',
    ///   render: function() {
    ///     return <div>Hello {this.props.name}</div>;
    ///   }
    /// });
    ///
    /// const Hello = React.memo(function Hello({ a }) {
    ///   return <>{a}</>
    /// })
    /// ```
    DisplayName,
    react,
    correctness
);

impl Rule for DisplayName {
    fn should_run(&self, ctx: &ContextHost) -> bool {
        ctx.source_type().is_jsx()
    }

    fn from_configuration(value: serde_json::Value) -> Self {
        let config = value.get(0);
        let ignore_transpiler_name = config
            .and_then(|config| config.get("ignoreTranspilerName"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let check_context_objects = config
            .and_then(|config| config.get("checkContextObjects"))
            .and_then(Value::as_bool)
            .unwrap_or(false);

        dbg!(ignore_transpiler_name, check_context_objects);

        Self { ignore_transpiler_name, check_context_objects }
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        (
            "
            var Hello = createReactClass({
                displayName: 'Hello',
                render: function() {
                    return <div>Hello {this.props.name}</div>;
                }
            });
            ",
            Some(serde_json::json!([{ "ignoreTranspilerName": true }])),
        ),
        (
            "
            class Hello extends React.Component {
                render() {
                    return <div>Hello {this.props.name}</div>;
                }
            }
            Hello.displayName = 'Hello'
            ",
            Some(serde_json::json!([{ "ignoreTranspilerName": true }])),
        ),
    ];

    let fail = vec![];

    // DEBUG:
    // Tester::new(DisplayName::NAME, DisplayName::PLUGIN, pass, fail).test_and_snapshot();
    Tester::new(DisplayName::NAME, DisplayName::PLUGIN, pass, fail).test();
}
