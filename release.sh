#!/bin/bash
set -e

if ! nix-shell --help &> /dev/null
then
    echo "nix-shell could not be found! Are you sure it is installed correctly?"
    exit
fi

echo "Creating four releases of Social-Context inside ./release"

[ ! -d "./release" ] && mkdir "./release"

echo "Create release with no features enabled..."

#Get new dna.yaml with correct props & build language
cp ./hc-dna/workdir/dna_basic.yaml ./hc-dna/workdir/dna.yaml
npm install && npm run build

#Check if basic_index directory exists, if not create
[ ! -d "./release/basic" ] && mkdir "./release/basic"

#Copy the build files to the release dir
cp ./build/bundle.js ./release/basic/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/basic/social-context.dna



echo "Create release with all features enabled..."

#Get new dna.yaml with correct props & build language
cp ./hc-dna/workdir/dna_full.yaml ./hc-dna/workdir/dna.yaml
npm install && npm run build

#Create the full_features release dir
[ ! -d "./release/full_features" ] && mkdir "./release/full_features"

#Copy the build files to the release dir
cp ./build/bundle.js ./release/full_features/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/full_features/social-context.dna



echo "Create release with time index but no signals..."

cp ./hc-dna/workdir/dna_time_index.yaml ./hc-dna/workdir/dna.yaml
npm install && npm run build

[ ! -d "./release/time_index" ] && mkdir "./release/time_index"

cp ./build/bundle.js ./release/time_index/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/time_index/social-context.dna



echo "Create release with signals but no time index..."

cp ./hc-dna/workdir/dna_signals.yaml ./hc-dna/workdir/dna.yaml
npm install && npm run build

[ ! -d "./release/signals" ] && mkdir "./release/signals"

cp ./build/bundle.js ./release/signals/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/signals/social-context.dna

cd ./release/basic && zip -j -r ../basic.zip ./* && cd -
cd ./release/full_features && zip -j -r ../full_features.zip ./* && cd -
cd ./release/time_index && zip -j -r ../time_index.zip ./* && cd -
cd ./release/signals && zip -j -r ../signals.zip ./* && cd -