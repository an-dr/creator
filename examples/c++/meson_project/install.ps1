#!/usr/bin/env pwsh

pushd $PSScriptRoot

# test
meson install -C build

popd
