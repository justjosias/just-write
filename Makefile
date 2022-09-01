NAME=jw
PREFIX=~/.local
INSTALL=install
CARGO=cargo
TAURI=cargo tauri

CLI_PATH=cli
GUI_PATH=gui/src-tauri

cli-build:
	$(CARGO) build --release --manifest-path=$(CLI_PATH)/Cargo.toml

cli-install: cli-build
	$(INSTALL) -d $(PREFIX)/bin
	$(INSTALL) -m755 $(CLI_PATH)/target/release/$(NAME) $(PREFIX)/bin
	
	$(INSTALL) -d $(PREFIX)/share/bash-completion/completions/
	$(INSTALL) -m755 $(CLI_PATH)/$(NAME).bash-completion $(PREFIX)/share/bash-completion/completions/$(NAME)

gui-build:
	$(TAURI) build

uninstall:
	$(RM) $(PREFIX)/bin/$(NAME)
	$(RM) $(PREFIX)/share/bash-completion/completions/$(NAME)

