//! Root command: typed model, lexopt parser, spec, execution.
//!
//! All option spellings come from [`crate::cli::command`] specs via
//! [`find_by_long`] / [`find_by_short`]; no repeated string literals.

use std::ffi::OsString;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use normpath::PathExt;

use crate::cli::command::{
    CommandSpec, FD_EXAMPLES, FD_OPTIONS, FD_POSITIONALS, OPT_ABSOLUTE_PATH, OPT_AND,
    OPT_BASE_DIRECTORY, OPT_BATCH_SIZE, OPT_CASE_SENSITIVE, OPT_CHANGED_BEFORE, OPT_CHANGED_WITHIN,
    OPT_COLOR, OPT_EXACT, OPT_EXCLUDE, OPT_EXEC, OPT_EXEC_BATCH, OPT_EXTENSION, OPT_FIXED_STRINGS,
    OPT_FOLLOW, OPT_FORMAT, OPT_FULL_PATH, OPT_GLOB, OPT_HELP, OPT_HIDDEN, OPT_HYPERLINK, OPT_IGNORE,
    OPT_IGNORE_CASE, OPT_IGNORE_CONTAIN, OPT_IGNORE_FILE, OPT_IGNORE_PARENT, OPT_IGNORE_VCS,
    OPT_LIST_DETAILS, OPT_MAX_BUFFER_TIME, OPT_MAX_DEPTH, OPT_MAX_ONE_RESULT, OPT_MAX_RESULTS,
    OPT_MIN_DEPTH, OPT_EXACT_DEPTH, OPT_NO_FOLLOW, OPT_NO_GLOBAL_IGNORE_FILE, OPT_NO_HIDDEN,
    OPT_NO_IGNORE, OPT_NO_IGNORE_PARENT, OPT_NO_IGNORE_VCS, OPT_NO_REQUIRE_GIT, OPT_ONE_FILE_SYSTEM,
    OPT_OWNER, OPT_PATH_SEPARATOR, OPT_PRINT0, OPT_PRUNE, OPT_QUIET, OPT_REGEX, OPT_RELATIVE_PATH,
    OPT_REQUIRE_GIT, OPT_SEARCH_PATH, OPT_SHOW_ERRORS, OPT_SIZE, OPT_STRIP_CWD_PREFIX, OPT_THREADS,
    OPT_TYPE, OPT_UNRESTRICTED, OPT_VERSION, POS_PATTERN, ValueMode, find_by_long, find_by_short,
};
use crate::cli::error::CliError;
use crate::config::Config;
use crate::exec::CommandSet;
use crate::exit_codes::ExitCode;
use crate::filesystem;
use crate::filetypes::FileTypes;
#[cfg(unix)]
use crate::filter::OwnerFilter;
use crate::filter::{SizeFilter, TimeFilter};
use crate::regex_helper::{pattern_has_uppercase_char, pattern_matches_strings_with_leading_dot};

pub const SPEC: CommandSpec = CommandSpec {
    name: "fd",
    description: "A program to find entries in your filesystem with regex and glob based matching. By default, fd respects gitignore rules, ignores hidden directories, and is case insensitive.",
    options: FD_OPTIONS,
    positionals: FD_POSITIONALS,
    examples: FD_EXAMPLES,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Executable,
    Empty,
    Socket,
    Pipe,
}

impl FileType {
    fn parse(s: &str) -> Option<Self> {
        use FileType::*;
        Some(match s {
            "file" | "f" => File,
            "directory" | "d" | "dir" => Directory,
            "symlink" | "l" => Symlink,
            "block-device" | "b" => BlockDevice,
            "char-device" | "c" => CharDevice,
            "executable" | "x" => Executable,
            "empty" | "e" => Empty,
            "socket" | "s" => Socket,
            "pipe" | "p" => Pipe,
            _ => return None,
        })
    }

    fn possible() -> &'static str {
        "file, directory, symlink, block-device, char-device, executable, empty, socket, pipe"
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

impl ColorWhen {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "auto" => Self::Auto,
            "always" => Self::Always,
            "never" => Self::Never,
            _ => return None,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StripCwdWhen {
    Auto,
    Always,
    Never,
}

impl StripCwdWhen {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "auto" => Self::Auto,
            "always" => Self::Always,
            "never" => Self::Never,
            _ => return None,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HyperlinkWhen {
    Auto,
    Always,
    Never,
}

impl HyperlinkWhen {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "auto" => Self::Auto,
            "always" => Self::Always,
            "never" => Self::Never,
            _ => return None,
        })
    }
}

