#!/usr/bin/env bash
# Give the template its real name. Run once, from the repository root:
#
#   ./rename.sh newname "Newname"
#
# `newname` is the slug (lowercase letters, digits, dashes): crate names, binaries, folders.
# "Newname" is the display name. Every occurrence of mymodule / Mymodule is replaced in
# place; files are renamed; nothing is deleted. Then delete this script yourself and commit.
set -euo pipefail
slug="${1:?slug, e.g. permitt}"
name="${2:?display name, e.g. Permitt}"
case "$slug" in *[!a-z0-9-]*) echo "slug: lowercase letters, digits and dashes only" >&2; exit 2;; esac
cd "$(dirname "${BASH_SOURCE[0]}")"
grep -rl --exclude-dir=target --exclude-dir=.git --exclude=rename.sh -e mymodule -e Mymodule . \
  | xargs sed -i -e "s/mymodule/$slug/g" -e "s/Mymodule/$name/g"
[ -f assets/mymodule.desktop ] && mv assets/mymodule.desktop "assets/$slug.desktop"
echo "renamed to $slug / $name — now: cargo test, review README.md and the pilotage files, delete rename.sh, commit"
