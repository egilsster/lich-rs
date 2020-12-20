# lich

![ci](https://github.com/egilsster/lich-rs/workflows/ci/badge.svg?branch=main)

**What is this?**

A <b>Li</b>cense <b>ch</b>ecker

**What is that?**

Great question! Companies that ship software that contains third party packages
need to make sure all the licenses for these packages have been vetted by company
lawyers. This is done to ensure the software a company releases is legally allowed
to use it.

**How does it work?**

A consumer has to maintain a Github project with a list of files, each of the item in that
list represents a license that has gone through the proper intake (approved by tech leads,
other tech relevant parties, and the law department). This application compares the
dependency file (`package.json` at first) in the folder where its run. It uses that data
and compares it to whats in the aforementioned Github project to find unapproved
dependency usages.

Fits perfectly in Github status check.

## Installing

Homebrew

```sh
brew tap egilsster/lich
brew install lich
```

`wget`

```sh
λ TAG=v0.1.1 wget https://github.com/egilsster/lich-rs/releases/download/$TAG/lich-x86_64-apple-darwin.zip
tar xf lich-x86_64-apple-darwin.zip -C /usr/local/bin
```

## Usage

```sh
λ lich <DEPENDENCY FILE> --repo <REPO> --owner <OWNER> [--token <TOKEN>]
```

Against a private repo using the `main` branch:

```sh
λ lich package.json --repo=approved-licenses --owner=egilsster --token $GITHUB_TOKEN
```