pub struct Opts {
    pub hidden: bool,
    pub no_ignore: bool,
    pub no_ignore_vcs: bool,
    pub no_require_git: bool,
    pub no_ignore_parent: bool,
    pub no_global_ignore_file: bool,
    pub unrestricted_count: u8,
    pub case_sensitive: bool,
    pub ignore_case: bool,
    pub glob: bool,
    pub fixed_strings: bool,
    pub exact: bool,
    pub exprs: Option<Vec<String>>,
    pub absolute_path: bool,
    pub list_details: bool,
    pub follow: bool,
    pub full_path: bool,
    pub null_separator: bool,
    pub max_depth: Option<usize>,
    pub min_depth: Option<usize>,
    pub exact_depth: Option<usize>,
    pub exclude: Vec<String>,
    pub prune: bool,
    pub filetype: Option<Vec<FileType>>,
    pub extensions: Option<Vec<String>>,
    pub size: Vec<SizeFilter>,
    pub changed_within: Option<String>,
    pub changed_before: Option<String>,
    #[cfg(unix)]
    pub owner: Option<OwnerFilter>,
    pub format: Option<String>,
    pub exec: Exec,
    pub batch_size: usize,
    pub batch_size_present: bool,
    pub ignore_file: Vec<PathBuf>,
    pub color: ColorWhen,
    pub hyperlink: HyperlinkWhen,
    pub ignore_contain: Vec<String>,
    pub threads: Option<NonZeroUsize>,
    pub max_buffer_time: Option<Duration>,
    pub max_results: Option<usize>,
    pub max_one_result: bool,
    pub quiet: bool,
    pub show_errors: bool,
    pub base_directory: Option<PathBuf>,
    pub pattern: String,
    pub path_separator: Option<String>,
    pub path: Vec<PathBuf>,
    pub search_path: Vec<PathBuf>,
    pub strip_cwd_prefix: Option<Option<StripCwdWhen>>,
    #[cfg(unix)]
    pub one_file_system: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            hidden: false,
            no_ignore: false,
            no_ignore_vcs: false,
            no_require_git: false,
            no_ignore_parent: false,
            no_global_ignore_file: false,
            unrestricted_count: 0,
            case_sensitive: false,
            ignore_case: false,
            glob: false,
            fixed_strings: false,
            exact: false,
            exprs: None,
            absolute_path: false,
            list_details: false,
            follow: false,
            full_path: false,
            null_separator: false,
            max_depth: None,
            min_depth: None,
            exact_depth: None,
            exclude: Vec::new(),
            prune: false,
            filetype: None,
            extensions: None,
            size: Vec::new(),
            changed_within: None,
            changed_before: None,
            #[cfg(unix)]
            owner: None,
            format: None,
            exec: Exec { command: None },
            batch_size: 0,
            batch_size_present: false,
            ignore_file: Vec::new(),
            color: ColorWhen::Auto,
            hyperlink: HyperlinkWhen::Never,
            ignore_contain: Vec::new(),
            threads: None,
            max_buffer_time: None,
            max_results: None,
            max_one_result: false,
            quiet: false,
            show_errors: false,
            base_directory: None,
            pattern: String::new(),
            path_separator: None,
            path: Vec::new(),
            search_path: Vec::new(),
            strip_cwd_prefix: None,
            #[cfg(unix)]
            one_file_system: false,
        }
    }
}

pub struct Exec {
    pub command: Option<CommandSet>,
}

impl Opts {
    pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        if !self.path.is_empty() {
            Ok(self
                .path
                .iter()
                .filter_map(|path| {
                    if filesystem::is_existing_directory(path) {
                        Some(self.normalize_path(path))
                    } else {
                        print_error!(
                            "Search path '{}' is not a directory.",
                            path.to_string_lossy().as_ref()
                        );
                        None
                    }
                })
                .collect())
        } else if !self.search_path.is_empty() {
            Ok(self
                .search_path
                .iter()
                .filter_map(|path| {
                    if filesystem::is_existing_directory(path) {
                        Some(self.normalize_path(path))
                    } else {
                        print_error!(
                            "Search path '{}' is not a directory.",
                            path.to_string_lossy().as_ref()
                        );
                        None
                    }
                })
                .collect())
        } else {
            let current_directory = Path::new("./");
            ensure_current_directory_exists(current_directory)?;
            Ok(vec![self.normalize_path(current_directory)])
        }
    }

    fn normalize_path(&self, path: &Path) -> PathBuf {
        if self.absolute_path {
            filesystem::absolute_path(path.normalize().unwrap().as_path()).unwrap()
        } else if path == Path::new(".") {
            PathBuf::from("./")
        } else if path == Path::new("-") {
            Path::new(".").join(path)
        } else {
            path.to_path_buf()
        }
    }

    pub fn no_search_paths(&self) -> bool {
        self.path.is_empty() && self.search_path.is_empty()
    }

    #[inline]
    pub fn rg_alias_ignore(&self) -> bool {
        self.unrestricted_count > 0
    }

    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth.or(self.exact_depth)
    }

    pub fn min_depth(&self) -> Option<usize> {
        self.min_depth.or(self.exact_depth)
    }

    pub fn threads(&self) -> NonZeroUsize {
        self.threads.unwrap_or_else(default_num_threads)
    }

    pub fn max_results(&self) -> Option<usize> {
        self.max_results
            .filter(|&m| m > 0)
            .or_else(|| self.max_one_result.then_some(1))
    }

    pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
        use StripCwdWhen::*;
        self.no_search_paths()
            && match self.strip_cwd_prefix.map_or(Auto, |o| o.unwrap_or(Always)) {
                Auto => auto_pred(),
                Always => true,
                Never => false,
            }
    }
}

fn default_num_threads() -> NonZeroUsize {
    let fallback = NonZeroUsize::MIN;
    let limit = NonZeroUsize::new(64).unwrap();
    std::thread::available_parallelism()
        .unwrap_or(fallback)
        .min(limit)
}

fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()> {
    if filesystem::is_existing_directory(current_directory) {
        Ok(())
    } else {
        Err(anyhow!(
            "Could not retrieve current directory (has it been deleted?)."
        ))
    }
}

fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    Ok(Duration::from_millis(arg.parse()?))
}

fn os_to_utf8(os: &OsString, option: &'static str) -> Result<String, CliError> {
    os.clone()
        .into_string()
        .map_err(|_| CliError::NonUtf8 { option })
}

fn looks_like_option(s: &OsString) -> bool {
    let bytes = s.as_encoded_bytes();
    if bytes.len() < 2 {
        return false;
    }
    if bytes[0] != b'-' {
        return false;
    }
    // Single "-" is a value, not an option.
    if bytes == b"-" {
        return false;
    }
    // "--" alone is the separator, handled by lexopt itself.
    if bytes == b"--" {
        return false;
    }
    true
}

pub enum ParseOutcome {
    Help,
    Version,
    Opts(Opts),
}

fn required_value(
    parser: &mut lexopt::Parser,
    spec: &'static crate::cli::command::OptionSpec,
) -> Result<OsString, CliError> {
    // Handle `--opt=value` without consuming a following arg.
    if let Some(v) = parser.optional_value() {
        return Ok(v);
    }
    // For options that do not allow hyphen values, a following option-like
    // token means the value is missing.
    if !spec.allow_hyphen_values
        && let Some(raw) = parser
            .try_raw_args()
            .and_then(|r| r.peek().map(|s| s.to_os_string()))
        && looks_like_option(&raw)
    {
        return Err(CliError::MissingValue { option: spec.long });
    }
    parser.value().map_err(|_| CliError::MissingValue { option: spec.long })
}

fn flag_unexpected_value(parser: &mut lexopt::Parser) -> bool {
    parser.optional_value().is_some()
}

