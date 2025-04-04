//! Create documentation pages for each rule. Pages are printed as Markdown and
//! get added to the website.

use std::{
    fmt::{self, Write},
    path::PathBuf,
};

use oxc_linter::{LintPlugins, table::RuleTableRow};

use super::HtmlWriter;

pub fn render_rule_docs_page(rule: &RuleTableRow, git_ref: &str) -> Result<String, fmt::Error> {
    const APPROX_FIX_CATEGORY_AND_PLUGIN_LEN: usize = 512;
    let RuleTableRow { name, documentation, plugin, turned_on_by_default, autofix, category } =
        rule;

    let mut page = HtmlWriter::with_capacity(
        documentation.map_or(0, str::len) + name.len() + APPROX_FIX_CATEGORY_AND_PLUGIN_LEN,
    );

    writeln!(
        page,
        "<!-- This file is auto-generated by {}. Do not edit it manually. -->\n",
        file!()
    )?;
    writeln!(page, r#"# {plugin}/{name} <Badge type="info" text="{category}" />"#)?;

    // rule metadata
    page.div(r#"class="rule-meta""#, |p| {
        if *turned_on_by_default {
            p.Alert(r#"class="default-on" type="success""#, |p| {
                p.writeln(r#"<span class="emoji">✅</span> This rule is turned on by default."#)
            })?;
        }

        if let Some(emoji) = autofix.emoji() {
            p.Alert(r#"class="fix" type="info""#, |p| {
                p.writeln(format!(
                    r#"<span class="emoji">{}</span> {}"#,
                    emoji,
                    autofix.description()
                ))
            })?;
        }

        Ok(())
    })?;

    // rule documentation
    if let Some(docs) = documentation {
        writeln!(page, "\n{}", *docs)?;
    }

    // how to use
    writeln!(page, "\n## How to use\n{}", how_to_use(rule))?;
    writeln!(page, "\n## References\n- [Rule Source]({})", rule_source(rule, git_ref))?;

    Ok(page.into())
}

fn rule_source(rule: &RuleTableRow, git_ref: &str) -> String {
    use std::sync::OnceLock;

    use project_root::get_project_root;

    const GITHUB_BASE_URL: &str = "https://github.com/oxc-project/oxc/blob/";
    const LINT_RULES_DIR: &str = "crates/oxc_linter/src/rules";
    static ROOT: OnceLock<PathBuf> = OnceLock::new();

    let github_url: String = GITHUB_BASE_URL.to_owned() + git_ref;
    let root = ROOT.get_or_init(|| get_project_root().unwrap());

    // Some rules are folders with a mod.rs file, others are just a rust file
    let mut rule_path =
        root.join(LINT_RULES_DIR).join(&rule.plugin).join(rule.name.replace('-', "_"));
    if rule_path.is_dir() {
        rule_path.push("mod.rs");
    } else {
        rule_path.set_extension("rs");
    }

    assert!(rule_path.exists(), "Rule source not found: {}", rule_path.display());
    assert!(rule_path.is_file(), "Rule source is not a file: {}", rule_path.display());

    rule_path.to_string_lossy().replace(root.to_str().unwrap(), github_url.as_str())
}

/// Returns `true` if the given plugin is a default plugin.
/// - Example: `eslint` => true
/// - Example: `jest` => false
fn is_default_plugin(plugin: &str) -> bool {
    let plugin = LintPlugins::from(plugin);
    LintPlugins::default().contains(plugin)
}

/// Returns the normalized plugin name.
/// - Example: `react_perf` -> `react-perf`
/// - Example: `eslint` -> `eslint`
/// - Example: `jsx_a11y` -> `jsx-a11y`
fn get_normalized_plugin_name(plugin: &str) -> &str {
    LintPlugins::from(plugin).into()
}

fn how_to_use(rule: &RuleTableRow) -> String {
    let plugin = &rule.plugin;
    let normalized_plugin_name = get_normalized_plugin_name(plugin);
    let rule_full_name = if normalized_plugin_name.is_empty() {
        rule.name.to_string()
    } else {
        format!("{}/{}", normalized_plugin_name, rule.name)
    };
    let is_default_plugin = is_default_plugin(plugin);
    let enable_bash_example = if is_default_plugin {
        format!(r"oxlint --deny {rule_full_name}")
    } else {
        format!(r"oxlint --deny {rule_full_name} --{normalized_plugin_name}-plugin")
    };
    let enable_config_example = if is_default_plugin {
        format!(
            r#"{{
    "rules": {{
        "{rule_full_name}": "error"
    }}
}}"#
        )
    } else {
        format!(
            r#"{{
    "plugins": ["{normalized_plugin_name}"],
    "rules": {{
        "{rule_full_name}": "error"
    }}
}}"#
        )
    };
    format!(
        r"
To **enable** this rule in the CLI or using the config file, you can use:

::: code-group

```bash [CLI]
{enable_bash_example}
```

```json [Config (.oxlintrc.json)]
{enable_config_example}
```

:::
"
    )
}
