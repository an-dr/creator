#!/usr/bin/env pwsh

pushd $PSScriptRoot

# test
meson test -C build --print-errorlogs

popd
