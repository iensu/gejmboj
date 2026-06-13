#!/usr/bin/env bash

# Runs a blarg cpu_instrs test and validates the result using gameboy-doctor

set -xe

rom_path="$1"

RUST_LOG=info cargo run -p gejmboj_app --example blargg_test -- "$rom_path"

rom_filename="${rom_path##*/}"
rom_number="${rom_filename%%-*}"
rom_number="$((10#$rom_number))"

gameboy-doctor blargg-test.log cpu_instrs "$rom_number"
