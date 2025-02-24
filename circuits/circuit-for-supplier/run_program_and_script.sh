echo "1/ Compile program..."
cd program
cargo prove build

if [ $? -ne 0 ]; then
  exit 1
fi

echo "\t"

echo "2/ Executing program with generating a proof."
cd ../script
RUST_LOG=info cargo run --release -- --prove

# echo "2/ Executing program without generating a proof."
# cd ../script
# RUST_LOG=info cargo run --release -- --execute

echo "Done"