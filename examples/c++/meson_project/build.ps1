#!/usr/bin/env pwsh

pushd $PSScriptRoot

meson setup --wipe ./build
meson compile -C build

popd
