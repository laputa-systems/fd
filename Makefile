PROFILE?=release
EXE=target/$(PROFILE)/fd
prefix=/usr/local
bindir=$(prefix)/bin
datadir=$(prefix)/share
exe_name=fd
ifdef VERSION
	ARCHIVE_NAME = fd-v$(VERSION)
else
	ARCHIVE_NAME = fd
endif
export ARCHIVE_NAME
archive_path=package/$(ARCHIVE_NAME).tar.gz

$(EXE): Cargo.toml src/**/*.rs
	cargo build --profile $(PROFILE) --locked

.PHONY: completions
completions: autocomplete/_fd autocomplete/_fdfind autocomplete/fdfind.bash autocomplete/fdfind.fish

comp_dir=@mkdir -p autocomplete

autocomplete/_fd: contrib/completion/_fd
	$(comp_dir)
	cp $< $@

autocomplete/_fdfind: contrib/completion/_fdfind
	$(comp_dir)
	cp $< $@

autocomplete/fdfind.bash: contrib/completion/fdfind.bash
	$(comp_dir)
	cp $< $@

autocomplete/fdfind.fish: contrib/completion/fdfind.fish
	$(comp_dir)
	cp $< $@

archive: $(archive_path)

$(archive_path): completions $(EXE)
	bash scripts/create-archive.sh

install: $(EXE)
	install -Dm755 $(EXE) $(DESTDIR)$(bindir)/fd
	install -Dm644 contrib/completion/_fd $(DESTDIR)/$(datadir)/zsh/site-functions/_fd
	install -Dm644 contrib/completion/_fdfind $(DESTDIR)/$(datadir)/zsh/site-functions/_fdfind
	install -Dm644 contrib/completion/fdfind.bash $(DESTDIR)/$(datadir)/bash-completion/completions/fdfind
	install -Dm644 contrib/completion/fdfind.fish $(DESTDIR)/$(datadir)/fish/vendor_completions.d/fdfind.fish
	install -Dm644 doc/fd.1 $(DESTDIR)/$(datadir)/man/man1/$(exe_name).1
