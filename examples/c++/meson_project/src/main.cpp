// *************************************************************************
//
// Copyright (c) 2025 Developer. All rights reserved.
//
// This file is licensed under the terms of the MIT license.  
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://inter.net
// e-mail:  developer@inter.net
//
// *************************************************************************


#include "hello.h"

int main(int argc, char *argv[])
{
    (void)argc; // unused
    return hello(argv[1]) >= 0 ? 0 : -1;
}
