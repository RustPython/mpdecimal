@ECHO OFF

set PYTHON="C:\Program Files (x86)\Python27\python.exe"

if not exist runtest.exe goto error

call gettests.bat

echo.
echo Running official tests ...
echo.
runtest.exe --all official.decTest
runtest_static.exe --all official.decTest
runtest_static2.exe --all official.decTest

echo.
echo Running additional tests ...
echo.
runtest.exe --all additional.decTest
runtest_static.exe --all additional.decTest
runtest_static2.exe --all additional.decTest

echo.
echo Running coverage tests ...
echo.
cov.exe
test_transpose.exe
fntcov.exe

if not exist deccheck.exe goto NEXT
echo.
echo Running decNumber tests ...
echo.
deccheck.exe
:NEXT

echo.
echo Running long tests ...
echo.
ppro_mulmod.exe
if exist mpd_mpz_add.exe mpd_mpz_add.exe -q
if exist mpd_mpz_sub.exe mpd_mpz_sub.exe -q
if exist mpd_mpz_mul.exe mpd_mpz_mul.exe -q
if exist mpd_mpz_divmod.exe mpd_mpz_divmod.exe -q
karatsuba_fnt.exe -q
karatsuba_fnt2.exe -q


echo.
echo Running locale and format tests ...
echo.
%PYTHON% genrandlocale.py | runtest.exe -
%PYTHON% genrandformat.py | runtest.exe -
%PYTHON% genlocale.py | runtest.exe -

goto finish


:error
echo.
echo error: the tests must be built first
echo.

:finish


