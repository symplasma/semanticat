# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2026-07-14)

<csr-id-a6b4465b06690f0177a50930d0d47a7e550d61d0/>
<csr-id-7f253d158417c4efb672206ad69d7d7e1c2241d5/>
<csr-id-75d9ca989eb5d43ea8367a97dda7f733411f1371/>

### Chore

 - <csr-id-a6b4465b06690f0177a50930d0d47a7e550d61d0/> remove ndarray dependency in favor of hdbscan

### New Features

 - <csr-id-6958add129a4ada54b360fb5b72ba52c5f0426e0/> suspend progress bar during model loading to avoid output clash
 - <csr-id-9d495070e0ec63dbc6e55cbf52f3fe910829f558/> show progress status and animate spinner during model loading and clustering
 - <csr-id-b2b57981ca67b97f35ac7f7732e3fde212805871/> show progress status for model loading and clustering steps
 - <csr-id-9a20b6e59e8dbe83569cc8cd9b41c4f076176226/> add real progress tracking with --no-progress flag
 - <csr-id-2c55bc74e2e58bcd5d44e24d9b4e248ab7924a6b/> add progress spinner for clustering operations exceeding 2 seconds
 - <csr-id-a4388a9c5cd4a3252f57cc86043f8c16e4dc2870/> add all fastembed text embedding models to --list-models
 - <csr-id-d14be1624e613d0942c9e7e16089a1133928e6f8/> add `--list-models` CLI option to display available embedding models
 - <csr-id-93202b1d766014ef54f7e0eb292168e5698e83cb/> add CLI option to specify embedding model
 - <csr-id-810809ead73ed098fc4b39c19b9c15bc0bd85b5a/> upgrade embedding model to BGEBaseENV15 for improved accuracy
 - <csr-id-7fb79ecc25c5faea09024c4834bb718a16333f24/> switch embedding backend from model2vec to fastembed
 - <csr-id-f72e3ee5723f89622f9f9f8bc0d5220107138b28/> add clipboard fallback when stdin is empty
 - <csr-id-9839db95616ae0e5844805315db38be93bc80972/> add comprehensive tracing and logging throughout application
 - <csr-id-d56da2c65e4313879ed3f47ba3470c0963f96478/> add tracing instrumentation with -v verbosity flag
 - <csr-id-be25d29de7edf4042664ab0fec225941ad508236/> scaffold semantic clustering pipeline with input, embedding, clustering, grouping, and output modules
 - <csr-id-c928d7335b692a693f97a00b48deb76e824e9cce/> strip markdown link URLs before embedding

### Bug Fixes

 - <csr-id-7aa20dcca78bce2e52da245e101ba21e22ef9af4/> disable fastembed's download progress bar to prevent flicker
 - <csr-id-4567d4590e6f9c9cacbdfdbcda1c8f505ac2e8aa/> use get_help() instead of help() for PossibleValue
 - <csr-id-ae6006e5aea1f68ff131147131044ed4783d0d33/> prevent hdbscan panic by clamping parameters to embedding count
 - <csr-id-68c485ceaae5dfad6dc67c51ed4bbf1ac3df2472/> resolve ndarray version conflict with avx-clustering
 - <csr-id-088d93913b91c5058cd7e7fa21265f67c6c4db2c/> resolve type mismatches in clustering and embedding APIs
 - <csr-id-cd40ffe9dd37971bb11b1fa083dd33032a88b0e9/> correct HDBSCAN API usage and error handling in clustering and embedding
 - <csr-id-641443e67586735c37fcc0e6c9dc0518a3a0a14f/> use correct HDBSCAN struct path in clustering module

### Refactor

 - <csr-id-7f253d158417c4efb672206ad69d7d7e1c2241d5/> replace avx_clustering with hdbscan crate

### Style

 - <csr-id-75d9ca989eb5d43ea8367a97dda7f733411f1371/> reorder imports and add mut to model binding

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add status section and LICENSE file ([`1158536`](https://github.com/symplasma/semanticat/commit/115853636e956f5404a785f777e828f00da74bc0))
    - Add more features ([`3c4b5dc`](https://github.com/symplasma/semanticat/commit/3c4b5dc9b581e72e25aaee05303048d7090b86cd))
    - Add features section ([`a8aec1e`](https://github.com/symplasma/semanticat/commit/a8aec1ed0efd696245f4362bfd519f678d4f6e47))
    - Add usage section to readme ([`c1ae7f8`](https://github.com/symplasma/semanticat/commit/c1ae7f8e8d94d10aa51f346fdae742dec611a3c6))
    - Update Cargo.lock ([`3df3438`](https://github.com/symplasma/semanticat/commit/3df3438151effcb114a44107588bd49b24fface8))
    - Strip markdown link URLs before embedding ([`c928d73`](https://github.com/symplasma/semanticat/commit/c928d7335b692a693f97a00b48deb76e824e9cce))
    - Revert "fix: disable fastembed's download progress bar to prevent flicker" ([`ed1296a`](https://github.com/symplasma/semanticat/commit/ed1296a6370f64077c7ed1127d5ee178782a4986))
</details>

