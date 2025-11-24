set -eu
contest_name=$1
project_root='/home/yohei/git/rust-kyopro'
cp -r "${project_root}/template/contest_dir/" "./${contest_name}/"
cd $contest_name
cargo build