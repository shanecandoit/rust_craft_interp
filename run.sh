
# stop on first error
set -e

# build
echo "build"
cargo build

echo "format"
cargo fmt

# run tests
echo "test"
./target/debug/craft_interp tests/test_add.rox

# run interactive
echo "run"
./target/debug/craft_interp

