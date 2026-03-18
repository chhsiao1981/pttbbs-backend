#!/bin/bash

BBSHOME=${1:-/home/bbs}

echo "BBSHOME: ${BBSHOME}"

if [ -f ${BBSHOME}/.PASSWDS ]; then
    echo "init-ptt.sh: ${BBSHOME}/.PASSWDS already exists."
    exit 0
fi

tar -C `dirname ${BBSHOME}` -zxvf bbs.tar.gz
