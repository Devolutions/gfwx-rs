set -ex

main() {
    cross build --target $TARGET --lib

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross build --target $TARGET --examples
    cross build --target $TARGET --benches

    cross test --target $TARGET
    cross test --target $TARGET --release

    cross bench -- --test
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