/// Collect one `--exec`/`--exec-batch` invocation: first value plus greedy tail
/// until `;` or end. Hyphen values are included.
fn collect_exec_values(
    parser: &mut lexopt::Parser,
    spec: &'static crate::cli::command::OptionSpec,
) -> Result<Vec<OsString>, CliError> {
    let mut vals: Vec<OsString> = Vec::new();
    let first = required_value(parser, spec)?;
    let terminator = spec.value_terminator.unwrap_or(";");
    if first.as_encoded_bytes() == terminator.as_bytes() {
        return Err(CliError::EmptyExec { option: spec.long });
    }
    vals.push(first);
    let terminator = spec.value_terminator.unwrap_or(";");
    loop {
        let next_opt = parser
            .try_raw_args()
            .and_then(|r| r.peek().map(|s| s.to_os_string()));
        match next_opt {
            None => break,
            Some(raw) => {
                if raw.as_encoded_bytes() == terminator.as_bytes() {
                    // Consume the terminator via raw_args.
                    if let Some(mut raws) = parser.try_raw_args() {
                        let _ = raws.next();
                    } else if let Ok(mut raws) = parser.raw_args() {
                        let _ = raws.next();
                    }
                    break;
                } else {
                    // Consume as exec value even if it looks like an option.
                    if let Some(mut raws) = parser.try_raw_args() {
                        if raws.peek().is_some() {
                            if let Some(v) = raws.next() {
                                vals.push(v);
                                continue;
                            }
                        }
                    }
                    // Fallback: use value() (should not normally hit).
                    match parser.value() {
                        Ok(v) => vals.push(v),
                        Err(_) => break,
                    }
                }
            }
        }
        if parser.try_raw_args().is_none() {
            break;
        }
    }
    Ok(vals)
}

fn parse_usize_value(spec_long: &'static str, s: &str) -> Result<usize, CliError> {
    s.parse::<usize>().map_err(|_| CliError::InvalidValue {
        option: spec_long,
        value: s.to_string(),
        reason: format!("invalid number '{s}'"),
    })
}

pub fn parse_from_args<I, T>(args: I) -> Result<ParseOutcome, CliError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let mut parser = lexopt::Parser::from_iter(args);
    parse_with(&mut parser)
}

fn parse_with(parser: &mut lexopt::Parser) -> Result<ParseOutcome, CliError> {
    use lexopt::Arg::{Long, Short, Value};
    let mut opts = Opts::default();
    let mut positionals: Vec<OsString> = Vec::new();
    let mut exec_cmds: Vec<Vec<String>> = Vec::new();
    let mut exec_batch_cmds: Vec<Vec<String>> = Vec::new();
    let mut help_requested = false;
    let mut version_requested = false;

    loop {
        let arg = parser.next().map_err(|e| CliError::InvalidValue {
            option: SPEC.name,
            value: format!("{e:?}"),
            reason: "argument parsing failed".to_string(),
        })?;
        let Some(arg) = arg else { break };
        match arg {
            Short(c) => {
                let Some(spec) = find_by_short(c) else {
                    return Err(CliError::UnknownShort(c));
                };
                if spec.long == OPT_HELP.long {
                    help_requested = true;
                    continue;
                }
                if spec.long == OPT_VERSION.long {
                    version_requested = true;
                    continue;
                }
                handle_option(parser, spec, false, &mut opts, &mut exec_cmds, &mut exec_batch_cmds)?;
            }
            Long(name) => {
                let owned: String = name.to_string();
                let Some(spec) = find_by_long(&owned) else {
                    return Err(CliError::UnknownOption(owned));
                };
                if spec.long == OPT_HELP.long {
                    if flag_unexpected_value(parser) {
                        return Err(CliError::InvalidValue {
                            option: spec.long,
                            value: "".to_string(),
                            reason: "unexpected value".to_string(),
                        });
                    }
                    help_requested = true;
                    continue;
                }
                if spec.long == OPT_VERSION.long {
                    if flag_unexpected_value(parser) {
                        return Err(CliError::InvalidValue {
                            option: spec.long,
                            value: "".to_string(),
                            reason: "unexpected value".to_string(),
                        });
                    }
                    version_requested = true;
                    continue;
                }
                handle_option(parser, spec, true, &mut opts, &mut exec_cmds, &mut exec_batch_cmds)?;
            }
            Value(v) => {
                positionals.push(v);
            }
        }
    }

    if help_requested {
        return Ok(ParseOutcome::Help);
    }
    if version_requested {
        return Ok(ParseOutcome::Version);
    }

    finalize(opts, positionals, exec_cmds, exec_batch_cmds).map(ParseOutcome::Opts)
}

