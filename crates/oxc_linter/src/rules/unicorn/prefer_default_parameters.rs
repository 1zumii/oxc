use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

fn prefer_default_parameters_diagnostic() -> OxcDiagnostic {}

#[derive(Debug, Default, Clone)]
pub struct PreferDefaultParameters;

declare_oxc_lint!(
    /// ### What it does
    /// Prefer default parameters over reassignment
    ///
    /// ### Why is this bad?
    /// Instead of reassigning a function parameter, default parameters should be used. The `foo = foo || 123` statement evaluates to `123` when `foo` is falsy, possibly leading to confusing behavior, whereas default parameters only apply when passed an `undefined` value. This rule only reports reassignments to literal values.
    ///
    /// ### Example
    /// ```javascript
    /// // Bad
    /// function abc(foo) {
    ///     foo = foo || 'bar';
    /// }
    ///
    /// function abc(foo) {
    ///     const bar = foo || 'bar';
    /// }
    ///
    /// // Good
    /// function abc(foo = 'bar') {}
    ///
    /// function abc(foo) {
    ///     foo = foo || bar();
    /// }
    /// ```
    PreferDefaultParameters,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc-project.github.io/docs/contribute/linter.html#rule-category> for details
);

impl Rule for PreferDefaultParameters {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;
    use std::path::PathBuf;

    let pass = vec![
        "function abc(foo = { bar: 123 }) { }",
        "function abc({ bar } = { bar: 123 }) { }",
        "function abc({ bar = 123 } = { bar }) { }",
        "function abc(foo = fooDefault) { }",
        "function abc(foo = {}) { }",
        "function abc(foo = 'bar') { }",
        "function abc({ bar = 123 } = {}) { }",
        "const abc = (foo = 'bar') => { };",
        "foo = foo || 'bar';",
        "const bar = foo || 'bar';",
        "const abc = function(foo = { bar: 123 }) { }",
        "const abc = function({ bar } = { bar: 123 }) { }",
        "const abc = function({ bar = 123 } = {}) { }",
        "
			function abc(foo) {
				foo = foo || bar();
			}
		",
        "
			function abc(foo) {
				foo = foo || {bar};
			}
		",
        "
			function abc(foo) {
				const {bar} = foo || 123;
			}
		",
        "
			function abc(foo, bar) {
				bar = foo || 'bar';
			}
		",
        "
			function abc(foo, bar) {
				foo = foo || 'bar';
				baz();
			}
		",
        "
			function abc(foo) {
				foo = foo && 'bar';
			}
		",
        "
			function abc(foo) {
				foo = foo || 1 && 2 || 3;
			}
		",
        "
			function abc(foo) {
				foo = !foo || 'bar';
			}
		",
        "
			function abc(foo) {
				foo = (foo && bar) || baz;
			}
		",
        "
			function abc(foo = 123) {
				foo = foo || 'bar';
			}
		",
        "
			function abc() {
				let foo = 123;
				foo = foo || 'bar';
			}
		",
        "
			function abc() {
				let foo = 123;
				const bar = foo || 'bar';
			}
		",
        "
			const abc = (foo, bar) => {
				bar = foo || 'bar';
			};
		",
        "
			const abc = function(foo, bar) {
				bar = foo || 'bar';
			}
		",
        "
			const abc = function(foo) {
				foo = foo || bar();
			}
		",
        "
			function abc(foo) {
				function def(bar) {
					foo = foo || 'bar';
				}
			}
		",
        "
			function abc(foo) {
				const bar = foo = foo || 123;
			}
		",
        "
			function abc(foo) {
				bar(foo = foo || 1);
				baz(foo);
			}
		",
        "
			function abc(foo) {
				console.log(foo);
				foo = foo || 123;
			}
		",
        "
			function abc(foo) {
				console.log(foo);
				foo = foo || 'bar';
			}
		",
        "
			function abc(foo) {
				const bar = foo || 'bar';
				console.log(foo, bar);
			}
		",
        "
			function abc(foo) {
				let bar = 123;
				bar = foo;
				foo = foo || 123;
			}
		",
        "
			function abc(foo) {
				bar();
				foo = foo || 123;
			}
		",
        "
			const abc = (foo) => {
				bar();
				foo = foo || 123;
			};
		",
        "
			const abc = function(foo) {
				bar();
				foo = foo || 123;
			};
		",
        "
			function abc(foo) {
				sideEffects();
				foo = foo || 123;

				function sideEffects() {
					foo = 456;
				}
			}
		",
        "
			function abc(foo) {
				const bar = sideEffects();
				foo = foo || 123;

				function sideEffects() {
					foo = 456;
				}
			}
		",
        "
			function abc(foo) {
				const bar = sideEffects() + 123;
				foo = foo || 123;

				function sideEffects() {
					foo = 456;
				}
			}
		",
        "
			function abc(foo) {
				const bar = !sideEffects();
				foo = foo || 123;

				function sideEffects() {
					foo = 456;
				}
			}
		",
        "
			function abc(foo) {
				const bar = function() {
					foo = 456;
				}
				foo = foo || 123;
			}
		",
        "
			function abc(...foo) {
				foo = foo || 'bar';
			}
		",
        "
			function abc(foo = 'bar') {
				foo = foo || 'baz';
			}
		",
        "
			function abc(foo, bar) {
				const { baz, ...rest } = bar;
				foo = foo || 123;
			}
		",
        "
			function abc(foo, bar) {
				const baz = foo?.bar;
				foo = foo || 123;
			}
		",
        "
			function abc(foo, bar) {
				import('foo');
				foo = foo || 123;
			}
		",
    ];

    let fail = vec![];

    Tester::new(PreferDefaultParameters::NAME, pass, fail).test_and_snapshot();
}
