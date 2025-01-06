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

# This script is used to export the necessary environment variables

# Set the path to the directory where the script is located
SCRIPT_ROOT=$(dirname $(readlink -f "$0"))

# Export the necessary environment variables
export DEVTEMPLATES_ENV="$SCRIPT_ROOT/templates"
export PATH="$SCRIPT_ROOT/tools:$PATH"
alias dt="devtemplate"


# Clear SCRIPT_ROOT var
unset SCRIPT_ROOT

