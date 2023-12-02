#!/usr/bin/env bash

set -euo pipefail

cargo bench

cp "target/criterion/Big Table/report/violin.svg" big-table.svg
cp "target/criterion/Teams/report/violin.svg" teams.svg
