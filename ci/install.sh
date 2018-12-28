set -ex

install_opencv_linux() {
    sudo apt-get -yq install build-essential cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libjasper-dev libdc1394-22-dev
	git clone --branch '3.4.4' --depth 1 -q https://github.com/opencv/opencv.git opencv || true
    if [ ! -d opencv/build ] ; then
        mkdir opencv/build
    fi
    cd opencv/build && \
    cmake -D BUILD_LIST=imgcodecs -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local .. >/dev/null && \
    make -s -j $(nproc) && \
    sudo make -s install
    cd ../..
}

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Install OpenCV
    if [ -z $DISABLE_TESTS ]; then
        if [ $TRAVIS_OS_NAME = linux ]; then
            install_opencv_linux
        else
            brew install glog >/dev/null
            brew install opencv >/dev/null
        fi
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup target install armv7-apple-ios
            ;;
        armv7s-apple-ios)
            rustup target install armv7s-apple-ios
            ;;
        i386-apple-ios)
            rustup target install i386-apple-ios
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
    esac

    # Install tools for codecov
    if [ $TARGET = $CODECOV_TARGET ]; then
        sudo apt-get -yq --no-install-suggests --no-install-recommends install libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev libiberty-dev
    fi

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target
}

main
