

/*********************************************************************/
/*                   Testcases for Newton Division                   */
/*********************************************************************/

static void
_mpd_qtest_newtondiv(int action, mpd_t *q, const mpd_t *a, const mpd_t *b,
                     const mpd_context_t *ctx, uint32_t *status)
{
    MPD_NEW_STATIC(aligned,0,0,0,0);
    mpd_uint_t ld;
    mpd_ssize_t shift, exp, tz;
    mpd_ssize_t newsize;
    mpd_ssize_t ideal_exp;
    mpd_uint_t rem;
    uint8_t sign_a = mpd_sign(a);
    uint8_t sign_b = mpd_sign(b);


    if (mpd_isspecial(a) || mpd_isspecial(b)) {
        if (mpd_qcheck_nans(q, a, b, ctx, status)) {
            return;
        }
        _mpd_qdiv_inf(q, a, b, ctx, status);
        return;
    }
    if (mpd_iszerocoeff(b)) {
        if (mpd_iszerocoeff(a)) {
            mpd_seterror(q, MPD_Division_undefined, status);
        }
        else {
            mpd_setspecial(q, sign_a^sign_b, MPD_INF);
            *status |= MPD_Division_by_zero;
        }
        return;
    }
    if (mpd_iszerocoeff(a)) {
        exp = a->exp - b->exp;
        _settriple(q, sign_a^sign_b, 0, exp);
        mpd_qfinalize(q, ctx, status);
        return;
    }

    shift = (b->digits - a->digits) + ctx->prec + 1;
    ideal_exp = a->exp - b->exp;
    exp = ideal_exp - shift;
    if (shift > 0) {
        if (!mpd_qshiftl(&aligned, a, shift, status)) {
            mpd_seterror(q, MPD_Malloc_error, status);
            goto finish;
        }
        a = &aligned;
    }
    else if (shift < 0) {
        shift = -shift;
        if (!mpd_qshiftl(&aligned, b, shift, status)) {
            mpd_seterror(q, MPD_Malloc_error, status);
            goto finish;
        }
        b = &aligned;
    }


    newsize = a->len - b->len + 1;
    if ((q != b && q != a) || (q == b && newsize > b->len)) {
        if (!mpd_qresize(q, newsize, status)) {
            mpd_seterror(q, MPD_Malloc_error, status);
            goto finish;
        }
    }

    {
        MPD_NEW_STATIC(r,0,0,0,0);
        _mpd_base_ndivmod(q, &r, a, b, status);
        if (mpd_isspecial(q) || mpd_isspecial(&r)) {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_del(&r);
            goto finish;
        }
        rem = !mpd_iszerocoeff(&r);
        mpd_del(&r);
        newsize = q->len;
    }

    newsize = _mpd_real_size(q->data, newsize);
    /* resize to smaller cannot fail */
    mpd_qresize(q, newsize, status);
    mpd_set_flags(q, sign_a^sign_b);
    q->len = newsize;
    mpd_setdigits(q);

    shift = ideal_exp - exp;
    if (rem) {
        ld = mpd_lsd(q->data[0]);
        if (ld == 0 || ld == 5) {
            q->data[0] += 1;
        }
    }
    else if (action == SET_IDEAL_EXP && shift > 0) {
        tz = mpd_trail_zeros(q);
        shift = (tz > shift) ? shift : tz;
        mpd_qshiftr_inplace(q, shift);
        exp += shift;
    }

    q->exp = exp;


finish:
    mpd_del(&aligned);
    mpd_qfinalize(q, ctx, status);
}

static void
mpd_qtest_newtondiv(mpd_t *q, const mpd_t *a, const mpd_t *b,
                    const mpd_context_t *ctx, uint32_t *status)
{
    _mpd_qtest_newtondiv(SET_IDEAL_EXP, q, a, b, ctx, status);
}

