README.md: README.tpl src/bin/readme.rs lists/*.txt
	cargo run -q --bin readme > README.md

.PHONY: count
count:
ifeq ($(CRATE),)
	@echo "make count CRATE=something"
else
	@cargo run -q --bin count-deps ${CRATE}
endif
