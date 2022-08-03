# Paths.
PREFIX = /usr/local
MANPREFIX = ${PREFIX}/share/man
COMPPREFIX = /usr/share

all:
	# Compile & generate manpage and completions for Bash, Fish, Zsh.
	cargo build --release

install:
	# Install binary.
	mkdir -p ${DESTDIR}${PREFIX}/bin
	setcap 'cap_sys_nice=eip' target/release/datalogger
	install -Dm 755 target/release/datalogger -t "${DESTDIR}${PREFIX}/bin"
	# Install manpage.
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	install -Dm 644 man/datalogger.1 -t "${DESTDIR}${MANPREFIX}/man1"
	# Install shell completions.
	mkdir -p ${COMPPREFIX}/bash-completion/completions\
		${COMPPREFIX}/zsh/site_functions\
		${COMPPREFIX}/fish/vendor_completions.d
	install -Dm 644 completions/datalogger.bash -t "${COMPPREFIX}/bash-completion/completions"
	install -Dm 644 completions/_datalogger -t "${COMPPREFIX}/zsh/site-functions"
	install -Dm 644 completions/datalogger.fish -t "${COMPPREFIX}/fish/vendor_completions.d"

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/datalogger\
		${DESTDIR}${MANPREFIX}/man1/datalogger.1\
		${COMPPREFIX}/bash-completion/completions/datalogger.bash\
		${COMPPREFIX}/zsh/site-functions/_datalogger\
		${COMPPREFIX}/fish/vendor_completions.d/datalogger.fish

clean:
	cargo clean
	rm -rf completions man

.PHONY: all install uninstall clean
