#!/usr/bin/env -S cargo +nightly-2024-05-22 -Zscript

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

fn main() {
    println!("{} {}", std::env::consts::OS, std::env::consts::ARCH);
}
