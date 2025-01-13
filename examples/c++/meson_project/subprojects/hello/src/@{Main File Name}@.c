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

#include <stdio.h>

int hello(const char *name)
{
    if (name != NULL)
    {
        printf("@{Greating}@ %s!\n", name);
        return 0;
    }
    else
    {
        printf("@{Greating}@!\n");
        return 1;
    }
    return -1; // should never happen
}
