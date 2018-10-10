# This script takes care of building the crate and packaging it for release.

PKG_NAME="{{PKG_NAME}}"

set -ex

main() {
  local src=$(pwd) \
  stage \
  linking_args

  case $TRAVIS_OS_NAME in
  linux)
    stage=$(mktemp -d)
    ;;
  osx)
    stage=$(mktemp -d -t tmp)
    ;;
  esac

  test -f Cargo.lock || cargo generate-lockfile

  # TODO: combine with -C lto
  case $TYPE in
  static)
    linking_args="--crate-type staticlib"
    ;;
  *)
    linking_args="--crate-type cdylib"
    ;;
  esac

  cross rustc --lib --target $TARGET --release -- $linking_args

  case $TYPE-$TRAVIS_OS_NAME in
  static-*)
    cp target/$TARGET/release/lib$PKG_NAME.a $stage/
    ;;
  *-osx)
    cp target/$TARGET/release/lib$PKG_NAME.dylib $stage/
    ;;
  *)
    cp target/$TARGET/release/lib$PKG_NAME.so $stage/
    ;;
  esac

  cd $stage
  tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
  cd $src

  rm -rf $stage
}

main
