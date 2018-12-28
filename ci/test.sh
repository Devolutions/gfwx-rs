set -ex

main() {
    cross build --target $TARGET --lib

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross build --target $TARGET --examples
    cross build --target $TARGET --benches

    if [ $TARGET = $CODECOV_TARGET ]; then
        cargo test
        cargo test --release
    else
        cross test --target $TARGET
        cross test --target $TARGET --release
    fi

    cross bench -- --test

    bash ci/func_tests.sh ci/test_images
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
