#!/bin/bash

set -euo pipefail
# 1B
RED=$'\e'"[0;31m"
RESET=$'\e'"[0m"


die() {
    echo "$RED""$@""$RESET" 1>&2
    exit 1
}

install_darwin() {
    if ! cargo lambda --help 1>&2; then
        if ! command -v brew 1>&2; then
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        if ! brew tap | grep cargo-lambda/cargo-lambda 1>&2; then
            brew tap cargo-lambda/cargo-lambda
        fi
        brew install cargo-lambda
    fi
}

main() {
    case "$(uname -s | tr A-Z a-z)" in
    darwin)
        install_darwin;;
        brew tap cargo-lambda/cargo-lambda;;
    linux) ;;
    *) die "Unsupported OS: $(uname -s)"
    esac
}

main "$@"