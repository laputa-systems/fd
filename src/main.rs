#[macro_use]
mod error;
mod cli;
mod config;
mod dir_entry;
mod exec;
mod exit_codes;
mod filesystem;
mod filetypes;
mod filter;
mod fmt;
mod hyperlink;
mod output;
#[cfg(test)]
mod git_rerun;
mod regex_helper;
mod sanitize;
mod walk;

#[cfg(all(
    not(target_os = "android"),
    not(target_os = "macos"),
    not(target_os = "freebsd"),
    not(target_os = "openbsd"),
    not(target_os = "illumos"),
    not(all(target_env = "musl", target_pointer_width = "32")),
    not(target_arch = "riscv64"),
    feature = "use-jemalloc"
))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    let code = cli::run(std::env::args_os());
    code.exit();
}
