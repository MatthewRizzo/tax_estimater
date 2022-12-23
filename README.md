# Tax Estimater

Playing around with estimating taxes in a programatic / variatic way. Useful
for financial planning. Implemented in Rust (yay!)

## Running the Project

Simply build / run using the client!

```bash
cargo run --manifest-path=estimate-client/Cargo.toml

## Or cd to the client dir
cd estimate-client
cargo run
```

## Development

### Short Development Install

Just run the following and everything will be handled for you!

```bash

curl -SL https://raw.githubusercontent.com/MatthewRizzo/mattrizzo_devops/main/bootstrap.sh | sudo bash

```

### Long Development Install

This is a more long-winded explanation.

This repository runs pre-commit hooks using
[`pre-commit`](https://pre-commit.com/). As part of that hook, a markdown linter
([`mdl`](https://github.com/markdownlint/markdownlint)) is used. If you want to
contribute to this repository, you will need to install both. Each have guides
on their pages. However, a brief synopsis is included below.

```bash
# Install pre-commit

## Debian distros
apt install pre-commit

## Redhat/Fedora distros
dnf install pre-commit

# Install mdl (and its required package manager)
apt install gem
gem install mdl

# On Redhat distros
dnf install gem
gem install mdl

# Install / setup pre-commit
pre-commit install --hook-type pre-commit --hook-type pre-push
python -m pip install poetry
poetry install
# Done!
```

## Currently Cannot Handle

* Misc withholdings. i.e. Family leave, medicare, social security

## TODO

* Add json for federal tax bracket - map income brackets -> tax rate
* Add json for state tax brackets
* Implement a true "Server" using protobufs and async threading
