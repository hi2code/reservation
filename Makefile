init:
	pre-commit install
	cargo install --locked cargo-deny
	cargo install cargo-nextest --locked
