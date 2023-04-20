#!/usr/bin/bash

src="coordinates_picker"

pkgname="coordinates_picker"
pkgver=1.0
tarballname="${pkgname}-${pkgver}"

if [ ! -e "${tarballname}" ]; then
    mkdir "${tarballname}"
    cp -r "../${src}/src" "${tarballname}/src"
    cp "../${src}/Cargo.toml" "${tarballname}/"
    cp "../${src}/Cargo.lock" "${tarballname}/"
fi


if [ ! -e "${tarballname}.tar.gz" ]; then
    tar -cvzf "${tarballname}.tar.gz" "${tarballname}"
fi

makepkg -g >> PKGBUILD
makepkg -s