#[allow(clippy::too_many_lines)]
fn handle_option(
    parser: &mut lexopt::Parser,
    spec: &'static crate::cli::command::OptionSpec,
    is_long: bool,
    opts: &mut Opts,
    exec_cmds: &mut Vec<Vec<String>>,
    exec_batch_cmds: &mut Vec<Vec<String>>,
) -> Result<(), CliError> {
    // Every spelling comes from an OptionSpec; dispatch on spec.long.
    let long = spec.long;
    if long == OPT_HIDDEN.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.hidden = true;
    } else if long == OPT_NO_HIDDEN.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.hidden = false;
        opts.unrestricted_count = 0;
    } else if long == OPT_NO_IGNORE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore = true;
    } else if long == OPT_IGNORE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore = false;
        opts.unrestricted_count = 0;
    } else if long == OPT_NO_IGNORE_VCS.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore_vcs = true;
    } else if long == OPT_IGNORE_VCS.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore_vcs = false;
    } else if long == OPT_NO_REQUIRE_GIT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_require_git = true;
    } else if long == OPT_REQUIRE_GIT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_require_git = false;
    } else if long == OPT_NO_IGNORE_PARENT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore_parent = true;
    } else if long == OPT_IGNORE_PARENT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_ignore_parent = false;
    } else if long == OPT_NO_GLOBAL_IGNORE_FILE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.no_global_ignore_file = true;
    } else if long == OPT_UNRESTRICTED.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.unrestricted_count = opts.unrestricted_count.saturating_add(1);
    } else if long == OPT_CASE_SENSITIVE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.case_sensitive = true;
        opts.ignore_case = false;
    } else if long == OPT_IGNORE_CASE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.ignore_case = true;
        opts.case_sensitive = false;
    } else if long == OPT_GLOB.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.glob = true;
    } else if long == OPT_REGEX.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.glob = false;
    } else if long == OPT_FIXED_STRINGS.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.fixed_strings = true;
    } else if long == OPT_EXACT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.exact = true;
    } else if long == OPT_AND.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.exprs.get_or_insert_with(Vec::new).push(s);
    } else if long == OPT_ABSOLUTE_PATH.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.absolute_path = true;
    } else if long == OPT_RELATIVE_PATH.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.absolute_path = false;
    } else if long == OPT_LIST_DETAILS.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.list_details = true;
    } else if long == OPT_FOLLOW.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.follow = true;
    } else if long == OPT_NO_FOLLOW.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.follow = false;
    } else if long == OPT_FULL_PATH.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.full_path = true;
    } else if long == OPT_PRINT0.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.null_separator = true;
    } else if long == OPT_MAX_DEPTH.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.max_depth = Some(parse_usize_value(spec.long, &s)?);
    } else if long == OPT_MIN_DEPTH.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.min_depth = Some(parse_usize_value(spec.long, &s)?);
    } else if long == OPT_EXACT_DEPTH.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.exact_depth = Some(parse_usize_value(spec.long, &s)?);
    } else if long == OPT_EXCLUDE.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.exclude.push(s);
    } else if long == OPT_PRUNE.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.prune = true;
    } else if long == OPT_TYPE.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        let ft = FileType::parse(&s).ok_or_else(|| CliError::InvalidValue {
            option: spec.long,
            value: s.clone(),
            reason: format!("possible values: {}", FileType::possible()),
        })?;
        opts.filetype.get_or_insert_with(Vec::new).push(ft);
    } else if long == OPT_EXTENSION.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.extensions.get_or_insert_with(Vec::new).push(s);
    } else if long == OPT_SIZE.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        let f = SizeFilter::from_string(&s).map_err(|_| CliError::InvalidValue {
            option: spec.long,
            value: s.clone(),
            reason: "not a valid size constraint. See 'fd --help'".to_string(),
        })?;
        opts.size.push(f);
    } else if long == OPT_CHANGED_WITHIN.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.changed_within = Some(s);
    } else if long == OPT_CHANGED_BEFORE.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.changed_before = Some(s);
    } else if long == OPT_OWNER.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        #[cfg(unix)]
        {
            let f = OwnerFilter::from_string(&s).map_err(|e| CliError::InvalidValue {
                option: spec.long,
                value: s.clone(),
                reason: e.to_string(),
            })?;
            // Preserve "ignore if noop" at execution time; store raw and filter later.
            // To keep behavior (empty owner is noop), store Some and let construct filter.
            opts.owner = Some(f);
        }
        #[cfg(not(unix))]
        {
            let _ = s;
        }
    } else if long == OPT_FORMAT.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.format = Some(s);
    } else if long == OPT_EXEC.long {
        let vals = collect_exec_values(parser, spec)?;
        let mut strs = Vec::with_capacity(vals.len());
        for v in vals {
            strs.push(os_to_utf8(&v, spec.long)?);
        }
        exec_cmds.push(strs);
    } else if long == OPT_EXEC_BATCH.long {
        let vals = collect_exec_values(parser, spec)?;
        let mut strs = Vec::with_capacity(vals.len());
        for v in vals {
            strs.push(os_to_utf8(&v, spec.long)?);
        }
        exec_batch_cmds.push(strs);
    } else if long == OPT_BATCH_SIZE.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.batch_size = parse_usize_value(spec.long, &s)?;
        opts.batch_size_present = true;
    } else if long == OPT_IGNORE_FILE.long {
        let v = required_value(parser, spec)?;
        opts.ignore_file.push(PathBuf::from(v));
    } else if long == OPT_COLOR.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.color = ColorWhen::parse(&s).ok_or_else(|| CliError::InvalidValue {
            option: spec.long,
            value: s.clone(),
            reason: "possible values: auto, always, never".to_string(),
        })?;
    } else if long == OPT_HYPERLINK.long {
        if let ValueMode::Optional { default_missing, .. } = spec.takes_value {
            if let Some(v) = parser.optional_value() {
                let s = os_to_utf8(&v, spec.long)?;
                opts.hyperlink = HyperlinkWhen::parse(&s).ok_or_else(|| CliError::InvalidValue {
                    option: spec.long,
                    value: s.clone(),
                    reason: "possible values: auto, always, never".to_string(),
                })?;
            } else {
                let dm = default_missing.unwrap_or("auto");
                opts.hyperlink = HyperlinkWhen::parse(dm).unwrap();
            }
        }
    } else if long == OPT_IGNORE_CONTAIN.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.ignore_contain.push(s);
    } else if long == OPT_THREADS.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        let n: usize = parse_usize_value(spec.long, &s)?;
        let nz = NonZeroUsize::new(n).ok_or_else(|| CliError::InvalidValue {
            option: spec.long,
            value: s.clone(),
            reason: "number would be zero for non-zero type".to_string(),
        })?;
        opts.threads = Some(nz);
    } else if long == OPT_MAX_BUFFER_TIME.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        let d = parse_millis(&s).map_err(|_| CliError::InvalidValue {
            option: spec.long,
            value: s.clone(),
            reason: "invalid number".to_string(),
        })?;
        opts.max_buffer_time = Some(d);
    } else if long == OPT_MAX_RESULTS.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        let n = parse_usize_value(spec.long, &s)?;
        opts.max_results = Some(n);
        opts.max_one_result = false;
    } else if long == OPT_MAX_ONE_RESULT.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.max_one_result = true;
        opts.max_results = None;
    } else if long == OPT_QUIET.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.quiet = true;
    } else if long == OPT_SHOW_ERRORS.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        opts.show_errors = true;
    } else if long == OPT_BASE_DIRECTORY.long {
        let v = required_value(parser, spec)?;
        opts.base_directory = Some(PathBuf::from(v));
    } else if long == OPT_PATH_SEPARATOR.long {
        let v = required_value(parser, spec)?;
        let s = os_to_utf8(&v, spec.long)?;
        opts.path_separator = Some(s);
    } else if long == OPT_SEARCH_PATH.long {
        let v = required_value(parser, spec)?;
        opts.search_path.push(PathBuf::from(v));
    } else if long == OPT_STRIP_CWD_PREFIX.long {
        if let ValueMode::Optional { default_missing, .. } = spec.takes_value {
            if let Some(v) = parser.optional_value() {
                let s = os_to_utf8(&v, spec.long)?;
                let w = StripCwdWhen::parse(&s).ok_or_else(|| CliError::InvalidValue {
                    option: spec.long,
                    value: s.clone(),
                    reason: "possible values: auto, always, never".to_string(),
                })?;
                opts.strip_cwd_prefix = Some(Some(w));
            } else {
                let dm = default_missing.unwrap_or("always");
                let w = StripCwdWhen::parse(dm).unwrap();
                opts.strip_cwd_prefix = Some(Some(w));
            }
        }
    } else if long == OPT_ONE_FILE_SYSTEM.long {
        if is_long && flag_unexpected_value(parser) {
            return Err(unexpected_value(spec));
        }
        #[cfg(unix)]
        {
            opts.one_file_system = true;
        }
    } else {
        return Err(CliError::UnknownOption(long.to_string()));
    }
    Ok(())
}

