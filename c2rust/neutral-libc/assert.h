/* Neutral <assert.h> for translation, see README.md. */

#undef assert

#ifdef NDEBUG
#define assert(e) ((void)0)
#else
void mpd_c2rust_assert_fail(const char *expr);
#define assert(e) ((e) ? (void)0 : mpd_c2rust_assert_fail(#e))
#endif
