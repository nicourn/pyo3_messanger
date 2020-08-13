.PHONY: build_py, build_py, run_py, run

PY_ENV := not_alone_py/bin/activate

build_rs: src/lib.rs src/client.rs src/server.rs
	cargo build

build_py: build_rs
	maturin develop

run_py: 
	python3 ./src/main.py

run: build_py
	run_py


