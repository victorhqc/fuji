#!/bin/sh

mkdir ./deps
cd ./deps

rm -rf exiftool

echo "Cloning exiftool@13.21"
git clone --depth 1 --branch 13.21 https://github.com/exiftool/exiftool.git

echo "Cleaning up exiftool"

cd ./exiftool
rm -rf .git
rm -rf html
rm -f README
rm -f Changes
rm -f windows_exiftool
rm -f perl-Image-Exiftool.spec
rm -f MANIFEST
rm -f Makefile.PL
rm -f META.yml
rm -f META.json
