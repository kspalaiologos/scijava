/*
    Copyright (C) 2011 Sebastian Pancratz
    Copyright (C) 2008, 2009 William Hart

    This file is part of FLINT.

    FLINT is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 2.1 of the License, or
    (at your option) any later version.  See <https://www.gnu.org/licenses/>.
*/

#include <stdlib.h>
#include "nmod_poly.h"

void nmod_poly_factor_fit_length(nmod_poly_factor_t fac, slong len)
{
    if (len > fac->alloc)
    {
        /* At least double number of allocated coeffs */
        if (len < 2 * fac->alloc)
            len = 2 * fac->alloc;
        nmod_poly_factor_realloc(fac, len);
    }
}
