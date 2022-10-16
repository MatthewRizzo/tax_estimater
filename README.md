# Tax Estimater

Playing around with estimating taxes in a programatic / variatic way. Useful
for financial planning. Implemented in Rust (yay!)

## Development

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
pre-commit install

# Done!
```
