# lich

![ci](https://github.com/egilsster/lich-rs/workflows/ci/badge.svg?branch=main)

**What is this?**

A License checker?

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

```sh
TBD
```

## Usage

```sh
lich package.json --repo=approved-licenses --owner=egilsster --token $(GITHUB_TOKEN)
```
