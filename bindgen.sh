#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

gen() {
    bindgen \
        --use-core \
        --default-enum-style rust_non_exhaustive \
        --with-derive-default \
        --with-derive-hash \
        --with-derive-partialeq \
        --with-derive-eq \
        --with-attribute-custom-enum '.*=#[cfg_attr(feature = "num_enum", derive(num_enum::IntoPrimitive,num_enum::TryFromPrimitive,num_enum::UnsafeFromPrimitive))]' \
        --with-attribute-custom-struct '.*=#[cfg_attr(feature = "zerocopy", derive(zerocopy_derive::FromZeroes,zerocopy_derive::FromBytes,zerocopy_derive::AsBytes))]' \
        "$@"
}

gen include/linux.h -o src/linux.rs
gen include/macos.h -o src/macos.rs -- -D __APPLE__
git apply patches/*
