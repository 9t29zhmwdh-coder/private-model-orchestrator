# AOT Conversion Reference — Private Model Orchestrator

## Overview

Foundation Models should be distributed as **AOT-compiled `.mlmodelc`** bundles for production deployment. AOT compilation:

- Eliminates JIT overhead at first inference
- Locks the compute unit assignment (ANE / GPU / CPU)
- Reduces startup latency by up to 60% on Apple Silicon
- Enables codesign verification before execution

## Conversion Pipeline

```
Source Model (.gguf / .safetensors / .pt)
           │
           ▼
  coremltools.convert()
  (produces .mlpackage — interpreted, development)
           │
           ▼
  coremltools.models.MLModel.save()
  with compute_units=ComputeUnit.ALL
  (produces .mlmodelc — AOT compiled, production)
           │
           ▼
  codesign --sign "Developer ID Application: ..." model.mlmodelc
           │
           ▼
  Checksum (SHA-256): shasum -a 256 model.mlmodelc.tar.gz
           │
           ▼
  ModelBundle { variant: MlModelC, checksum: "..." }
```

## Using the Conversion Script

```bash
# Install requirements
pip install coremltools torch

# Convert (uses ALL compute units by default — ANE preferred on Apple Silicon)
./scripts/convert_model.sh ./models/mistral-7b.mlpackage ./dist/

# ANE-only (fastest on M-series, no GPU fallback)
./scripts/convert_model.sh ./models/mistral-7b.mlpackage ./dist/ CPU_AND_NE
```

## Compute Unit Guidance

| Flag | Use case |
|------|----------|
| `ALL` | Default — OS selects optimal unit per layer |
| `CPU_AND_NE` | Maximum ANE utilisation, best perf/watt |
| `CPU_AND_GPU` | GPU-heavy models (vision), NE unavailable |
| `CPU_ONLY` | Debugging, compatibility testing |

## Codesigning

```bash
codesign \
  --sign "Developer ID Application: Rafael Yilmaz (TEAMID)" \
  --timestamp \
  --options runtime \
  dist/mistral-7b.mlmodelc
```

Required for notarization and Gatekeeper bypass on managed fleets.

## Bundle Checksum

```bash
shasum -a 256 dist/mistral-7b.mlmodelc | awk '{print $1}'
```

Store in `ModelBundle.checksum`. PMO verifies this at load time.
