# Semanticat

An experiment to sort the lines of a file based on word embeddings.

After some research, the technique needed to make this program work is **semantic clustering**, rather than the more common vector search. [CSEP (Cosine Similarity Embeddings Print)](https://lib.rs/crates/csep) already handles printing all lines similar to a specific query.

Ideally we want to read in text from STDIN, segment it (probably just by line for now), compute the embeddings, run a clustering algorithm, and return the lines as clusters.
