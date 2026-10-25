#!/bin/sh

SYSTEM=`uname -s`
case $SYSTEM in
    AIX)
        PATCH=/opt/freeware/bin/patch
        ;;
    *)
        PATCH=patch
        ;;
esac

SHA256=`which sha256sum`
if [ ! -f "$SHA256" ]; then
    SHA256=`which shasum`
fi

if [ ! -d decNumber ]; then
    WGET=`which wget`
    if [ ! -f "$WGET" ]; then
        printf "\n*** getdn.sh: error: could not find wget. In order to run the decNumber tests,\n"
        printf "    download http://download.icu-project.org/files/decNumber/decNumber-icu-368.zip\n"
        printf "    and extract the archive into mpdecimal-x.y/tests/decNumber.\n\n"
        exit 1
    fi
    printf "\nGetting decNumber ... \n\n"
    $WGET http://www.bytereef.org/repository/decNumber-icu-368.zip || exit 1
    $SHA256 -c SHA256SUM || exit 1
    unzip -qq decNumber-icu-368.zip &&
    cd decNumber && $PATCH < ../dnloop.patch
fi
