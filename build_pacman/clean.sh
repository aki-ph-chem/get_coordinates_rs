#!/usr/bin/bash

pkgname="coordinates_picker"
pkgver=1.0
tarballname="${pkgname}-${pkgver}"
pkgname="${tarballname}-1-x86_64"

flist=("${tarballname}" "${tarballname}.tar.gz" "${pkgname}.pkg.tar.zst" "src" "pkg")

for f in "${flist[@]}"; do
    if [ -e "${f}" ]; then
        rm -r "${f}" 
    fi
done