fn unexpected_value(spec: &'static crate::cli::command::OptionSpec) -> CliError {
    CliError::InvalidValue {
        option: spec.long,
        value: "".to_string(),
        reason: "unexpected value".to_string(),
    }
}

fn finalize(
    mut opts: Opts,
    positionals: Vec<OsString>,
    exec_cmds: Vec<Vec<String>>,
    exec_batch_cmds: Vec<Vec<String>>,
) -> Result<Opts, CliError> {
    // Positionals: first is pattern, rest are paths.
    if let Some((first, rest)) = positionals.split_first() {
        opts.pattern = first
            .clone()
            .into_string()
            .map_err(|_| CliError::NonUtf8 {
                option: POS_PATTERN.name,
            })?;
        for p in rest {
            opts.path.push(PathBuf::from(p));
        }
    }

    // Build exec CommandSet, preserving validation messages.
    if !exec_cmds.is_empty() && !exec_batch_cmds.is_empty() {
        return Err(conflict_opts(&OPT_EXEC, &OPT_EXEC_BATCH));
    }
    if !exec_cmds.is_empty() {
        let cs = CommandSet::new(exec_cmds).map_err(|e| CliError::InvalidValue {
            option: OPT_EXEC.long,
            value: "".to_string(),
            reason: e.to_string(),
        })?;
        opts.exec.command = Some(cs);
    } else if !exec_batch_cmds.is_empty() {
        let cs = CommandSet::new_batch(exec_batch_cmds).map_err(|e| CliError::InvalidValue {
            option: OPT_EXEC_BATCH.long,
            value: "".to_string(),
            reason: e.to_string(),
        })?;
        opts.exec.command = Some(cs);
    }

    validate(&opts)?;
    Ok(opts)
}

fn opt_name(spec: &crate::cli::command::OptionSpec) -> String {
    format!("--{}", spec.long)
}

fn conflict_opts(
    a: &crate::cli::command::OptionSpec,
    b: &crate::cli::command::OptionSpec,
) -> CliError {
    CliError::Conflict {
        first: opt_name(a),
        second: opt_name(b),
    }
}

fn conflict_opt_path(a: &crate::cli::command::OptionSpec, b: &str) -> CliError {
    CliError::Conflict {
        first: opt_name(a),
        second: b.to_string(),
    }
}

fn validate(opts: &Opts) -> Result<(), CliError> {
    let has_exec = opts.exec.command.as_ref().map(|c| c.in_batch_mode());
    let is_exec_one = has_exec == Some(false);
    let is_exec_batch = has_exec == Some(true);

    // group execs vs max-results/quiet/max-one-result
    let has_exec_family = is_exec_one || is_exec_batch || opts.list_details;
    let has_limit_family =
        opts.max_results.is_some() || opts.max_one_result || opts.quiet;
    if has_exec_family && has_limit_family {
        if opts.list_details {
            if opts.max_results.is_some() {
                return Err(conflict_opts(&OPT_LIST_DETAILS, &OPT_MAX_RESULTS));
            }
            if opts.max_one_result {
                return Err(conflict_opts(&OPT_LIST_DETAILS, &OPT_MAX_ONE_RESULT));
            }
            if opts.quiet {
                return Err(conflict_opts(&OPT_LIST_DETAILS, &OPT_QUIET));
            }
        }
        if is_exec_one {
            if opts.max_results.is_some() {
                return Err(conflict_opts(&OPT_EXEC, &OPT_MAX_RESULTS));
            }
            if opts.max_one_result {
                return Err(conflict_opts(&OPT_EXEC, &OPT_MAX_ONE_RESULT));
            }
            if opts.quiet {
                return Err(conflict_opts(&OPT_EXEC, &OPT_QUIET));
            }
        }
        if is_exec_batch {
            if opts.max_results.is_some() {
                return Err(conflict_opts(&OPT_EXEC_BATCH, &OPT_MAX_RESULTS));
            }
            if opts.max_one_result {
                return Err(conflict_opts(&OPT_EXEC_BATCH, &OPT_MAX_ONE_RESULT));
            }
            if opts.quiet {
                return Err(conflict_opts(&OPT_EXEC_BATCH, &OPT_QUIET));
            }
        }
    }
    if opts.quiet && opts.max_results.is_some() {
        return Err(conflict_opts(&OPT_QUIET, &OPT_MAX_RESULTS));
    }

    if opts.glob && opts.fixed_strings {
        return Err(conflict_opts(&OPT_GLOB, &OPT_FIXED_STRINGS));
    }
    if opts.exact && opts.glob {
        return Err(conflict_opts(&OPT_EXACT, &OPT_GLOB));
    }
    if opts.exact && opts.fixed_strings {
        return Err(conflict_opts(&OPT_EXACT, &OPT_FIXED_STRINGS));
    }
    if opts.list_details && opts.absolute_path {
        return Err(conflict_opts(&OPT_LIST_DETAILS, &OPT_ABSOLUTE_PATH));
    }
    if opts.null_separator && opts.list_details {
        return Err(conflict_opts(&OPT_PRINT0, &OPT_LIST_DETAILS));
    }
    if opts.format.is_some() && opts.list_details {
        return Err(conflict_opts(&OPT_FORMAT, &OPT_LIST_DETAILS));
    }
    if opts.exact_depth.is_some() {
        if opts.max_depth.is_some() {
            return Err(conflict_opts(&OPT_EXACT_DEPTH, &OPT_MAX_DEPTH));
        }
        if opts.min_depth.is_some() {
            return Err(conflict_opts(&OPT_EXACT_DEPTH, &OPT_MIN_DEPTH));
        }
    }
    if opts.prune {
        if !opts.size.is_empty() {
            return Err(conflict_opts(&OPT_PRUNE, &OPT_SIZE));
        }
        if opts.exact_depth.is_some() {
            return Err(conflict_opts(&OPT_PRUNE, &OPT_EXACT_DEPTH));
        }
    }
    if !opts.path.is_empty() && !opts.search_path.is_empty() {
        return Err(conflict_opt_path(&OPT_SEARCH_PATH, "[path]"));
    }
    if opts.strip_cwd_prefix.is_some() && (!opts.path.is_empty() || !opts.search_path.is_empty()) {
        return Err(conflict_opt_path(&OPT_STRIP_CWD_PREFIX, "[path]"));
    }
    if opts.batch_size_present && !is_exec_batch {
        return Err(CliError::Requires {
            option: OPT_BATCH_SIZE.long,
            requires: OPT_EXEC_BATCH.long,
        });
    }
    Ok(())
}

