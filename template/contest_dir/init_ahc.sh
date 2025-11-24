set -eu
tools_url=$1

# get tools
wget -O tools.zip $tools_url
unzip tools.zip
rm tools.zip

# get contest name
contest=`pwd | awk -F "/" '{ print $NF }'`

# set_project_name
cat "/home/yohei/git/rust-kyopro/template/contest_dir/Cargo.toml" | dasel -r toml -w json | jq ".package.name=\"${contest}\"" | dasel -r json -w toml > tmp.txt
cp tmp.txt Cargo.toml
rm tmp.txt

# init pahcer
pahcer init -p $contest -o min -l rust

# clone pahcer-studio
git clone git@github.com:yunix-kyopro/pahcer-studio.git
cd pahcer-studio
yarn install
