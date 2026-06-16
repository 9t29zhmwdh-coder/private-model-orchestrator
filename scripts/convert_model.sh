#!/usr/bin/env bash
# convert_model.sh — AOT-compile a Core ML package to .mlmodelc
#
# Usage: ./scripts/convert_model.sh <model.mlpackage> <output_dir> [compute_units]
# compute_units: ALL (default) | CPU_AND_NE | CPU_AND_GPU | CPU_ONLY
#
# Requirements: Python 3.11+, coremltools >= 7.0, macOS 13+
set -euo pipefail

MODEL_PATH="${1:?Usage: $0 <model.mlpackage> <output_dir> [compute_units]}"
OUTPUT_DIR="${2:?Usage: $0 <model.mlpackage> <output_dir> [compute_units]}"
COMPUTE_UNITS="${3:-ALL}"

if ! command -v python3 &>/dev/null; then
  echo "Error: python3 not found" >&2; exit 1
fi

python3 - <<PYEOF
import coremltools as ct
import os, sys

model_path    = "${MODEL_PATH}"
output_dir    = "${OUTPUT_DIR}"
compute_units = ct.ComputeUnit["${COMPUTE_UNITS}"]

print(f"Loading  : {model_path}")
model = ct.models.MLModel(model_path, compute_units=compute_units)

os.makedirs(output_dir, exist_ok=True)
out_path = os.path.join(output_dir, os.path.basename(model_path).replace(".mlpackage", ".mlmodelc"))

print(f"Compiling: {out_path} (compute_units={compute_units})")
model.save(out_path)
print(f"Done     : {out_path}")
PYEOF
