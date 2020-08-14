#! /bin/bash
virtualenv3 not_alone_py
source not_alone_py/bin/activate
pip install -r src/requirements.txt
cargo build --release
maturin develop
ln not_alone_py/bin/activate pyenv
echo "$ source env"
echo "$ python client.py or python server.py"