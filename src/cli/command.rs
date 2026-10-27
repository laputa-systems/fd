//! Shared command metadata. Single source of truth for option spellings,
//! help, usage, defaults and examples.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueMode {
    Flag,
    Required,
    Optional {
        require_equals: bool,
        default_missing: Option<&'static str>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct OptionSpec {
    pub short: Option<char>,
    pub long: &'static str,
    pub aliases: &'static [&'static str],
    pub value_name: Option<&'static str>,
    pub takes_value: ValueMode,
    pub repeatable: bool,
    pub allow_hyphen_values: bool,
    pub value_terminator: Option<&'static str>,
    pub default: Option<&'static str>,
    pub env: Option<&'static str>,
    pub help: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct PositionalSpec {
    pub name: &'static str,
    pub value_name: Option<&'static str>,
    pub required: bool,
    pub repeatable: bool,
    pub help: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub options: &'static [OptionSpec],
    pub positionals: &'static [PositionalSpec],
    pub examples: &'static [&'static str],
}

// --- Option specs (one per long flag) ---

pub const OPT_HIDDEN: OptionSpec = OptionSpec {
    short: Some('H'),
    long: "hidden",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Search hidden files and directories. Files and directories are considered to be hidden if their name starts with a `.` sign (dot). Any files or directories that are ignored due to the rules described by --no-ignore are still ignored unless otherwise specified. The flag can be overridden with --no-hidden.",
};

pub const OPT_NO_HIDDEN: OptionSpec = OptionSpec {
    short: None,
    long: "no-hidden",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --hidden.",
};

pub const OPT_NO_IGNORE: OptionSpec = OptionSpec {
    short: Some('I'),
    long: "no-ignore",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not respect .(git|fd)ignore files. The flag can be overridden with --ignore.",
};

pub const OPT_IGNORE: OptionSpec = OptionSpec {
    short: None,
    long: "ignore",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --no-ignore.",
};

pub const OPT_NO_IGNORE_VCS: OptionSpec = OptionSpec {
    short: None,
    long: "no-ignore-vcs",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not respect .gitignore files. The flag can be overridden with --ignore-vcs.",
};

pub const OPT_IGNORE_VCS: OptionSpec = OptionSpec {
    short: None,
    long: "ignore-vcs",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --no-ignore-vcs.",
};

pub const OPT_NO_REQUIRE_GIT: OptionSpec = OptionSpec {
    short: None,
    long: "no-require-git",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not require a git repository to respect gitignores. By default, fd will only respect global gitignore rules, .gitignore rules, and local exclude rules if fd detects that you are searching inside a git repository. This flag allows you to relax this restriction such that fd will respect all git related ignore rules regardless of whether you're searching in a git repository or not. This flag can be disabled with --require-git.",
};

pub const OPT_REQUIRE_GIT: OptionSpec = OptionSpec {
    short: None,
    long: "require-git",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --no-require-git.",
};

pub const OPT_NO_IGNORE_PARENT: OptionSpec = OptionSpec {
    short: None,
    long: "no-ignore-parent",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not respect .(git|fd)ignore files in parent directories. The flag can be overridden with --ignore-parent.",
};

pub const OPT_IGNORE_PARENT: OptionSpec = OptionSpec {
    short: None,
    long: "ignore-parent",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --no-ignore-parent.",
};

pub const OPT_NO_GLOBAL_IGNORE_FILE: OptionSpec = OptionSpec {
    short: None,
    long: "no-global-ignore-file",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not respect the global ignore file.",
};

pub const OPT_UNRESTRICTED: OptionSpec = OptionSpec {
    short: Some('u'),
    long: "unrestricted",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform an unrestricted search, including ignored and hidden files. This is an alias for '--no-ignore --hidden'.",
};

pub const OPT_CASE_SENSITIVE: OptionSpec = OptionSpec {
    short: Some('s'),
    long: "case-sensitive",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform a case-sensitive search. By default, fd uses case-insensitive searches, unless the pattern contains an uppercase character (smart case).",
};

pub const OPT_IGNORE_CASE: OptionSpec = OptionSpec {
    short: Some('i'),
    long: "ignore-case",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform a case-insensitive search. By default, fd uses case-insensitive searches, unless the pattern contains an uppercase character (smart case).",
};

pub const OPT_GLOB: OptionSpec = OptionSpec {
    short: Some('g'),
    long: "glob",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform a glob-based search instead of a regular expression search.",
};

pub const OPT_REGEX: OptionSpec = OptionSpec {
    short: None,
    long: "regex",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform a regular-expression based search (default). This can be used to override --glob.",
};

