# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2026-07-14)

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

 - 38 commits contributed to the release over the course of 151 calendar days.
 - 24 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add metadata ([`2eb6218`](https://github.com/symplasma/semanticat/commit/2eb62182fcaa1cebbe2cac8e13f85445137533a3))
    - Disable fastembed's download progress bar to prevent flicker ([`7aa20dc`](https://github.com/symplasma/semanticat/commit/7aa20dcca78bce2e52da245e101ba21e22ef9af4))
    - Suspend progress bar during model loading to avoid output clash ([`6958add`](https://github.com/symplasma/semanticat/commit/6958add129a4ada54b360fb5b72ba52c5f0426e0))
    - Show progress status and animate spinner during model loading and clustering ([`9d49507`](https://github.com/symplasma/semanticat/commit/9d495070e0ec63dbc6e55cbf52f3fe910829f558))
    - Show progress status for model loading and clustering steps ([`b2b5798`](https://github.com/symplasma/semanticat/commit/b2b57981ca67b97f35ac7f7732e3fde212805871))
    - Correct number of steps ([`3a4a253`](https://github.com/symplasma/semanticat/commit/3a4a2539c49ecc70d12fdc6842d1e90d775ed665))
    - Add real progress tracking with --no-progress flag ([`9a20b6e`](https://github.com/symplasma/semanticat/commit/9a20b6e59e8dbe83569cc8cd9b41c4f076176226))
    - Add progress spinner for clustering operations exceeding 2 seconds ([`2c55bc7`](https://github.com/symplasma/semanticat/commit/2c55bc74e2e58bcd5d44e24d9b4e248ab7924a6b))
    - Make default trace level warn ([`33dc3b4`](https://github.com/symplasma/semanticat/commit/33dc3b4dbaf8adf9a824b94014952d6d2bbe1499))
    - Add all fastembed text embedding models to --list-models ([`a4388a9`](https://github.com/symplasma/semanticat/commit/a4388a9c5cd4a3252f57cc86043f8c16e4dc2870))
    - Use get_help() instead of help() for PossibleValue ([`4567d45`](https://github.com/symplasma/semanticat/commit/4567d4590e6f9c9cacbdfdbcda1c8f505ac2e8aa))
    - Add `--list-models` CLI option to display available embedding models ([`d14be16`](https://github.com/symplasma/semanticat/commit/d14be1624e613d0942c9e7e16089a1133928e6f8))
    - Prevent hdbscan panic by clamping parameters to embedding count ([`ae6006e`](https://github.com/symplasma/semanticat/commit/ae6006e5aea1f68ff131147131044ed4783d0d33))
    - Add CLI option to specify embedding model ([`93202b1`](https://github.com/symplasma/semanticat/commit/93202b1d766014ef54f7e0eb292168e5698e83cb))
    - Upgrade embedding model to BGEBaseENV15 for improved accuracy ([`810809e`](https://github.com/symplasma/semanticat/commit/810809ead73ed098fc4b39c19b9c15bc0bd85b5a))
    - Reorder imports and add mut to model binding ([`75d9ca9`](https://github.com/symplasma/semanticat/commit/75d9ca989eb5d43ea8367a97dda7f733411f1371))
    - Switch embedding backend from model2vec to fastembed ([`7fb79ec`](https://github.com/symplasma/semanticat/commit/7fb79ecc25c5faea09024c4834bb718a16333f24))
    - Switch to fastembed crate ([`17ab1f5`](https://github.com/symplasma/semanticat/commit/17ab1f5b9e9e7b6a400bc6559353a0b178814801))
    - Replace avx_clustering with hdbscan crate ([`7f253d1`](https://github.com/symplasma/semanticat/commit/7f253d158417c4efb672206ad69d7d7e1c2241d5))
    - Remove ndarray dependency in favor of hdbscan ([`a6b4465`](https://github.com/symplasma/semanticat/commit/a6b4465b06690f0177a50930d0d47a7e550d61d0))
    - Switch clustering crate ([`185ef64`](https://github.com/symplasma/semanticat/commit/185ef649051ba3d58f014b3058eb82bf97672399))
    - Update incompatible dependencies ([`419a7e7`](https://github.com/symplasma/semanticat/commit/419a7e7f6d43e81f54fe832fd02989eaa4b92321))
    - Add clipboard fallback when stdin is empty ([`f72e3ee`](https://github.com/symplasma/semanticat/commit/f72e3ee5723f89622f9f9f8bc0d5220107138b28))
    - Add arboard crate ([`3201dbf`](https://github.com/symplasma/semanticat/commit/3201dbf20269bcf5a9149147a8ce16a7397807ee))
    - Add comprehensive tracing and logging throughout application ([`9839db9`](https://github.com/symplasma/semanticat/commit/9839db95616ae0e5844805315db38be93bc80972))
    - Add tracing instrumentation with -v verbosity flag ([`d56da2c`](https://github.com/symplasma/semanticat/commit/d56da2c65e4313879ed3f47ba3470c0963f96478))
    - Add Cargo.lock ([`541f848`](https://github.com/symplasma/semanticat/commit/541f84862cabe36608fcfbe2347bc79c6896eaea))
    - Resolve ndarray version conflict with avx-clustering ([`68c485c`](https://github.com/symplasma/semanticat/commit/68c485ceaae5dfad6dc67c51ed4bbf1ac3df2472))
    - Resolve type mismatches in clustering and embedding APIs ([`088d939`](https://github.com/symplasma/semanticat/commit/088d93913b91c5058cd7e7fa21265f67c6c4db2c))
    - Correct HDBSCAN API usage and error handling in clustering and embedding ([`cd40ffe`](https://github.com/symplasma/semanticat/commit/cd40ffe9dd37971bb11b1fa083dd33032a88b0e9))
    - Use correct HDBSCAN struct path in clustering module ([`641443e`](https://github.com/symplasma/semanticat/commit/641443e67586735c37fcc0e6c9dc0518a3a0a14f))
    - Scaffold semantic clustering pipeline with input, embedding, clustering, grouping, and output modules ([`be25d29`](https://github.com/symplasma/semanticat/commit/be25d29de7edf4042664ab0fec225941ad508236))
    - Add agents file ([`06d01cd`](https://github.com/symplasma/semanticat/commit/06d01cdedc0055ec872adec04f65c41641391119))
    - Add basic feature description ([`6ad217a`](https://github.com/symplasma/semanticat/commit/6ad217a9545e4e73b95801e1b2716480943fc29a))
    - Add more crates ([`a5ee8a3`](https://github.com/symplasma/semanticat/commit/a5ee8a3e7ec17318842409a10b8b2d20cd0a65cb))
    - Add Rust crate guidance ([`e34b534`](https://github.com/symplasma/semanticat/commit/e34b534b3183a2f297d393e6c4149aa6fec0cbc0))
    - Add semantic clustering info ([`9914c89`](https://github.com/symplasma/semanticat/commit/9914c89ec874f72f5b87fa36d3bdd36e9129bf6e))
    - Initial Commit ([`6171929`](https://github.com/symplasma/semanticat/commit/6171929dcd86f160e92ccdd6677a6a6c128dd184))
</details>

