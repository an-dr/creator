#!/usr/bin/env pwsh
# *************************************************************************
#
# Copyright (c) 2024 Andrei Gramakov. All rights reserved.
#
# This file is licensed under the terms of the MIT license.  
# For a copy, see: https://opensource.org/licenses/MIT
#
# site:    https://agramakov.me
# e-mail:  mail@agramakov.me
#
# *************************************************************************

$env:DEVTEMPLATES_ENV = "$PSScriptRoot/templates"

function Copy-DevTemplate {
    python $PSScriptRoot/scripts/devtemplate.py
}

New-Alias -Name devtemplate -Value Copy-DevTemplate
New-Alias -Name dt -Value Copy-DevTemplate
