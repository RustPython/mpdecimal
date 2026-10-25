@ECHO OFF

if exist decNumber goto END

if exist decNumber-icu-368.zip goto UNZIP
powershell -Command "wget http://www.bytereef.org/repository/decNumber-icu-368.zip -outfile decNumber-icu-368.zip"

:UNZIP
powershell -Command "Expand-Archive decNumber-icu-368.zip -DestinationPath decNumberTmp"
cd decNumberTmp
move decNumber ..
cd ..
rmdir decNumberTmp

git apply dnloop.patch

:END
