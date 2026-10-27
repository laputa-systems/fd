//! Help rendering generated from [`CommandSpec`]. No hand-written usage elsewhere.

use crate::cli::command::{CommandSpec, OptionSpec, ValueMode};

pub(crate) fn format_option_invocation(opt: &OptionSpec) -> String {
    let mut s = String::new();
    if let Some(short) = opt.short {
        s.push('-');
        s.push(short);
        s.push_str(", ");
    }
    s.push_str("--");
    s.push_str(opt.long);
    if let Some(vn) = opt.value_name {
        match opt.takes_value {
            ValueMode::Flag => {}
            ValueMode::Required => {
                s.push_str(&format!(" <{vn}>"));
            }
            ValueMode::Optional { .. } => {
                s.push_str(&format!("[=<{vn}>]"));
            }
        }
    }
    if opt.repeatable {
        // `...` marks repeatable value options.
        if opt.takes_value != ValueMode::Flag {
            s.push_str("...");
        }
    }
    s
}

fn format_option_detail(opt: &OptionSpec) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(vn) = opt.value_name {
        parts.push(format!("value: <{vn}>"));
    }
    if let Some(d) = opt.default {
        parts.push(format!("default: {d}"));
    }
    if let Some(e) = opt.env {
        parts.push(format!("env: {e}"));
    }
    match opt.takes_value {
        ValueMode::Flag => {}
        ValueMode::Required => parts.push("required value".to_string()),
        ValueMode::Optional { require_equals, default_missing } => {
            let mut o = "optional value".to_string();
            if require_equals {
                o.push_str(" (use =)");
            }
            if let Some(dm) = default_missing {
                o.push_str(&format!(" [bare flag: {dm}]"));
            }
            parts.push(o);
        }
    }
    if opt.repeatable {
        parts.push("repeatable".to_string());
    }
    if !opt.aliases.is_empty() {
        parts.push(format!("aliases: {}", opt.aliases.join(", ")));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" [{}]", parts.join(", "))
    }
}

pub fn render_help(spec: &CommandSpec) -> String {
    let mut out = String::new();
    out.push_str(spec.description);
    out.push_str("\n\n");
    // Usage generated from spec, never hand-written elsewhere.
    out.push_str(&format!("Usage: {} [OPTIONS]", spec.name));
    for pos in spec.positionals {
        if pos.required && !pos.repeatable {
            if let Some(vn) = pos.value_name {
                out.push_str(&format!(" <{vn}>"));
            } else {
                out.push_str(&format!(" <{}>", pos.name));
            }
        } else if pos.required && pos.repeatable {
            if let Some(vn) = pos.value_name {
                out.push_str(&format!(" <{vn}>..."));
            } else {
                out.push_str(&format!(" <{}>...", pos.name));
            }
        } else if !pos.required && pos.repeatable {
            if let Some(vn) = pos.value_name {
                out.push_str(&format!(" [{vn}...]"));
            } else {
                out.push_str(&format!(" [{}...]", pos.name));
            }
        } else if let Some(vn) = pos.value_name {
            out.push_str(&format!(" [{vn}]"));
        } else {
            out.push_str(&format!(" [{}]", pos.name));
        }
    }
    out.push_str("\n\nArguments:\n");
    for pos in spec.positionals {
        let label = if pos.repeatable {
            if let Some(vn) = pos.value_name {
                format!("  [{vn}...]")
            } else {
                format!("  [{}...]", pos.name)
            }
        } else if let Some(vn) = pos.value_name {
            format!("  [{vn}]")
        } else {
            format!("  [{}]", pos.name)
        };
        out.push_str(&format!("{label}\n          {}{}\n", pos.help, if pos.required { " (required)" } else { "" }));
    }
    out.push_str("\nOptions:\n");
    for opt in spec.options {
        let inv = format_option_invocation(opt);
        let detail = format_option_detail(opt);
        out.push_str(&format!("  {inv}\n          {}{}\n", opt.help, detail));
    }
    if !spec.examples.is_empty() {
        out.push_str("\nExamples:\n");
        for ex in spec.examples {
            out.push_str(&format!("  {ex}\n"));
        }
    }
    out.push_str("\nBugs can be reported on GitHub: https://github.com/laputa-systems/fd/issues\n");
    out
}

pub fn full_help() -> String {
    render_help(&crate::cli::fd::SPEC)
}

#[cfg(test)]
mod tests {
    use super::{format_option_invocation, full_help, render_help};
    use crate::cli::command::{FD_OPTIONS, FD_POSITIONALS, find_by_long, find_by_short};
    use crate::cli::fd::SPEC;
    use std::collections::HashSet;

    #[test]
    fn top_level_help_shape_and_order() {
        let h = full_help();
        assert!(h.contains(SPEC.description));
        assert!(h.contains("Usage:"));
        assert!(h.contains(SPEC.name));
        // Every option appears with its long spelling.
        for opt in SPEC.options {
            assert!(
                h.contains(&format!("--{}", opt.long)),
                "help missing --{}",
                opt.long
            );
            assert!(!opt.help.is_empty(), "empty help for --{}", opt.long);
        }
        // Every positional appears.
        for pos in SPEC.positionals {
            assert!(h.contains(pos.help), "help missing positional {}", pos.name);
        }
        // Deterministic order: options appear in spec order.
        // Search for the invocation line (two leading spaces) to avoid
        // matching `--opt` mentions inside other options' help text.
        let mut last = 0usize;
        for opt in SPEC.options {
            let inv = format_option_invocation(opt);
            let needle = format!("\n  {inv}");
            let pos = h.find(&needle).unwrap();
            assert!(pos >= last, "help order not deterministic for --{}", opt.long);
            last = pos;
        }
    }

    #[test]
    fn metadata_names_unique() {
        let mut longs = HashSet::new();
        let mut shorts = HashSet::new();
        for opt in FD_OPTIONS {
            assert!(longs.insert(opt.long), "duplicate long --{}", opt.long);
            for a in opt.aliases {
                assert!(longs.insert(*a), "duplicate alias --{a} (from --{})", opt.long);
            }
            if let Some(s) = opt.short {
                assert!(shorts.insert(s), "duplicate short -{s}");
            }
        }
        let mut pos_names = HashSet::new();
        for p in FD_POSITIONALS {
            assert!(pos_names.insert(p.name), "duplicate positional {}", p.name);
        }
    }

    #[test]
    fn every_option_has_description_and_spec_lookup() {
        for opt in FD_OPTIONS {
            assert!(!opt.help.is_empty());
            assert_eq!(find_by_long(opt.long).unwrap().long, opt.long);
            if let Some(s) = opt.short {
                assert_eq!(find_by_short(s).unwrap().long, opt.long);
            }
        }
        assert!(!SPEC.description.is_empty());
    }
}
