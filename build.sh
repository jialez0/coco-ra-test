root_dir=$(pwd)
KBC=cc_kbc
ENC_MOD=cc_kbc_enc

git submodule init
git submodule update

rm -rf bin && mkdir bin

cd image-pulling-tool && cargo build && cp target/debug/image-pulling-tool ../bin
cd $root_dir
cd attestation-agent && make KBC=$KBC && cp app/target/x86_64-unknown-linux-gnu/release/attestation-agent ../bin
cd $root_dir
cd attestation-agent/sample_keyprovider && cargo build --features $ENC_MOD && cp target/debug/sample_keyprovider ../../bin
cd $root_dir
cd kbs && make kbs && cp target/debug/kbs ../bin


