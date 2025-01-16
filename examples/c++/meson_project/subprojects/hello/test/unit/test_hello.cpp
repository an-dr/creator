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


int test_hello(void)
{
    int result = -1;

    result = hello("@{Test Subject}@");
    if (result != 0)
    {
        return -1;
    }

    return result;
}


int main()
{
    int result = -1;
    result = test_hello();
    if (result != 0)
    {
        return -1;
    }
    return 0;
}