static void
_mpd_qtest_ndivmod(mpd_t *q, mpd_t *r, const mpd_t *a, const mpd_t *b,
                   const mpd_context_t *ctx, uint32_t *status)
{
    MPD_NEW_STATIC(aligned,0,0,0,0);
    mpd_ssize_t qsize, rsize;
    mpd_ssize_t ideal_exp, expdiff, shift;
    uint8_t sign_a = mpd_sign(a);
    uint8_t sign_ab = mpd_sign(a)^mpd_sign(b);


    ideal_exp = (a->exp > b->exp) ?  b->exp : a->exp;
    if (mpd_iszerocoeff(a)) {
        if (!mpd_qcopy(r, a, status)) {
            goto nanresult; /* GCOV_NOT_REACHED */
        }
        r->exp = ideal_exp;
        _settriple(q, sign_ab, 0, 0);
        return;
    }

    expdiff = mpd_adjexp(a) - mpd_adjexp(b);
    if (expdiff < 0) {
        if (a->exp > b->exp) {
            /* positive and less than b->digits - a->digits */
            shift = a->exp - b->exp;
            if (!mpd_qshiftl(r, a, shift, status)) {
                goto nanresult;
            }
            r->exp = ideal_exp;
        }
        else {
            if (!mpd_qcopy(r, a, status)) {
                goto nanresult;
            }
        }
        _settriple(q, sign_ab, 0, 0);
        return;
    }
    if (expdiff > ctx->prec) {
        *status |= MPD_Division_impossible;
        goto nanresult;
    }


    /*
     * At this point we have:
     *   (1) 0 <= a->exp + a->digits - b->exp - b->digits <= prec
     *   (2) a->exp - b->exp >= b->digits - a->digits
     *   (3) a->exp - b->exp <= prec + b->digits - a->digits
     */
    if (a->exp != b->exp) {
        shift = a->exp - b->exp;
        if (shift > 0) {
            /* by (3), after the shift a->digits <= prec + b->digits */
            if (!mpd_qshiftl(&aligned, a, shift, status)) {
                goto nanresult;
            }
            a = &aligned;
        }
        else  {
            shift = -shift;
            /* by (2), after the shift b->digits <= a->digits */
            if (!mpd_qshiftl(&aligned, b, shift, status)) {
                goto nanresult;
            }
            b = &aligned;
        }
    }


    qsize = a->len - b->len + 1;
    if (!(q == a && qsize < a->len) && !(q == b && qsize < b->len)) {
        if (!mpd_qresize(q, qsize, status)) {
            goto nanresult;
        }
    }

    rsize = b->len;
    if (!(r == a && rsize < a->len)) {
        if (!mpd_qresize(r, rsize, status)) {
            goto nanresult;
        }
    }

    {
        _mpd_base_ndivmod(q, r, a, b, status);
        if (mpd_isspecial(q) || mpd_isspecial(r)) {
            goto nanresult;
        }
        qsize = q->len;
        rsize = r->len;
    }

    qsize = _mpd_real_size(q->data, qsize);
    /* resize to smaller cannot fail */
    mpd_qresize(q, qsize, status);
    q->len = qsize;
    mpd_setdigits(q);
    mpd_set_flags(q, sign_ab);
    q->exp = 0;
    if (q->digits > ctx->prec) {
        *status |= MPD_Division_impossible;
        goto nanresult;
    }

    rsize = _mpd_real_size(r->data, rsize);
    /* resize to smaller cannot fail */
    mpd_qresize(r, rsize, status);
    r->len = rsize;
    mpd_setdigits(r);
    mpd_set_flags(r, sign_a);
    r->exp = ideal_exp;

out:
    mpd_del(&aligned);
    return;

nanresult:
    mpd_setspecial(q, MPD_POS, MPD_NAN);
    mpd_setspecial(r, MPD_POS, MPD_NAN);
    goto out;
}

static void
mpd_qtest_newtondivint(mpd_t *q, const mpd_t *a, const mpd_t *b,
                       const mpd_context_t *ctx, uint32_t *status)
{
    MPD_NEW_STATIC(r,0,0,0,0);
    uint8_t sign = mpd_sign(a)^mpd_sign(b);

    if (mpd_isspecial(a) || mpd_isspecial(b)) {
        if (mpd_qcheck_nans(q, a, b, ctx, status)) {
            return;
        }
        if (mpd_isinfinite(a) && mpd_isinfinite(b)) {
            mpd_seterror(q, MPD_Invalid_operation, status);
            return;
        }
        if (mpd_isinfinite(a)) {
            mpd_setspecial(q, sign, MPD_INF);
            return;
        }
        if (mpd_isinfinite(b)) {
            _settriple(q, sign, 0, 0);
            return;
        }
        /* debug */
        abort(); /* GCOV_NOT_REACHED */
    }
    if (mpd_iszerocoeff(b)) {
        if (mpd_iszerocoeff(a)) {
            mpd_seterror(q, MPD_Division_undefined, status);
        }
        else {
            mpd_setspecial(q, sign, MPD_INF);
            *status |= MPD_Division_by_zero;
        }
        return;
    }


    _mpd_qtest_ndivmod(q, &r, a, b, ctx, status);
    mpd_del(&r);
    mpd_qfinalize(q, ctx, status);
}

