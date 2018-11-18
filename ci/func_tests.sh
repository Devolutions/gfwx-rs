SCRIPT_DIR=$(dirname $(realpath -s "$0"))

build_ref_app() {
    mkdir ref_build
    cd ref_build && \
    cmake ${SCRIPT_DIR}/../reference_app/ && \
    make && \
    cp reference_test_app ../ && \
    cd .. && \
    rm -r ref_build
}

build_examples() {
    cargo build --release --examples --manifest-path ${SCRIPT_DIR}/../Cargo.toml && \
    cp ${SCRIPT_DIR}/../target/release/examples/compress ./ && \
    cp ${SCRIPT_DIR}/../target/release/examples/decompress ./ && \
    cp ${SCRIPT_DIR}/../target/release/examples/compare ./
}

IMAGES_DIR=$(pwd)/$1

mkdir /tmp/gfwx
pushd /tmp/gfwx && \
build_ref_app && \
build_examples && \
${SCRIPT_DIR}/test_helper.py ${IMAGES_DIR}
popd
