#!/bin/bash

BASE_PATH="./tmp"

# Check if required arguments are provided
if [ $# -lt 2 ]; then
    echo "Usage: $0 BENCHMARK_NAME IMPL_VALUES..."
    echo "Example: $0 p1910_memory 1 2 3"
    exit 1
fi

# Parse arguments
BENCHMARK_NAME=$1
shift  # Remove the first argument (BENCHMARK_NAME)
IMPL_VALUES=("$@")  # Store all remaining arguments as array

# Ensure the tmp directory exists
mkdir -p "$BASE_PATH"

# Ensure the benchmark is built
echo "Building benchmark: $BENCHMARK_NAME"
cargo build --release --bench $BENCHMARK_NAME

# Find the actual binary - the newest one that matches the pattern
# shellcheck disable=SC2038
MEM_BINARY=$(find target/release/deps -name "$BENCHMARK_NAME-*" -type f -executable | xargs ls -t | head -1)

if [ -z "$MEM_BINARY" ]; then
    echo "Error: Could not find the benchmark binary for $BENCHMARK_NAME"
    exit 1
fi

echo "Using binary: $MEM_BINARY"

# Run benchmarks for each implementation
for IMPL in "${IMPL_VALUES[@]}"; do
    echo "Running implementation $IMPL memory benchmark..."
    # shellcheck disable=SC2098
    # shellcheck disable=SC2097
    IMPL=$IMPL valgrind --tool=massif --massif-out-file="$BASE_PATH/${BENCHMARK_NAME}_massif_${IMPL}.out" \
      --detailed-freq=1 --max-snapshots=100 --time-unit=B $MEM_BINARY

    echo "Generating report for implementation $IMPL..."
    ms_print "$BASE_PATH/${BENCHMARK_NAME}_massif_${IMPL}.out" > "$BASE_PATH/${BENCHMARK_NAME}_profile_${IMPL}.txt"
done

echo "Memory profiling complete!"
echo "Reports saved to:"
for IMPL in "${IMPL_VALUES[@]}"; do
    echo "  $BASE_PATH/${BENCHMARK_NAME}_profile_${IMPL}.txt"
done

# Construct massif-visualizer command with all output files
VISUALIZER_CMD="massif-visualizer"
for IMPL in "${IMPL_VALUES[@]}"; do
    VISUALIZER_CMD+=" $BASE_PATH/${BENCHMARK_NAME}_massif_${IMPL}.out"
done

# Launch massif-visualizer with all output files for comparison
echo "Launching massif-visualizer for comparing implementations..."
echo "Running: $VISUALIZER_CMD"
$VISUALIZER_CMD
