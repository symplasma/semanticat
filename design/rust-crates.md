# Rust Crates

[Self-Contained Rust Text Embeddings & Clustering](https://aistudio.google.com/prompts/1M9Sm8XeUIXNuEgx1GQxFauLNW60eARLq)

## Embedding Crates

- [fastembed](https://crates.io/crates/fastembed)
- [embed_anything](https://crates.io/crates/embed_anything)
- [model2vec-rs](https://crates.io/crates/model2vec-rs): This looks like the simplest one to start with.

## Clustering Crates

- [sheaf](https://crates.io/crates/sheaf)
- [avx-clustering](https://crates.io/crates/avx-clustering): This looks faster, so let's start with this.

## Original guidance

For building a fully self-contained Rust application that performs text embedding and clustering without external dependencies (no APIs, no Ollama, no local daemons), you should look at crates built on **ONNX Runtime** or the **Candle** ecosystem.

### 1. Best Crates for Text Embeddings (Self-Contained)

These crates act as libraries within your binary. They handle the tokenization and model inference locally.

- **[fastembed](https://crates.io/crates/fastembed)** (Recommended)
  - **Backend:** ONNX Runtime (via the `ort` crate).
  - **Why it’s great:** It is the easiest to use. It manages the downloading and caching of optimized models (like `BGE`, `MiniLM`, and `Nomic`) automatically. It performs the "Mean Pooling" and normalization steps for you, returning a simple `Vec<f32>` for each text snippet.
  - **Trade-off:** ONNX Runtime provides excellent performance but links against a shared library (`libonnxruntime`), which can slightly complicate cross-compilation.

- **[embed-anything](https://crates.io/crates/embed_anything)**
  - **Backend:** Support for both **Candle** (pure Rust) and **ONNX**.
  - **Why it’s great:** Highly modular. It is designed specifically to be a minimalist embedding pipeline. It supports ModernBERT, Jina, and other state-of-the-art models. Because it supports Candle, you can achieve a "pure Rust" build that is easier to static-link.

- **[model2vec-rs](https://crates.io/crates/model2vec-rs)**
  - **Backend:** Static word embeddings (no transformer required).
  - **Why it’s great:** If you need **extreme speed** and a tiny binary footprint. It uses static embeddings that are ~10x-100x faster than transformers. It is ideal for CLI tools or edge devices where high-quality semantic nuance is less critical than throughput.

---

### 2. Best Crates for Clustering

Once you have your embeddings (typically a `Vec<f32>` or an `ndarray`), you can pass them to these mathematical crates.

- **[linfa-clustering](https://crates.io/crates/linfa-clustering)**
  - **Algorithms:** K-Means, DBSCAN.
  - **Why it’s great:** Part of the `linfa` project (the "Scikit-learn of Rust"). It is stable, well-documented, and integrates perfectly with `ndarray`, which most embedding crates use internally.
  - **Use Case:** General purpose K-means when you know the number of clusters.

- **[sheaf](https://crates.io/crates/sheaf)**
  - **Algorithms:** K-Means, DBSCAN, Hierarchical, Leiden/Louvain.
  - **Why it’s great:** A modern, feature-rich clustering crate. It is particularly good if you need **Hierarchical** clustering or community detection (Leiden), which are often more useful for text because you don't always know the number of clusters (`K`) in advance.

- **[avx-clustering](https://crates.io/crates/avx-clustering)**
  - **Algorithms:** K-Means, DBSCAN, HDBSCAN, Spectral, Agglomerative.
  - **Why it’s great:** High-performance implementations optimized for SIMD (AVX). It includes **HDBSCAN**, which is arguably the "gold standard" for text clustering because it handles varying densities and identifies "noise" (outliers) that shouldn't belong to any cluster.

---

### Comparison Summary

| Task           | Top Choice        | Alternative                | Pure Rust?                     |
| :------------- | :---------------- | :------------------------- | :----------------------------- |
| **Embeddings** | `fastembed`       | `embed-anything`           | `embed-anything` (with Candle) |
| **Clustering** | `linfa` (K-Means) | `avx-clustering` (HDBSCAN) | Yes                            |

### Implementation Tip

Most embedding crates return data as `Vec<f32>` or `Vec<Vec<f32>>`. To use them with clustering crates like `linfa` or `avx-clustering`, you will likely need the **[ndarray](https://crates.io/crates/ndarray)** crate to convert your vectors into a matrix:

```rust
use ndarray::Array2;
// Suppose 'embeddings' is Vec<Vec<f32>> from fastembed
let rows = embeddings.len();
let cols = embeddings[0].len();
let data: Vec<f32> = embeddings.into_iter().flatten().collect();
let matrix = Array2::from_shape_vec((rows, cols), data).unwrap();

// Now pass 'matrix' to Linfa or Sheaf
```

**Note on Models:** While the crates are "self-contained" code-wise, the model files themselves (the `.onnx` or `.safetensors` files) are usually 20MB–100MB. You can either have the crate download them on the first run (default) or bundle them into your binary using `include_bytes!` if you need a truly offline single-file executable.