pub fn execute(opts: Opts) -> Result<ExitCode> {
    set_working_dir(&opts)?;
    let search_paths = opts.search_paths()?;
    if search_paths.is_empty() {
        bail!("No valid search paths given.");
    }

    ensure_search_pattern_is_not_a_path(&opts)?;
    let pattern = &opts.pattern;
    let exprs = &opts.exprs;
    let empty = Vec::new();

    let pattern_regexps = exprs
        .as_ref()
        .unwrap_or(&empty)
        .iter()
        .chain([pattern])
        .map(|pat| build_pattern_regex(pat, &opts))
        .collect::<Result<Vec<String>>>()?;

    let config = construct_config(opts, &pattern_regexps)?;

    ensure_use_hidden_option_for_leading_dot_pattern(&config, &pattern_regexps)?;

    let regexps = pattern_regexps
        .into_iter()
        .map(|pat| build_regex(pat, &config))
        .collect::<Result<Vec<regex::bytes::Regex>>>()?;

    crate::walk::scan(&search_paths, regexps, config)
}

fn set_working_dir(opts: &Opts) -> Result<()> {
    if let Some(ref base_directory) = opts.base_directory {
        if !filesystem::is_existing_directory(base_directory) {
            return Err(anyhow!(
                "The '--base-directory' path '{}' is not a directory.",
                base_directory.to_string_lossy()
            ));
        }
        std::env::set_current_dir(base_directory).with_context(|| {
            format!(
                "Could not set '{}' as the current working directory",
                base_directory.to_string_lossy()
            )
        })?;
    }
    Ok(())
}

fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()> {
    if opts.full_path {
        return Ok(());
    }
    for pattern in std::iter::once(&opts.pattern).chain(opts.exprs.iter().flatten()) {
        ensure_single_search_pattern_is_not_a_path(pattern)?;
    }
    Ok(())
}

fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()> {
    if pattern.contains('/') {
        Err(anyhow!(
            "The search pattern '{pattern}' contains a path-separation character \
             and will not lead to any search results.\n\n\
             If you want to search for all files inside the '{pattern}' directory, use a match-all pattern:\n\n  \
             fd . '{pattern}'\n\n\
             Instead, if you want your pattern to match the full file path, use:\n\n  \
             fd --full-path '{pattern}'",
            pattern = pattern,
        ))
    } else {
        Ok(())
    }
}

fn build_pattern_regex(pattern: &str, opts: &Opts) -> Result<String> {
    Ok(if opts.glob && !pattern.is_empty() {
        let glob = globset::GlobBuilder::new(pattern).literal_separator(true).build()?;
        glob.regex().to_owned()
    } else if opts.exact {
        format!("^{}$", regex::escape(pattern))
    } else if opts.fixed_strings {
        regex::escape(pattern)
    } else {
        String::from(pattern)
    })
}

