#!/bin/sh


VERSIONED_FILES=\
"libmpdec/Makefile.vc libmpdec/configure libmpdec/configure.ac"

for file in $VERSIONED_FILES; do
    sed -i "s/@RELEASE_VERSION@/$@/g" $file
done


