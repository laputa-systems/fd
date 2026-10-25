# fd

[![CICD](https://github.com/sharkdp/fd/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/fd/actions/workflows/CICD.yml)
[![Version info](https://img.shields.io/crates/v/fd-find.svg)](https://crates.io/crates/fd-find)
[[中文](https://github.com/cha0ran/fd-zh)]
[[한국어](https://github.com/spearkkk/fd-kor)]

`fd` is a program to find entries in your filesystem.
It is a simple, fast and user-friendly alternative to [`find`](https://www.gnu.org/software/findutils/).
While it does not aim to support all of `find`'s powerful functionality, it provides sensible
(opinionated) defaults for a majority of use cases.

[Installation](#installation) • [How to use](#how-to-use) • [Troubleshooting](#troubleshooting)

## Features

* Intuitive syntax: `fd PATTERN` instead of `find -iname '*PATTERN*'`.
* Regular expression (default) and glob-based patterns.
* [Very fast](#benchmark) due to parallelized directory traversal.
* Uses colors to highlight different file types (same as `ls`).
* Supports [parallel command execution](#command-execution)
* Smart case: the search is case-insensitive by default. It switches to
  case-sensitive if the pattern contains an uppercase
  character[\*](http://vimdoc.sourceforge.net/htmldoc/options.html#'smartcase').
* Ignores hidden directories and files, by default.
* Ignores patterns from your `.gitignore`, by default.
* The command name is *50%* shorter[\*](https://github.com/ggreer/the_silver_searcher) than
  `find` :-).

## Demo

![Demo](doc/screencast.svg)

## How to use

First, to get an overview of all available command line options, you can either run
[`fd -h`](#command-line-options) for a concise help message or `fd --help` for a more detailed
version.

### Simple search

*fd* is designed to find entries in your filesystem. The most basic search you can perform is to
run *fd* with a single argument: the search pattern. For example, assume that you want to find an
old script of yours (the name included `netflix`):
``` bash
> fd netfl
Software/python/imdb-ratings/netflix-details.py
```
If called with just a single argument like this, *fd* searches the current directory recursively
for any entries that *contain* the pattern `netfl`.

### Regular expression search

The search pattern is treated as a regular expression. Here, we search for entries that start
with `x` and end with `rc`:
``` bash
> cd /etc
> fd '^x.*rc$'
X11/xinit/xinitrc
X11/xinit/xserverrc
```

The regular expression syntax used by `fd` is [documented here](https://docs.rs/regex/latest/regex/#syntax).

### Specifying the root directory

If we want to search a specific directory, it can be given as a second argument to *fd*:
``` bash
> fd passwd /etc
/etc/default/passwd
/etc/pam.d/passwd
/etc/passwd
```

### List all files, recursively

*fd* can be called with no arguments. This is very useful to get a quick overview of all entries
in the current directory, recursively (similar to `ls -R`):
``` bash
> cd fd/tests
> fd
testenv
testenv/mod.rs
tests.rs
```

If you want to use this functionality to list all files in a given directory, you have to use
a catch-all pattern such as `.` or `^`:
``` bash
> fd . fd/tests/
testenv
testenv/mod.rs
tests.rs
```

### Searching for a particular file extension

Often, we are interested in all files of a particular type. This can be done with the `-e` (or
`--extension`) option. Here, we search for all Markdown files in the fd repository:
``` bash
> cd fd
> fd -e md
CONTRIBUTING.md
README.md
```

The `-e` option can be used in combination with a search pattern:
``` bash
> fd -e rs mod
src/fshelper/mod.rs
src/lscolors/mod.rs
tests/testenv/mod.rs
```

### Searching for a particular file name

 To find files with exactly the provided search pattern, use the `-g` (or `--glob`) option:
``` bash
> fd -g libc.so /usr
/usr/lib32/libc.so
/usr/lib/libc.so
```

### Hidden and ignored files
By default, *fd* does not search hidden directories and does not show hidden files in the
search results. To disable this behavior, we can use the `-H` (or `--hidden`) option:
``` bash
> fd pre-commit
> fd -H pre-commit
.git/hooks/pre-commit.sample
```

If we work in a directory that is a Git repository (or includes Git repositories), *fd* does not
search folders (and does not show files) that match one of the `.gitignore` patterns. To disable
this behavior, we can use the `-I` (or `--no-ignore`) option:
``` bash
> fd num_cpu
> fd -I num_cpu
target/debug/deps/libnum_cpus-f5ce7ef99006aa05.rlib
```

To really search *all* files and directories, simply combine the hidden and ignore features to show
everything (`-HI`) or use `-u`/`--unrestricted`.

### Matching the full path
By default, *fd* only matches the filename of each file. However, using the `--full-path` or `-p` option,
you can match against the full path.

```bash
> fd -u -p -g '**/.git/config'
> fd -p '.*/lesson-\d+/[a-z]+.(jpg|png)'
```

### Command execution

Instead of just showing the search results, you often want to *do something* with them. `fd`
provides two ways to execute external commands for each of your search results:

* The `-x`/`--exec` option runs an external command *for each of the search results* (in parallel).
* The `-X`/`--exec-batch` option launches the external command once, with *all search results as arguments*.

#### Examples

Recursively find all zip archives and unpack them:
``` bash
fd -e zip -x unzip
```
If there are two such files, `file1.zip` and `backup/file2.zip`, this would execute
`unzip file1.zip` and `unzip backup/file2.zip`. The two `unzip` processes run in parallel
(if the files are found fast enough).

Find all `*.h` and `*.cpp` files and auto-format them inplace with `clang-format -i`:
``` bash
fd -e h -e cpp -x clang-format -i
```
Note how the `-i` option to `clang-format` can be passed as a separate argument. This is why
we put the `-x` option last.

Any positional arguments after `-x` belong to the command template, not to `fd` itself. If you
also want to pass a pattern or search path, put `-x` last:
``` bash
fd pattern path -x echo
```

Find all `test_*.py` files and open them in your favorite editor:
``` bash
fd -g 'test_*.py' -X vim
```
Note that we use capital `-X` here to open a single `vim` instance. If there are two such files,
`test_basic.py` and `lib/test_advanced.py`, this will run `vim test_basic.py lib/test_advanced.py`.

To see details like file permissions, owners, file sizes etc., you can tell `fd` to show them
by running `ls` for each result:
``` bash
fd … -X ls -lhd --color=always
```
This pattern is so useful that `fd` provides a shortcut. You can use the `-l`/`--list-details`
option to execute `ls` in this way: `fd … -l`.

The `-X` option is also useful when combining `fd` with [ripgrep](https://github.com/BurntSushi/ripgrep/) (`rg`) in order to search within a certain class of files, like all C++ source files:
```bash
fd -e cpp -e cxx -e h -e hpp -X rg 'std::cout'
```

Convert all `*.jpg` files to `*.png` files:
``` bash
fd -e jpg -x convert {} {.}.png
```
Here, `{}` is a placeholder for the search result. `{.}` is the same, without the file extension.
See below for more details on the placeholder syntax.

The terminal output of commands run from parallel threads using `-x` will not be interlaced or garbled,
so `fd -x` can be used to rudimentarily parallelize a task run over many files.
An example of this is calculating the checksum of each individual file within a directory.
``` bash
fd -tf -x md5sum > file_checksums.txt
```

#### Placeholder syntax

The `-x` and `-X` options take a *command template* as a series of arguments (instead of a single string).
If you want to add additional options to `fd` after the command template, you can terminate it with a `\;`.

For example, `fd -x echo \; pattern path` treats `pattern path` as `fd` arguments instead of
passing them to `echo`. In practice, it is often clearer to write `fd pattern path -x echo`.

The syntax for generating commands is similar to that of [GNU Parallel](https://www.gnu.org/software/parallel/):

- `{}`: A placeholder token that will be replaced with the path of the search result
  (`documents/images/party.jpg`).
- `{.}`: Like `{}`, but without the file extension (`documents/images/party`).
- `{/}`: A placeholder that will be replaced by the basename of the search result (`party.jpg`).
- `{//}`: The parent of the discovered path (`documents/images`).
- `{/.}`: The basename, with the extension removed (`party`).

If you do not include a placeholder, *fd* automatically adds a `{}` at the end.

#### Parallel vs. serial execution

For `-x`/`--exec`, you can control the number of parallel jobs by using the `-j`/`--threads` option.
Use `--threads=1` for serial execution.

### Excluding specific files or directories

Sometimes we want to ignore search results from a specific subdirectory. For example, we might
want to search all hidden files and directories (`-H`) but exclude all matches from `.git`
directories. We can use the `-E` (or `--exclude`) option for this. It takes an arbitrary glob
pattern as an argument:
``` bash
> fd -H -E .git …
```

We can also use this to skip mounted directories:
``` bash
> fd -E /mnt/external-drive …
```

.. or to skip certain file types:
``` bash
> fd -E '*.bak' …
```

To make exclude-patterns like these permanent, you can create a `.fdignore` file. They work like
`.gitignore` files, but are specific to `fd`. For example:
``` bash
> cat ~/.fdignore
/mnt/external-drive
*.bak
```

> [!NOTE]
> `fd` also supports `.ignore` files that are used by other programs such as `rg` or `ag`.

If you want `fd` to ignore these patterns globally, you can put them in `fd`'s global ignore file.
This is usually located in `~/.config/fd/ignore`.

You may wish to include `.git/` in your `fd/ignore` file so that `.git` directories, and their contents
are not included in output if you use the `--hidden` option.

### Deleting files

You can use `fd` to remove all files and directories that are matched by your search pattern.
If you only want to remove files, you can use the `--exec-batch`/`-X` option to call `rm`. For
example, to recursively remove all `.DS_Store` files, run:
``` bash
> fd -H '^\.DS_Store$' -tf -X rm
```
If you are unsure, always call `fd` without `-X rm` first. Alternatively, use `rm`s "interactive"
option:
``` bash
> fd -H '^\.DS_Store$' -tf -X rm -i
```

If you also want to remove a certain class of directories, you can use the same technique. You will
have to use `rm`s `--recursive`/`-r` flag to remove directories.

> [!NOTE]
> There are scenarios where using `fd … -X rm -r` can cause race conditions: if you have a
path like `…/foo/bar/foo/…` and want to remove all directories named `foo`, you can end up in a
situation where the outer `foo` directory is removed first, leading to (harmless) *"'foo/bar/foo':
No such file or directory"* errors in the `rm` call.

### Command-line options

This is the output of `fd -h` (same as `fd --help`).

```
A program to find entries in your filesystem with regex and glob based matching. By default, fd respects gitignore rules, ignores hidden directories, and is case insensitive.

Usage: fd [OPTIONS] [pattern] [path...]

Arguments:
  [pattern]
          the search pattern (a regular expression, unless '--glob' is used; optional). If your pattern starts with a dash (-), make sure to pass '--' first, or it will be considered as a flag (fd -- '-foo').
  [path...]
          the root directories for the filesystem search (optional). If omitted, search the current working directory.

Options:
  -H, --hidden
          Search hidden files and directories. Files and directories are considered to be hidden if their name starts with a `.` sign (dot). Any files or directories that are ignored due to the rules described by --no-ignore are still ignored unless otherwise specified. The flag can be overridden with --no-hidden.
  --no-hidden
          Overrides --hidden.
  -I, --no-ignore
          Do not respect .(git|fd)ignore files. The flag can be overridden with --ignore.
  --ignore
          Overrides --no-ignore.
  --no-ignore-vcs
          Do not respect .gitignore files. The flag can be overridden with --ignore-vcs.
  --ignore-vcs
          Overrides --no-ignore-vcs.
  --no-require-git
          Do not require a git repository to respect gitignores. By default, fd will only respect global gitignore rules, .gitignore rules, and local exclude rules if fd detects that you are searching inside a git repository. This flag allows you to relax this restriction such that fd will respect all git related ignore rules regardless of whether you're searching in a git repository or not. This flag can be disabled with --require-git.
  --require-git
          Overrides --no-require-git.
  --no-ignore-parent
          Do not respect .(git|fd)ignore files in parent directories. The flag can be overridden with --ignore-parent.
  --ignore-parent
          Overrides --no-ignore-parent.
  --no-global-ignore-file
          Do not respect the global ignore file.
  -u, --unrestricted
          Perform an unrestricted search, including ignored and hidden files. This is an alias for '--no-ignore --hidden'. [repeatable]
  -s, --case-sensitive
          Perform a case-sensitive search. By default, fd uses case-insensitive searches, unless the pattern contains an uppercase character (smart case).
  -i, --ignore-case
          Perform a case-insensitive search. By default, fd uses case-insensitive searches, unless the pattern contains an uppercase character (smart case).
  -g, --glob
          Perform a glob-based search instead of a regular expression search.
  --regex
          Perform a regular-expression based search (default). This can be used to override --glob.
  -F, --fixed-strings
          Treat the pattern as a literal string instead of a regular expression. Note that this also performs substring comparison. If you want to match on an exact filename, consider using '--glob' or '--exact' instead. [aliases: literal]
  --exact
          Perform an exact match. This is equivalent to '--fixed-strings' but requires the pattern to match the entire filename (or path if '--full-path' is used), rather than a substring. Special regex characters in the pattern are treated as literal characters.
  --and <pattern>...
          Add additional required search patterns, all of which must be matched. Multiple additional patterns can be specified. The patterns are regular expressions, unless '--glob' or '--fixed-strings' is used. [value: <pattern>, required value, repeatable]
  -a, --absolute-path
          Shows the full path starting from the root as opposed to relative paths. The flag can be overridden with --relative-path.
  --relative-path
          Overrides --absolute-path.
  -l, --list-details
          Use a detailed listing format like 'ls -l'. This is basically an alias for '--exec-batch ls -l' with some additional 'ls' options. This can be used to see more metadata, to show symlink targets and to achieve a deterministic sort order.
  -L, --follow
          By default, fd does not descend into symlinked directories. Using this flag, symbolic links are also traversed. Flag can be overridden with --no-follow. [aliases: dereference]
  --no-follow
          Overrides --follow.
  -p, --full-path
          By default, the search pattern is only matched against the filename (or directory name). Using this flag, the pattern is matched against the full (absolute) path. Example: fd --glob -p '**/.git/config'
  -0, --print0
          Separate search results by the null character (instead of newlines). Useful for piping results to 'xargs'.
  -d, --max-depth <depth>
          Limit the directory traversal to a given depth. By default, there is no limit on the search depth. [value: <depth>, required value, aliases: maxdepth]
  --min-depth <depth>
          Only show search results starting at the given depth. See also: '--max-depth' and '--exact-depth' [value: <depth>, required value, aliases: mindepth]
  --exact-depth <depth>
          Only show search results at the exact given depth. This is an alias for '--min-depth <depth> --max-depth <depth>'. [value: <depth>, required value]
  -E, --exclude <glob>...
          Exclude files/directories that match the given glob pattern. This overrides any other ignore logic. Multiple exclude patterns can be specified. Examples: --exclude '*.pyc' --exclude node_modules [value: <glob>, required value, repeatable]
  --prune
          Do not traverse into directories that match the search criteria. If you want to exclude specific directories, use the '--exclude=…' option.
  -t, --type <filetype>...
          Filter by type: file (f), directory (d/dir), symlink (l), executable (x), empty (e), socket (s), pipe (p), char-device (c), block-device (b). This option can be specified more than once to include multiple file types. [value: <filetype>, required value, repeatable]
  -e, --extension <ext>...
          Filter by extension. By default, this matches all entry types (including directories whose names end with the extension). Use `--type` to restrict the search to specific types. Multiple extensions can be specified. If you want to search for files without extension, you can use the regex '^[^.]+$' as a normal search pattern. [value: <ext>, required value, repeatable]
  -S, --size <size>...
          Limit results based on the size of files using the format <+-><NUM><UNIT>. '+': file size must be greater than or equal to this '-': file size must be less than or equal to this. If neither '+' nor '-' is specified, file size must be exactly equal to this. 'NUM': The numeric size (e.g. 500) 'UNIT': The units for NUM. Allowed unit values: 'b': bytes 'k': kilobytes (base ten, 10^3 = 1000 bytes) 'm': megabytes 'g': gigabytes 't': terabytes 'ki': kibibytes (base two, 2^10 = 1024 bytes) 'mi': mebibytes 'gi': gibibytes 'ti': tebibytes [value: <size>, required value, repeatable]
  --changed-within <date|dur>
          Filter by file modification time (newer than). The argument can be provided as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min). If the time is not specified, it defaults to 00:00:00. '--change-newer-than', '--newer', or '--changed-after' can be used as aliases. Examples: --changed-within 2weeks --change-newer-than '2018-10-27 10:00:00' --newer 2018-10-27 --changed-after 1day [value: <date|dur>, required value, aliases: change-newer-than, newer, changed-after]
  --changed-before <date|dur>
          Filter by file modification time (older than). The argument can be provided as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min). '--change-older-than' or '--older' can be used as aliases. Examples: --changed-before '2018-10-27 10:00:00' --change-older-than 2weeks --older 2018-10-27 [value: <date|dur>, required value, aliases: change-older-than, older]
  -o, --owner <user:group>
          Filter by owning user and/or group. Format: [(user|uid)][:(group|gid)]. Either side is optional. Precede either side with a '!' to exclude files instead. Examples: --owner john --owner :students --owner '!john:students' [value: <user:group>, required value]
  --format <fmt>
          Print results according to template with placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension. [value: <fmt>, required value]
  -x, --exec <cmd>...
          Execute a command for each search result in parallel (use --threads=1 for sequential command execution). There is no guarantee of the order commands are executed in. All positional arguments following --exec are considered to be arguments to the command - not to fd. Use ';' to terminate the command template if you need to continue passing fd arguments afterwards. Placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension, '{{': literal '{', '}}': literal '}'. If no placeholder is present, an implicit "{}" at the end is assumed. Examples: fd -e zip -x unzip; fd -e h -e cpp -x clang-format -i; fd . src -x echo; fd -e jpg -x convert {} {.}.png [value: <cmd>, required value, repeatable]
  -X, --exec-batch <cmd>...
          Execute the given command once, with all search results as arguments. The order of the arguments is non-deterministic. Placeholders: '{}': path, '{/}': basename, '{//}': parent directory, '{.}': path without file extension, '{/.}': basename without file extension, '{{': literal '{', '}}': literal '}'. If no placeholder is present, an implicit "{}" at the end is assumed. Examples: fd -g 'test_*.py' -X vim; fd -e rs -X wc -l [value: <cmd>, required value, repeatable]
  --batch-size <size>
          Maximum number of arguments to pass to the command given with -X. If the number of results is greater than the given size, the command given with -X is run again with remaining arguments. A batch size of zero means there is no limit (default), but note that batching might still happen due to OS restrictions on the maximum length of command lines. [value: <size>, default: 0, required value]
  --ignore-file <path>...
          Add a custom ignore-file in '.gitignore' format. These files have a low precedence. [value: <path>, required value, repeatable]
  -c, --color <when>
          Declare when to use color for the pattern match output [default: auto] [possible values: auto, always, never]. Environment: NO_COLOR is respected (non-empty disables color in auto mode); LS_COLORS defines file styling. [value: <when>, default: auto, required value]
  --hyperlink[=<when>]
          Add a terminal hyperlink to a file:// url for each path in the output. If no argument is given, auto mode is used. This doesn't do anything for --exec and --exec-batch. [default: never] [possible values: auto, always, never] [value: <when>, default: never, optional value (use =) [bare flag: auto], aliases: hyper]
  --ignore-contain <name>...
          Ignore directories containing the named entry. [value: <name>, required value, repeatable]
  -j, --threads <num>
          Set number of threads to use for searching & executing (default: number of available CPU cores). [value: <num>, required value]
  --max-buffer-time
          Milliseconds to buffer before streaming search results to console. [required value]
  --max-results <count>
          Limit the number of search results to 'count' and quit immediately. [value: <count>, required value]
  -1, --max-one-result
          Limit the search to a single result and quit immediately. This is an alias for '--max-results=1'.
  -q, --quiet
          When the flag is present, the program does not print anything and will return with an exit code of 0 if there is at least one match. Otherwise, the exit code will be 1. '--has-results' can be used as an alias. [aliases: has-results]
  --show-errors
          Enable the display of filesystem errors for situations such as insufficient permissions or dead symlinks.
  -C, --base-directory <path>
          Change the current working directory of fd to the provided path. This means that search results will be shown with respect to the given base path. Note that relative paths which are passed to fd via the positional <path> argument or the '--search-path' option will also be resolved relative to this directory. [value: <path>, required value]
  --path-separator <separator>
          Set the path separator to use when printing file paths. The default is '/'. [value: <separator>, required value]
  --search-path <search-path>...
          Provide paths to search as an alternative to the positional <path> argument. Changes the usage to `fd [OPTIONS] --search-path <path> --search-path <path2> [<pattern>]` [value: <search-path>, required value, repeatable]
  --strip-cwd-prefix[=<when>]
          By default, relative paths are prefixed with './' when -x/--exec, -X/--exec-batch, or -0/--print0 are given, to reduce the risk of a path starting with '-' being treated as a command line option. Use this flag to change this behavior. If this flag is used without a value, it is equivalent to passing "always". [possible values: auto, always, never] [value: <when>, optional value (use =) [bare flag: always]]
  --one-file-system
          By default, fd will traverse the file system tree as far as other options dictate. With this flag, fd ensures that it does not descend into a different file system than the one it started in. Comparable to the -mount or -xdev filters of find(1). [aliases: mount, xdev]
  -h, --help
          Print help.
  -V, --version
          Print version.

Examples:
  fd . src -x echo
  fd -e rs -X wc -l
  fd --glob '**/.git/config'

Bugs can be reported on GitHub: https://github.com/sharkdp/fd/issues
```

Note that options can be given after the pattern and/or path as well.

## Benchmark

Let's search my home folder for files that end in `[0-9].jpg`. It contains ~750,000
subdirectories and about a 4 million files. For averaging and statistical analysis, I'm using
[hyperfine](https://github.com/sharkdp/hyperfine). The following benchmarks are performed
with a "warm"/pre-filled disk-cache (results for a "cold" disk-cache show the same trends).

Let's start with `find`:
```
Benchmark 1: find ~ -iregex '.*[0-9]\.jpg$'
  Time (mean ± σ):     19.922 s ±  0.109 s
  Range (min … max):   19.765 s … 20.065 s
```

`find` is much faster if it does not need to perform a regular-expression search:
```
Benchmark 2: find ~ -iname '*[0-9].jpg'
  Time (mean ± σ):     11.226 s ±  0.104 s
  Range (min … max):   11.119 s … 11.466 s
```

Now let's try the same for `fd`. Note that `fd` performs a regular expression
search by default. The options `-u`/`--unrestricted` option is needed here for
a fair comparison. Otherwise `fd` does not have to traverse hidden folders and
ignored paths (see below):
```
Benchmark 3: fd -u '[0-9]\.jpg$' ~
  Time (mean ± σ):     854.8 ms ±  10.0 ms
  Range (min … max):   839.2 ms … 868.9 ms
```
For this particular example, `fd` is approximately **23 times faster** than `find -iregex`
and about **13 times faster** than `find -iname`. By the way, both tools found the exact
same 546 files :smile:.

**Note**: This is *one particular* benchmark on *one particular* machine. While we have
performed a lot of different tests (and found consistent results), things might
be different for you! We encourage everyone to try it out on their own. See
[this repository](https://github.com/sharkdp/fd-benchmarks) for all necessary scripts.

Concerning *fd*'s speed, a lot of credit goes to the `regex` and `ignore` crates that are
also used in [ripgrep](https://github.com/BurntSushi/ripgrep) (check it out!).

## Troubleshooting

### `fd` does not find my file!

Remember that `fd` ignores hidden directories and files by default. It also ignores patterns
from `.gitignore` files. If you want to make sure to find absolutely every possible file, always
use the options `-u`/`--unrestricted` option (or `-HI` to enable hidden and ignored files):
``` bash
> fd -u …
```

Also remember that by default, `fd` only searches based on the filename and
doesn't compare the pattern to the full path. If you want to search based on the
full path (similar to the `-path` option of `find`) you need to use the `--full-path`
(or `-p`) option.

### Colorized output

`fd` can colorize files by extension, just like `ls`. In order for this to work, the environment
variable [`LS_COLORS`](https://linux.die.net/man/5/dir_colors) has to be set. Typically, the value
of this variable is set by the `dircolors` command which provides a convenient configuration format
to define colors for different file formats.
On most distributions, `LS_COLORS` should be set already. If you are looking
for alternative, more complete (or more colorful) variants, see [here](https://github.com/sharkdp/vivid),
[here](https://github.com/seebi/dircolors-solarized) or
[here](https://github.com/trapd00r/LS_COLORS).

`fd` also honors the [`NO_COLOR`](https://no-color.org/) environment variable.

### `fd` doesn't seem to interpret my regex pattern correctly

A lot of special regex characters (like `[]`, `^`, `$`, ..) are also special characters in your
shell. If in doubt, always make sure to put single quotes around the regex pattern:

``` bash
> fd '^[A-Z][0-9]+$'
```

If your pattern starts with a dash, you have to add `--` to signal the end of command line
options. Otherwise, the pattern will be interpreted as a command-line option. Alternatively,
use a character class with a single hyphen character:

``` bash
> fd -- '-pattern'
> fd '[-]pattern'
```

### "Command not found" for `alias`es or shell functions

Shell `alias`es and shell functions can not be used for command execution via `fd -x` or
`fd -X`. In `zsh`, you can make the alias global via `alias -g myalias="…"`. In `bash`,
you can use `export -f my_function` to make available to child processes. You would still
need to call `fd -x bash -c 'my_function "$1"' bash`. For other use cases or shells, use
a (temporary) shell script.

### Placeholders in `-x`/`-X`

Depending on your shell, you may need to quote the placeholders (`{}`, `{/}`, `{//}`,
`{.}`, `{/.}`) to prevent the shell from interpreting them before `fd` sees them.

## Integration with other programs

### Using fd with `fzf`

You can use *fd* to generate input for the command-line fuzzy finder [fzf](https://github.com/junegunn/fzf):
``` bash
export FZF_DEFAULT_COMMAND='fd --type file'
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
```

Then, you can type `vim <Ctrl-T>` on your terminal to open fzf and search through the fd-results.

Alternatively, you might like to follow symbolic links and include hidden files (but exclude `.git` folders):
``` bash
export FZF_DEFAULT_COMMAND='fd --type file --follow --hidden --exclude .git'
```

You can even use fd's colored output inside fzf by setting:
``` bash
export FZF_DEFAULT_COMMAND="fd --type file --color=always"
export FZF_DEFAULT_OPTS="--ansi"
```

For more details, see the [Tips section](https://github.com/junegunn/fzf#tips) of the fzf README.

### Using fd with `rofi`

[*rofi*](https://github.com/davatorium/rofi) is a graphical launch menu application that is able to create menus by reading from *stdin*. Piping `fd` output into `rofi`s `-dmenu` mode creates fuzzy-searchable lists of files and directories.

#### Example

Create a case-insensitive searchable multi-select list of *PDF* files under your `$HOME` directory and open the selection with your configured PDF viewer. To list all file types, drop the `-e pdf` argument.

``` bash
fd --type f -e pdf . $HOME | rofi -keep-right -dmenu -i -p FILES -multi-select | xargs -I {} xdg-open {}
```

To modify the list that is presented by rofi, add arguments to the `fd` command. To modify the search behaviour of rofi, add arguments to the `rofi` command.

### Using fd with `emacs`

The emacs package [find-file-in-project](https://github.com/technomancy/find-file-in-project) can
use *fd* to find files.

After installing `find-file-in-project`, add the line `(setq ffip-use-rust-fd t)` to your
`~/.emacs` or `~/.emacs.d/init.el` file.

In emacs, run `M-x find-file-in-project-by-selected` to find matching files. Alternatively, run
`M-x find-file-in-project` to list all available files in the project.

### Printing the output as a tree

To format the output of `fd` as a file-tree you can use the `tree` command with
`--fromfile`:
```bash
❯ fd | tree --fromfile
```

This can be more useful than running `tree` by itself because `tree` does not
ignore any files by default, nor does it support as rich a set of options as
`fd` does to control what to print:
```bash
❯ fd --extension rs | tree --fromfile
.
├── build.rs
└── src
    ├── app.rs
    └── error.rs
```

On bash and similar you can simply create an alias:
```bash
❯ alias as-tree='tree --fromfile'
```

### Using fd with `xargs` or `parallel`

Note that `fd` has a builtin feature for [command execution](#command-execution) with
its `-x`/`--exec` and `-X`/`--exec-batch` options. If you prefer, you can still use
it in combination with `xargs`:
``` bash
> fd -0 -e rs | xargs -0 wc -l
```
Here, the `-0` option tells *fd* to separate search results by the NULL character (instead of
newlines). In the same way, the `-0` option of `xargs` tells it to read the input in this way.

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/fd-find.svg)](https://repology.org/project/fd-find/versions)

### On Ubuntu
*... and other Debian-based Linux distributions.*

If you run Ubuntu 19.04 (Disco Dingo) or newer, you can install the
[officially maintained package](https://packages.ubuntu.com/fd-find):
```
apt install fd-find
```
Note that the binary is called `fdfind` as the binary name `fd` is already used by another package.
It is recommended that after installation, you add a link to `fd` by executing command
`ln -s $(which fdfind) ~/.local/bin/fd`, in order to use `fd` in the same way as in this documentation.
Make sure that `$HOME/.local/bin` is in your `$PATH`.

If you use an older version of Ubuntu, you can download the latest `.deb` package from the
[release page](https://github.com/sharkdp/fd/releases) and install it via:
``` bash
dpkg -i fd_9.0.0_amd64.deb # adapt version number and architecture
```

Note that the .deb packages on the release page for this project still name the executable `fd`.

### On Debian

If you run Debian Buster or newer, you can install the
[officially maintained Debian package](https://tracker.debian.org/pkg/rust-fd-find):
```
apt-get install fd-find
```
Note that the binary is called `fdfind` as the binary name `fd` is already used by another package.
It is recommended that after installation, you add a link to `fd` by executing command
`ln -s $(which fdfind) ~/.local/bin/fd`, in order to use `fd` in the same way as in this documentation.
Make sure that `$HOME/.local/bin` is in your `$PATH`.

Note that the .deb packages on the release page for this project still name the executable `fd`.

### On Fedora

Starting with Fedora 28, you can install `fd` from the official package sources:
``` bash
dnf install fd-find
```

### On Alpine Linux

You can install [the fd package](https://pkgs.alpinelinux.org/packages?name=fd)
from the official sources, provided you have the appropriate repository enabled:
```
apk add fd
```

### On Arch Linux

You can install [the fd package](https://www.archlinux.org/packages/extra/x86_64/fd/) from the official repos:
```
pacman -S fd
```
You can also install fd [from the AUR](https://aur.archlinux.org/packages/fd-git).

### On Gentoo Linux

You can use [the fd ebuild](https://packages.gentoo.org/packages/sys-apps/fd) from the official repo:
```
emerge -av fd
```

### On openSUSE Linux

You can install [the fd package](https://software.opensuse.org/package/fd) from the official repo:
```
zypper in fd
```

### On Void Linux

You can install `fd` via xbps-install:
```
xbps-install -S fd
```

### On ALT Linux

You can install [the fd package](https://packages.altlinux.org/en/sisyphus/srpms/fd/) from the official repo:
```
apt-get install fd
```

### On Solus

You can install [the fd package](https://github.com/getsolus/packages/tree/main/packages/f/fd) from the official repo:
```
eopkg install fd
```

### On RedHat Enterprise Linux (RHEL) 8/9/10, Almalinux 8/9/10, EuroLinux 8/9 or Rocky Linux 8/9/10

You can install [the `fd` package](https://copr.fedorainfracloud.org/coprs/tkbcopr/fd/) from Fedora Copr.

```bash
dnf copr enable tkbcopr/fd
dnf install fd
```

A different version using the [slower](https://github.com/sharkdp/fd/pull/481#issuecomment-534494592) malloc [instead of jemalloc](https://bugzilla.redhat.com/show_bug.cgi?id=2216193#c1) is also available from the EPEL8/9 repo as the package `fd-find`.

### On macOS

You can install `fd` with [Homebrew](https://formulae.brew.sh/formula/fd):
```
brew install fd
```

… or with MacPorts:
```
port install fd
```

### On GuixOS

You can install [the fd package](https://guix.gnu.org/en/packages/fd-8.1.1/) from the official repo:
```
guix install fd
```

### On Mise

You can use [mise](https://github.com/jdx/mise) to install `fd` with a command like this:
```
mise use -g fd@latest
```

### On NixOS / via Nix

You can use the [Nix package manager](https://nixos.org/nix/) to install `fd`:
```
nix-env -i fd
```

### Via Flox

You can use [Flox](https://flox.dev) to install `fd` into a Flox environment:
```
flox install fd
```

### On FreeBSD

You can install [the fd-find package](https://www.freshports.org/sysutils/fd) from the official repo:
```
pkg install fd-find
```

### From npm

On Linux and macOS, you can install the [fd-find](https://npm.im/fd-find) package:

```
npm install -g fd-find
```

### From source

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install *fd* via:
```
cargo install fd-find
```
Note that rust version *1.90.0* or later is required.

`make` is also needed for the build.

### From binaries

The [release page](https://github.com/sharkdp/fd/releases) includes precompiled binaries for Linux and macOS. Statically-linked binaries are also available: look for archives with `musl` in the file name.

## Development
```bash
git clone https://github.com/sharkdp/fd

# Build
cd fd
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

### Completions

#### From Release Archives

Pre-built completion files are included in the release archives (`.tar.gz`/`.zip`) on the
[Releases page](https://github.com/sharkdp/fd/releases), in the `autocomplete` directory.
To use these completions:

- **bash**: Source the `fd.bash` file in your `~/.bashrc`, or place it in a directory that gets sourced automatically.
- **zsh**: Move `_fd` to a directory in your `fpath` (e.g., `~/.zfunc`).
- **fish**: Copy `fd.fish` to `~/.config/fish/completions/`.
- **powershell**: Source `_fd.ps1` from one of your [profile scripts](https://learn.microsoft.com/en-us/powershell/scripting/learn/shell/creating-profiles?view=powershell-7.5).

#### Generate from fd

You can also generate completions directly using `fd --gen-completions <shell>`:

```bash
# Bash
fd --gen-completions bash > ~/.local/share/bash-completion/completions/fd

# Zsh (ensure ~/.zfunc is in your fpath)
fd --gen-completions zsh > ~/.zfunc/_fd

# Fish
fd --gen-completions fish > ~/.config/fish/completions/fd.fish

# PowerShell
fd --gen-completions powershell >> $PROFILE
```

## Maintainers

- [sharkdp](https://github.com/sharkdp)
- [tmccombs](https://github.com/tmccombs)
- [tavianator](https://github.com/tavianator)

## License

`fd` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