#[allow(clippy::too_many_lines)]
fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config> {
    use crate::cli::fd::{ColorWhen as CW, HyperlinkWhen as HW};
    let case_sensitive = !opts.ignore_case
        && (opts.case_sensitive
            || pattern_regexps
                .iter()
                .any(|pat| pattern_has_uppercase_char(pat)));

    let path_separator = opts.path_separator.take();
    let actual_path_separator =
        path_separator.clone().unwrap_or_else(|| std::path::MAIN_SEPARATOR.to_string());

    let size_limits = std::mem::take(&mut opts.size);
    let time_constraints = extract_time_constraints(&opts)?;
    #[cfg(unix)]
    let owner_constraint: Option<OwnerFilter> = opts.owner.and_then(OwnerFilter::filter_ignore);

    let interactive_terminal = std::io::IsTerminal::is_terminal(&std::io::stdout());

    let colored_output = match opts.color {
        CW::Always => true,
        CW::Never => false,
        CW::Auto => {
            let no_color = std::env::var_os("NO_COLOR").is_some_and(|x| !x.is_empty());
            !no_color && interactive_terminal
        }
    };

    let ls_colors = if colored_output {
        Some(
            lscolors::LsColors::from_env()
                .unwrap_or_else(|| lscolors::LsColors::from_string(crate::cli::DEFAULT_LS_COLORS)),
        )
    } else {
        None
    };
    let hyperlink = match opts.hyperlink {
        HW::Always => true,
        HW::Never => false,
        HW::Auto => colored_output,
    };
    let command = extract_command(&mut opts, colored_output)?;
    let has_command = command.is_some();

    let full_path_base = if opts.full_path {
        Some(std::env::current_dir().context(
            "Could not determine current directory. \
             This is required for --full-path.",
        )?)
    } else {
        None
    };

    Ok(Config {
        case_sensitive,
        full_path_base,
        ignore_hidden: !(opts.hidden || opts.rg_alias_ignore()),
        read_fdignore: !(opts.no_ignore || opts.rg_alias_ignore()),
        read_vcsignore: !(opts.no_ignore || opts.rg_alias_ignore() || opts.no_ignore_vcs),
        require_git_to_read_vcsignore: !opts.no_require_git,
        read_parent_ignore: !opts.no_ignore_parent,
        read_global_ignore: !(opts.no_ignore
            || opts.rg_alias_ignore()
            || opts.no_global_ignore_file),
        follow_links: opts.follow,
        one_file_system: {
            #[cfg(unix)]
            {
                opts.one_file_system
            }
            #[cfg(not(unix))]
            {
                false
            }
        },
        null_separator: opts.null_separator,
        quiet: opts.quiet,
        max_depth: opts.max_depth(),
        min_depth: opts.min_depth(),
        prune: opts.prune,
        threads: opts.threads().get(),
        max_buffer_time: opts.max_buffer_time,
        ls_colors,
        hyperlink,
        interactive_terminal,
        file_types: opts.filetype.as_ref().map(|values| FileTypes::from_types(values)),
        extensions: opts
            .extensions
            .as_ref()
            .map(|exts| {
                let patterns = exts
                    .iter()
                    .map(|e| e.trim_start_matches('.'))
                    .map(|e| format!(r".\.{}$", regex::escape(e)));
                regex::bytes::RegexSetBuilder::new(patterns)
                    .case_insensitive(true)
                    .build()
            })
            .transpose()?,
        format: opts.format.as_deref().map(crate::fmt::FormatTemplate::parse),
        command: command.map(std::sync::Arc::new),
        batch_size: opts.batch_size,
        exclude_patterns: opts.exclude.iter().map(|p| String::from("!") + p).collect(),
        ignore_files: std::mem::take(&mut opts.ignore_file),
        size_constraints: size_limits,
        time_constraints,
        #[cfg(unix)]
        owner_constraint,
        show_filesystem_errors: opts.show_errors,
        path_separator,
        actual_path_separator,
        max_results: opts.max_results(),
        strip_cwd_prefix: opts.strip_cwd_prefix(|| !(opts.null_separator || has_command)),
        ignore_contain: opts.ignore_contain,
    })
}

fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>> {
    opts.exec
        .command
        .take()
        .map(Ok)
        .or_else(|| {
            if !opts.list_details {
                return None;
            }
            let res = determine_ls_command(colored_output)
                .map(|cmd| CommandSet::new_batch([cmd]).unwrap());
            Some(res)
        })
        .transpose()
}

fn determine_ls_command(colored_output: bool) -> Result<Vec<&'static str>> {
    #[allow(unused)]
    let gnu_ls = |command_name| {
        let color_arg = if colored_output {
            "--color=always"
        } else {
            "--color=never"
        };
        vec![
            command_name,
            "-l",
            "-h",
            "-d",
            color_arg,
        ]
    };
    let cmd: Vec<&str> = if !cfg!(any(
        target_os = "macos",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )) {
        gnu_ls("ls")
    } else {
        use std::process::{Command, Stdio};
        let gnu_ls_exists = Command::new("gls")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();
        if gnu_ls_exists {
            gnu_ls("gls")
        } else {
            let mut cmd = vec!["ls", "-l", "-h", "-d"];
            if !cfg!(any(target_os = "netbsd", target_os = "openbsd")) && colored_output {
                cmd.push("-G");
            }
            cmd
        }
    };
    Ok(cmd)
}

fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>> {
    let mut time_constraints: Vec<TimeFilter> = Vec::new();
    if let Some(ref t) = opts.changed_within {
        if let Some(f) = TimeFilter::after(t) {
            time_constraints.push(f);
        } else {
            return Err(anyhow!(
                "'{}' is not a valid date or duration. See 'fd --help'.",
                t
            ));
        }
    }
    if let Some(ref t) = opts.changed_before {
        if let Some(f) = TimeFilter::before(t) {
            time_constraints.push(f);
        } else {
            return Err(anyhow!(
                "'{}' is not a valid date or duration. See 'fd --help'.",
                t
            ));
        }
    }
    Ok(time_constraints)
}

fn ensure_use_hidden_option_for_leading_dot_pattern(
    config: &Config,
    pattern_regexps: &[String],
) -> Result<()> {
    if config.ignore_hidden
        && pattern_regexps
            .iter()
            .any(|pat| pattern_matches_strings_with_leading_dot(pat))
    {
        Err(anyhow!(
            "The pattern(s) seems to only match files with a leading dot, but hidden files are \
            filtered by default. Consider adding -H/--hidden to search hidden files as well \
            or adjust your search pattern(s)."
        ))
    } else {
        Ok(())
    }
}

fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex> {
    regex::bytes::RegexBuilder::new(&pattern_regex)
        .case_insensitive(!config.case_sensitive)
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| {
            anyhow!(
                "{}\n\nNote: You can search for literal substrings with '--fixed-strings' \
                 or literal strings with '--exact' options (instead of a regular expression). \
                 Alternatively, you can \
                 also use the '--glob' option to match on a glob pattern.",
                e
            )
        })
}

#[cfg(test)]
mod tests {
    use super::{CliError, ColorWhen, FileType, HyperlinkWhen, Opts, ParseOutcome, SPEC, parse_from_args};
    use crate::cli::command::{FD_OPTIONS, find_by_long};
    use std::ffi::OsString;
    use std::path::PathBuf;

    fn parse(args: &[&str]) -> Result<ParseOutcome, CliError> {
        let owned: Vec<OsString> = std::iter::once(OsString::from("fd"))
            .chain(args.iter().map(|s| OsString::from(*s)))
            .collect();
        parse_from_args(owned)
    }

