if ! nix-shell --help &> /dev/null
then
    echo "nix-shell could not be found! Are you sure it is installed correctly?"
    exit
fi

echo "Creating three releases of Social-Context inside ./release"

[ ! -d "./release" ] && mkdir "./release"

echo "Create release with full index & no signals or time index..."

#Get new dna.yaml with correct props & build language
cp ./hc-dna/workdir/dna_basic_full_index.yaml ./hc-dna/workdir/dna.yaml
cd hc-dna && build.sh
cd ../
npm run build

#Check if full_index directory exists, if not create
[ ! -d "./release/full_index" ] && mkdir "./release/full_index"

#Copy the build files to the release dir
cp ./build/bundle.js ./release/full_index/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/full_index/social-context.dna



echo "Create release with Partial index (no wildcard) + signals + time index..."

#Get new dna.yaml with correct props & build language
cp ./hc-dna/workdir/dna_signals.yaml ./hc-dna/workdir/dna.yaml
cd hc-dna && build.sh
cd ../
npm run build

#Create the dna_signals release dir
[ ! -d "./release/dna_signals" ] && mkdir "./release/dna_signals"

#Copy the build files to the release dir
cp ./build/bundle.js ./release/dna_signals/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/dna_signals/social-context.dna



echo "Create release with full index + time index but no signals..."

cp ./hc-dna/workdir/dna_time_index.yaml ./hc-dna/workdir/dna.yaml
cd hc-dna && build.sh
cd ../
npm run build

[ ! -d "./release/full_time_index" ] && mkdir "./release/full_time_index"

cp ./build/bundle.js ./release/full_time_index/bundle.js
cp ./hc-dna/workdir/social-context.dna ./release/full_time_index/social-context.dna