pub const OPT_FIXED_STRINGS: OptionSpec = OptionSpec {
    short: Some('F'),
    long: "fixed-strings",
    aliases: &["literal"],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Treat the pattern as a literal string instead of a regular expression. Note that this also performs substring comparison. If you want to match on an exact filename, consider using '--glob' or '--exact' instead.",
};

pub const OPT_EXACT: OptionSpec = OptionSpec {
    short: None,
    long: "exact",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Perform an exact match. This is equivalent to '--fixed-strings' but requires the pattern to match the entire filename (or path if '--full-path' is used), rather than a substring. Special regex characters in the pattern are treated as literal characters.",
};

pub const OPT_AND: OptionSpec = OptionSpec {
    short: None,
    long: "and",
    aliases: &[],
    value_name: Some("pattern"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: true,
    value_terminator: None,
    default: None,
    env: None,
    help: "Add additional required search patterns, all of which must be matched. Multiple additional patterns can be specified. The patterns are regular expressions, unless '--glob' or '--fixed-strings' is used.",
};

pub const OPT_ABSOLUTE_PATH: OptionSpec = OptionSpec {
    short: Some('a'),
    long: "absolute-path",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Shows the full path starting from the root as opposed to relative paths. The flag can be overridden with --relative-path.",
};

pub const OPT_RELATIVE_PATH: OptionSpec = OptionSpec {
    short: None,
    long: "relative-path",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --absolute-path.",
};

pub const OPT_LIST_DETAILS: OptionSpec = OptionSpec {
    short: Some('l'),
    long: "list-details",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Use a detailed listing format like 'ls -l'. This is basically an alias for '--exec-batch ls -l' with some additional 'ls' options. This can be used to see more metadata, to show symlink targets and to achieve a deterministic sort order.",
};

pub const OPT_FOLLOW: OptionSpec = OptionSpec {
    short: Some('L'),
    long: "follow",
    aliases: &["dereference"],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "By default, fd does not descend into symlinked directories. Using this flag, symbolic links are also traversed. Flag can be overridden with --no-follow.",
};

pub const OPT_NO_FOLLOW: OptionSpec = OptionSpec {
    short: None,
    long: "no-follow",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Overrides --follow.",
};

pub const OPT_FULL_PATH: OptionSpec = OptionSpec {
    short: Some('p'),
    long: "full-path",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "By default, the search pattern is only matched against the filename (or directory name). Using this flag, the pattern is matched against the full (absolute) path. Example: fd --glob -p '**/.git/config'",
};

pub const OPT_PRINT0: OptionSpec = OptionSpec {
    short: Some('0'),
    long: "print0",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Separate search results by the null character (instead of newlines). Useful for piping results to 'xargs'.",
};

pub const OPT_MAX_DEPTH: OptionSpec = OptionSpec {
    short: Some('d'),
    long: "max-depth",
    aliases: &["maxdepth"],
    value_name: Some("depth"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Limit the directory traversal to a given depth. By default, there is no limit on the search depth.",
};

pub const OPT_MIN_DEPTH: OptionSpec = OptionSpec {
    short: None,
    long: "min-depth",
    aliases: &["mindepth"],
    value_name: Some("depth"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Only show search results starting at the given depth. See also: '--max-depth' and '--exact-depth'",
};

pub const OPT_EXACT_DEPTH: OptionSpec = OptionSpec {
    short: None,
    long: "exact-depth",
    aliases: &[],
    value_name: Some("depth"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Only show search results at the exact given depth. This is an alias for '--min-depth <depth> --max-depth <depth>'.",
};

pub const OPT_EXCLUDE: OptionSpec = OptionSpec {
    short: Some('E'),
    long: "exclude",
    aliases: &[],
    value_name: Some("glob"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Exclude files/directories that match the given glob pattern. This overrides any other ignore logic. Multiple exclude patterns can be specified. Examples: --exclude '*.pyc' --exclude node_modules",
};

pub const OPT_PRUNE: OptionSpec = OptionSpec {
    short: None,
    long: "prune",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Do not traverse into directories that match the search criteria. If you want to exclude specific directories, use the '--exclude=…' option.",
};

pub const OPT_TYPE: OptionSpec = OptionSpec {
    short: Some('t'),
    long: "type",
    aliases: &[],
    value_name: Some("filetype"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Filter by type: file (f), directory (d/dir), symlink (l), executable (x), empty (e), socket (s), pipe (p), char-device (c), block-device (b). This option can be specified more than once to include multiple file types.",
};

pub const OPT_EXTENSION: OptionSpec = OptionSpec {
    short: Some('e'),
    long: "extension",
    aliases: &[],
    value_name: Some("ext"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Filter by extension. By default, this matches all entry types (including directories whose names end with the extension). Use `--type` to restrict the search to specific types. Multiple extensions can be specified. If you want to search for files without extension, you can use the regex '^[^.]+$' as a normal search pattern.",
};

pub const OPT_SIZE: OptionSpec = OptionSpec {
    short: Some('S'),
    long: "size",
    aliases: &[],
    value_name: Some("size"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: true,
    value_terminator: None,
    default: None,
    env: None,
    help: "Limit results based on the size of files using the format <+-><NUM><UNIT>. '+': file size must be greater than or equal to this '-': file size must be less than or equal to this. If neither '+' nor '-' is specified, file size must be exactly equal to this. 'NUM': The numeric size (e.g. 500) 'UNIT': The units for NUM. Allowed unit values: 'b': bytes 'k': kilobytes (base ten, 10^3 = 1000 bytes) 'm': megabytes 'g': gigabytes 't': terabytes 'ki': kibibytes (base two, 2^10 = 1024 bytes) 'mi': mebibytes 'gi': gibibytes 'ti': tebibytes",
};

pub const OPT_CHANGED_WITHIN: OptionSpec = OptionSpec {
    short: None,
    long: "changed-within",
    aliases: &["change-newer-than", "newer", "changed-after"],
    value_name: Some("date|dur"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Filter by file modification time (newer than). The argument can be provided as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min). If the time is not specified, it defaults to 00:00:00. '--change-newer-than', '--newer', or '--changed-after' can be used as aliases. Examples: --changed-within 2weeks --change-newer-than '2018-10-27 10:00:00' --newer 2018-10-27 --changed-after 1day",
};

pub const OPT_CHANGED_BEFORE: OptionSpec = OptionSpec {
    short: None,
    long: "changed-before",
    aliases: &["change-older-than", "older"],
    value_name: Some("date|dur"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Filter by file modification time (older than). The argument can be provided as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min). '--change-older-than' or '--older' can be used as aliases. Examples: --changed-before '2018-10-27 10:00:00' --change-older-than 2weeks --older 2018-10-27",
};

pub const OPT_OWNER: OptionSpec = OptionSpec {
    short: Some('o'),
    long: "owner",
    aliases: &[],
    value_name: Some("user:group"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: true,
    value_terminator: None,
    default: None,
    env: None,
    help: "Filter by owning user and/or group. Format: [(user|uid)][:(group|gid)]. Either side is optional. Precede either side with a '!' to exclude files instead. Examples: --owner john --owner :students --owner '!john:students'",
};

pub const OPT_FORMAT: OptionSpec = OptionSpec {
    short: None,
    long: "format",
    aliases: &[],
    value_name: Some("fmt"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: true,
    value_terminator: None,
    default: None,
    env: None,
    help: "Print results according to template with placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension.",
};

pub const OPT_EXEC: OptionSpec = OptionSpec {
    short: Some('x'),
    long: "exec",
    aliases: &[],
    value_name: Some("cmd"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: true,
    value_terminator: Some(";"),
    default: None,
    env: None,
    help: "Execute a command for each search result in parallel (use --threads=1 for sequential command execution). There is no guarantee of the order commands are executed in. All positional arguments following --exec are considered to be arguments to the command - not to fd. Use ';' to terminate the command template if you need to continue passing fd arguments afterwards. Placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension, '{{': literal '{', '}}': literal '}'. If no placeholder is present, an implicit \"{}\" at the end is assumed. Examples: fd -e zip -x unzip; fd -e h -e cpp -x clang-format -i; fd . src -x echo; fd -e jpg -x convert {} {.}.png",
};

pub const OPT_EXEC_BATCH: OptionSpec = OptionSpec {
    short: Some('X'),
    long: "exec-batch",
    aliases: &[],
    value_name: Some("cmd"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: true,
    value_terminator: Some(";"),
    default: None,
    env: None,
    help: "Execute the given command once, with all search results as arguments. The order of the arguments is non-deterministic. Placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension, '{{': literal '{', '}}': literal '}'. If no placeholder is present, an implicit \"{}\" at the end is assumed. Examples: fd -g 'test_*.py' -X vim; fd -e rs -X wc -l",
};

pub const OPT_BATCH_SIZE: OptionSpec = OptionSpec {
    short: None,
    long: "batch-size",
    aliases: &[],
    value_name: Some("size"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: Some("0"),
    env: None,
    help: "Maximum number of arguments to pass to the command given with -X. If the number of results is greater than the given size, the command given with -X is run again with remaining arguments. A batch size of zero means there is no limit (default), but note that batching might still happen due to OS restrictions on the maximum length of command lines.",
};

pub const OPT_IGNORE_FILE: OptionSpec = OptionSpec {
    short: None,
    long: "ignore-file",
    aliases: &[],
    value_name: Some("path"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Add a custom ignore-file in '.gitignore' format. These files have a low precedence.",
};

pub const OPT_COLOR: OptionSpec = OptionSpec {
    short: Some('c'),
    long: "color",
    aliases: &[],
    value_name: Some("when"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: Some("auto"),
    env: None,
    help: "Declare when to use color for the pattern match output [default: auto] [possible values: auto, always, never]. Environment: NO_COLOR is respected (non-empty disables color in auto mode); LS_COLORS defines file styling.",
};

pub const OPT_HYPERLINK: OptionSpec = OptionSpec {
    short: None,
    long: "hyperlink",
    aliases: &["hyper"],
    value_name: Some("when"),
    takes_value: ValueMode::Optional {
        require_equals: true,
        default_missing: Some("auto"),
    },
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: Some("never"),
    env: None,
    help: "Add a terminal hyperlink to a file:// url for each path in the output. If no argument is given, auto mode is used. This doesn't do anything for --exec and --exec-batch. [default: never] [possible values: auto, always, never]",
};

pub const OPT_IGNORE_CONTAIN: OptionSpec = OptionSpec {
    short: None,
    long: "ignore-contain",
    aliases: &[],
    value_name: Some("name"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Ignore directories containing the named entry.",
};

pub const OPT_THREADS: OptionSpec = OptionSpec {
    short: Some('j'),
    long: "threads",
    aliases: &[],
    value_name: Some("num"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Set number of threads to use for searching and executing. The default is the number of available CPU cores, at most 64.",
};

pub const OPT_MAX_BUFFER_TIME: OptionSpec = OptionSpec {
    short: None,
    long: "max-buffer-time",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Milliseconds to buffer before streaming search results to console.",
};

pub const OPT_MAX_RESULTS: OptionSpec = OptionSpec {
    short: None,
    long: "max-results",
    aliases: &[],
    value_name: Some("count"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Limit the number of search results to 'count' and quit immediately.",
};

pub const OPT_MAX_ONE_RESULT: OptionSpec = OptionSpec {
    short: Some('1'),
    long: "max-one-result",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Limit the search to a single result and quit immediately. This is an alias for '--max-results=1'.",
};

pub const OPT_QUIET: OptionSpec = OptionSpec {
    short: Some('q'),
    long: "quiet",
    aliases: &["has-results"],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "When the flag is present, the program does not print anything and will return with an exit code of 0 if there is at least one match. Otherwise, the exit code will be 1. '--has-results' can be used as an alias.",
};

pub const OPT_SHOW_ERRORS: OptionSpec = OptionSpec {
    short: None,
    long: "show-errors",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Enable the display of filesystem errors for situations such as insufficient permissions or dead symlinks.",
};

pub const OPT_BASE_DIRECTORY: OptionSpec = OptionSpec {
    short: Some('C'),
    long: "base-directory",
    aliases: &[],
    value_name: Some("path"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Change the current working directory of fd to the provided path. This means that search results will be shown with respect to the given base path. Note that relative paths which are passed to fd via the positional <path> argument or the '--search-path' option will also be resolved relative to this directory.",
};

pub const OPT_PATH_SEPARATOR: OptionSpec = OptionSpec {
    short: None,
    long: "path-separator",
    aliases: &[],
    value_name: Some("separator"),
    takes_value: ValueMode::Required,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Set the path separator to use when printing file paths. The default is '/'.",
};

pub const OPT_SEARCH_PATH: OptionSpec = OptionSpec {
    short: None,
    long: "search-path",
    aliases: &[],
    value_name: Some("search-path"),
    takes_value: ValueMode::Required,
    repeatable: true,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Provide paths to search as an alternative to the positional <path> argument. Changes the usage to `fd [OPTIONS] --search-path <path> --search-path <path2> [<pattern>]`",
};

pub const OPT_STRIP_CWD_PREFIX: OptionSpec = OptionSpec {
    short: None,
    long: "strip-cwd-prefix",
    aliases: &[],
    value_name: Some("when"),
    takes_value: ValueMode::Optional {
        require_equals: true,
        default_missing: Some("always"),
    },
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "By default, relative paths are prefixed with './' when -x/--exec, -X/--exec-batch, or -0/--print0 are given, to reduce the risk of a path starting with '-' being treated as a command line option. Use this flag to change this behavior. If this flag is used without a value, it is equivalent to passing \"always\". [possible values: auto, always, never]",
};

pub const OPT_ONE_FILE_SYSTEM: OptionSpec = OptionSpec {
    short: None,
    long: "one-file-system",
    aliases: &["mount", "xdev"],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "By default, fd will traverse the file system tree as far as other options dictate. With this flag, fd ensures that it does not descend into a different file system than the one it started in. Comparable to the -mount or -xdev filters of find(1).",
};

pub const OPT_HELP: OptionSpec = OptionSpec {
    short: Some('h'),
    long: "help",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Print help.",
};

pub const OPT_VERSION: OptionSpec = OptionSpec {
    short: Some('V'),
    long: "version",
    aliases: &[],
    value_name: None,
    takes_value: ValueMode::Flag,
    repeatable: false,
    allow_hyphen_values: false,
    value_terminator: None,
    default: None,
    env: None,
    help: "Print version.",
};

pub const POS_PATTERN: PositionalSpec = PositionalSpec {
    name: "pattern",
    value_name: Some("pattern"),
    required: false,
    repeatable: false,
    help: "the search pattern (a regular expression, unless '--glob' is used; optional). If your pattern starts with a dash (-), make sure to pass '--' first, or it will be considered as a flag (fd -- '-foo').",
};

pub const POS_PATH: PositionalSpec = PositionalSpec {
    name: "path",
    value_name: Some("path"),
    required: false,
    repeatable: true,
    help: "the root directories for the filesystem search (optional). If omitted, search the current working directory.",
};

pub const FD_OPTIONS: &[OptionSpec] = &[
    OPT_HIDDEN,
    OPT_NO_HIDDEN,
    OPT_NO_IGNORE,
    OPT_IGNORE,
    OPT_NO_IGNORE_VCS,
    OPT_IGNORE_VCS,
    OPT_NO_REQUIRE_GIT,
    OPT_REQUIRE_GIT,
    OPT_NO_IGNORE_PARENT,
    OPT_IGNORE_PARENT,
    OPT_NO_GLOBAL_IGNORE_FILE,
    OPT_UNRESTRICTED,
    OPT_CASE_SENSITIVE,
    OPT_IGNORE_CASE,
    OPT_GLOB,
    OPT_REGEX,
    OPT_FIXED_STRINGS,
    OPT_EXACT,
    OPT_AND,
    OPT_ABSOLUTE_PATH,
    OPT_RELATIVE_PATH,
    OPT_LIST_DETAILS,
    OPT_FOLLOW,
    OPT_NO_FOLLOW,
    OPT_FULL_PATH,
    OPT_PRINT0,
    OPT_MAX_DEPTH,
    OPT_MIN_DEPTH,
    OPT_EXACT_DEPTH,
    OPT_EXCLUDE,
    OPT_PRUNE,
    OPT_TYPE,
    OPT_EXTENSION,
    OPT_SIZE,
    OPT_CHANGED_WITHIN,
    OPT_CHANGED_BEFORE,
    OPT_OWNER,
    OPT_FORMAT,
    OPT_EXEC,
    OPT_EXEC_BATCH,
    OPT_BATCH_SIZE,
    OPT_IGNORE_FILE,
    OPT_COLOR,
    OPT_HYPERLINK,
    OPT_IGNORE_CONTAIN,
    OPT_THREADS,
    OPT_MAX_BUFFER_TIME,
    OPT_MAX_RESULTS,
    OPT_MAX_ONE_RESULT,
    OPT_QUIET,
    OPT_SHOW_ERRORS,
    OPT_BASE_DIRECTORY,
    OPT_PATH_SEPARATOR,
    OPT_SEARCH_PATH,
    OPT_STRIP_CWD_PREFIX,
    OPT_ONE_FILE_SYSTEM,
    OPT_HELP,
    OPT_VERSION,
];

pub const FD_POSITIONALS: &[PositionalSpec] = &[POS_PATTERN, POS_PATH];

pub const FD_EXAMPLES: &[&str] = &[
    "fd . src -x echo",
    "fd -e rs -X wc -l",
    "fd --glob '**/.git/config'",
];

/// Find an option by long name or alias. Consumes [`OptionSpec`], never repeats literals.
pub fn find_by_long(name: &str) -> Option<&'static OptionSpec> {
    FD_OPTIONS
        .iter()
        .find(|o| o.long == name || o.aliases.contains(&name))
}

/// Find an option by short flag. Consumes [`OptionSpec`].
pub fn find_by_short(short: char) -> Option<&'static OptionSpec> {
    FD_OPTIONS.iter().find(|o| o.short == Some(short))
}