    fn opts(args: &[&str]) -> Opts {
        match parse(args).expect("parse ok") {
            ParseOutcome::Opts(o) => o,
            ParseOutcome::Help => panic!("unexpected help"),
            ParseOutcome::Version => panic!("unexpected version"),
        }
    }

    #[test]
    fn help_flags() {
        assert!(matches!(parse(&["-h"]), Ok(ParseOutcome::Help)));
        assert!(matches!(parse(&["--help"]), Ok(ParseOutcome::Help)));
    }

    #[test]
    fn version_flags() {
        assert!(matches!(parse(&["-V"]), Ok(ParseOutcome::Version)));
        assert!(matches!(parse(&["--version"]), Ok(ParseOutcome::Version)));
    }

    #[test]
    fn unknown_option() {
        assert!(matches!(
            parse(&["--unknown-opt"]),
            Err(CliError::UnknownOption(_))
        ));
        assert!(matches!(parse(&["-Z"]), Err(CliError::UnknownShort(_))));
    }

    #[test]
    fn missing_and_invalid_values() {
        assert!(matches!(
            parse(&["--max-depth"]),
            Err(CliError::MissingValue { .. })
        ));
        assert!(matches!(
            parse(&["--type", "bogus"]),
            Err(CliError::InvalidValue { .. })
        ));
        assert!(matches!(
            parse(&["--threads", "0"]),
            Err(CliError::InvalidValue { .. })
        ));
        assert!(matches!(
            parse(&["--color", "ALWAYS"]),
            Err(CliError::InvalidValue { .. })
        ));
    }

    #[test]
    fn required_optional_repeated_values() {
        // Required single: --max-depth
        assert_eq!(opts(&["--max-depth", "3"]).max_depth, Some(3));
        // Optional with require_equals: bare --hyperlink means auto
        assert_eq!(
            opts(&["--hyperlink"]).hyperlink,
            HyperlinkWhen::Auto
        );
        assert_eq!(
            opts(&["--hyperlink=never"]).hyperlink,
            HyperlinkWhen::Never
        );
        // Repeatable: --and, --exclude, --type
        let o = opts(&["--and", "a", "--and", "b"]);
        assert_eq!(
            o.exprs,
            Some(vec!["a".to_string(), "b".to_string()])
        );
        let o = opts(&["-E", "a", "--exclude", "b"]);
        assert_eq!(o.exclude, vec!["a".to_string(), "b".to_string()]);
        let o = opts(&["-t", "f", "--type", "d"]);
        assert_eq!(
            o.filetype,
            Some(vec![FileType::File, FileType::Directory])
        );
    }

    #[test]
    fn dash_dash_and_positionals() {
        let o = opts(&["--", "-foo"]);
        assert_eq!(o.pattern, "-foo");
        let o = opts(&["foo", "bar", "baz"]);
        assert_eq!(o.pattern, "foo");
        assert_eq!(o.path, vec![PathBuf::from("bar"), PathBuf::from("baz")]);
    }

    #[test]
    fn clusters_and_equals() {
        let o = opts(&["-HI"]);
        assert!(o.hidden);
        assert!(o.no_ignore);
        let o = opts(&["--max-depth=3"]);
        assert_eq!(o.max_depth, Some(3));
        let o = opts(&["-d3"]);
        assert_eq!(o.max_depth, Some(3));
        let o = opts(&["--color=always"]);
        assert_eq!(o.color, ColorWhen::Always);
    }

    #[test]
    fn three_arg_paths() {
        // Short, Long, Value
        let o = opts(&["-H"]);
        assert!(o.hidden);
        let o = opts(&["--hidden"]);
        assert!(o.hidden);
        let o = opts(&["some-pattern"]);
        assert_eq!(o.pattern, "some-pattern");
    }

    #[test]
    fn opaque_exec_tail_with_terminator() {
        // Greedy exec swallows hyphen values until `;`, then fd options resume.
        let o = opts(&["foo", "-x", "echo", "--max-results", "1", ";", "--max-depth", "2"]);
        assert_eq!(o.max_depth, Some(2));
        assert!(o.exec.command.is_some());
        // Without terminator, trailing flags are part of exec and do not conflict.
        let o = opts(&["--exec", "echo", "foo", "--max-results", "1"]);
        assert!(o.exec.command.is_some());
        assert!(o.max_results.is_none());
    }

    #[test]
    fn conflicts_and_requires() {
        assert!(parse(&["--exec", "echo", ";", "--max-results", "1"]).is_err());
        assert!(parse(&["--batch-size", "2"]).is_err());
        assert!(parse(&["--glob", "--fixed-strings"]).is_err());
    }

    #[test]
    fn non_utf8_rejected() {
        use std::os::unix::ffi::OsStringExt;
        let bad = OsString::from_vec(vec![0xff, 0xfe]);
        let args = vec![OsString::from("fd"), OsString::from("--and"), bad];
        let res = parse_from_args(args);
        assert!(matches!(res, Err(CliError::NonUtf8 { .. })));
    }

    #[test]
    fn metadata_no_hand_duplicated_usage() {
        let src = include_str!("fd.rs");
        let needle = format!("{}: {}", "Usage", "fd");
        assert!(
            !src.contains(&needle),
            "usage must be generated in help.rs, not hand-written in fd.rs"
        );
    }

    #[test]
    fn metadata_parser_uses_specs() {
        // Every registered long must resolve via the shared lookup.
        for opt in FD_OPTIONS {
            assert_eq!(find_by_long(opt.long).unwrap().long, opt.long);
        }
        assert_eq!(SPEC.options.len(), FD_OPTIONS.len());
    }

    #[test]
    fn last_wins_overrides() {
        let o = opts(&["--hidden", "--no-hidden"]);
        assert!(!o.hidden);
        let o = opts(&["--no-hidden", "--hidden"]);
        assert!(o.hidden);
        let o = opts(&["--case-sensitive", "--ignore-case"]);
        assert!(o.ignore_case);
        assert!(!o.case_sensitive);
    }

    #[test]
    fn unrestricted_opposing_clears() {
        let o = opts(&["-u", "--ignore", "--no-hidden"]);
        assert_eq!(o.unrestricted_count, 0);
        assert!(!o.no_ignore);
        assert!(!o.hidden);
    }
}