void
mpd_qtest_newtonrem(mpd_t *r, const mpd_t *a, const mpd_t *b, const mpd_context_t *ctx,
                    uint32_t *status)
{
    MPD_NEW_STATIC(q,0,0,0,0);

    if (mpd_isspecial(a) || mpd_isspecial(b)) {
        if (mpd_qcheck_nans(r, a, b, ctx, status)) {
            return;
        }
        if (mpd_isinfinite(a)) {
            mpd_seterror(r, MPD_Invalid_operation, status);
            return;
        }
        if (mpd_isinfinite(b)) {
            mpd_qcopy(r, a, status);
            mpd_qfinalize(r, ctx, status);
            return;
        }
        /* debug */
        abort(); /* GCOV_NOT_REACHED */
    }
    if (mpd_iszerocoeff(b)) {
        if (mpd_iszerocoeff(a)) {
            mpd_seterror(r, MPD_Division_undefined, status);
        }
        else {
            mpd_seterror(r, MPD_Invalid_operation, status);
        }
        return;
    }

    _mpd_qtest_ndivmod(&q, r, a, b, ctx, status);
    mpd_del(&q);
    mpd_qfinalize(r, ctx, status);
}

static void
mpd_qtest_newtondivmod(mpd_t *q, mpd_t *r, const mpd_t *a, const mpd_t *b,
                       const mpd_context_t *ctx, uint32_t *status)
{
    uint8_t sign = mpd_sign(a)^mpd_sign(b);

    if (mpd_isspecial(a) || mpd_isspecial(b)) {
        if (mpd_qcheck_nans(q, a, b, ctx, status)) {
            mpd_qcopy(r, q, status);
            return;
        }
        if (mpd_isinfinite(a)) {
            if (mpd_isinfinite(b)) {
                mpd_setspecial(q, MPD_POS, MPD_NAN);
            }
            else {
                mpd_setspecial(q, sign, MPD_INF);
            }
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status |= MPD_Invalid_operation;
            return;
        }
        if (mpd_isinfinite(b)) {
            if (!mpd_qcopy(r, a, status)) {
                mpd_seterror(q, MPD_Malloc_error, status);
                return;
            }
            mpd_qfinalize(r, ctx, status);
            _settriple(q, sign, 0, 0);
            return;
        }
        /* debug */
        abort(); /* GCOV_NOT_REACHED */
    }
    if (mpd_iszerocoeff(b)) {
        if (mpd_iszerocoeff(a)) {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status |= MPD_Division_undefined;
        }
        else {
            mpd_setspecial(q, sign, MPD_INF);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status |= (MPD_Division_by_zero|MPD_Invalid_operation);
        }
        return;
    }

    _mpd_qtest_ndivmod(q, r, a, b, ctx, status);
    mpd_qfinalize(q, ctx, status);
    mpd_qfinalize(r, ctx, status);
}

void
mpd_test_newtondiv(mpd_t *q, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx)
{
    uint32_t status = 0;
    mpd_qtest_newtondiv(q, a, b, ctx, &status);
    mpd_addstatus_raise(ctx, status);
}

void
mpd_test_newtondivint(mpd_t *q, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx)
{
    uint32_t status = 0;
    mpd_qtest_newtondivint(q, a, b, ctx, &status);
    mpd_addstatus_raise(ctx, status);
}

void
mpd_test_newtonrem(mpd_t *r, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx)
{
    uint32_t status = 0;
    mpd_qtest_newtonrem(r, a, b, ctx, &status);
    mpd_addstatus_raise(ctx, status);
}

void
mpd_test_newtondivmod(mpd_t *q, mpd_t *r, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx)
{
    uint32_t status = 0;
    mpd_qtest_newtondivmod(q, r, a, b, ctx, &status);
    mpd_addstatus_raise(ctx, status);
}
