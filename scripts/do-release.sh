#!/bin/bash

# cd to the root of the project

cd $(dirname $0)/.. || exit

old_version=$(git describe --tags --abbrev=0)

echo "Old version: $old_version"

NEW_VERSION=$(semver -i patch "$old_version")
echo "New version: $NEW_VERSION"
sed -i '' "s/^version = ".*"/version = \"$NEW_VERSION\"/" ./Cargo.toml
cargo build
git cliff --tag "$NEW_VERSION" -o CHANGELOG.md
git add CHANGELOG.md
git add ./Cargo.*
git commit -m "Release $NEW_VERSION"
git tag "$NEW_VERSION"

git push
git push --tags
