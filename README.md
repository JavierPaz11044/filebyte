# FileByte

**Version:** 0.0.1

Rust library and CLI for reading files in chunks with metadata extraction and reassembly. Useful for streaming, transfer, or processing large files in fixed-size blocks.

---

## Features

- **Metadata extraction** from file paths (name, size, MIME type, directory flag).
- **Chunking** of files into fixed-size buffers with metadata attached to each chunk.
- **Reassembly** of chunks back into a single file (append order).
- Pluggable metadata via the `MetadataExtractor` trait.
- Optional error and chunk callbacks for integration in pipelines.

---

## First services (v0.0.1)

The binary in `src/main.rs` demonstrates chunking a file and reassembling it; the snippets below reflect that same API. For use as a library from other crates, expose `metadata` and `stream_buffer` in `lib.rs` (e.g. `pub mod metadata; pub mod stream_buffer;`).

### 1. Metadata: `ExtractMetadata`

Extracts file metadata from a path.

- **Input:** `MetadataInput::PATH(PathBuf)` (buffer input is planned, not yet implemented).
- **Output:** `Metadata` with:
  - `name` – file name
  - `size` – size in bytes
  - `is_dir` – whether it is a directory
  - `mime_type` – detected via [mimetype-detector](https://crates.io/crates/mimetype-detector)

**Usage (concept):**

```rust
use file_byte::metadata::file_metadata::ExtractMetadata;
use file_byte::metadata::domain::enums::MetadataInput;
use std::path::PathBuf;

let extractor = ExtractMetadata::new();
let path = PathBuf::from("/path/to/file.png");
let metadata = extractor.get_metadata(MetadataInput::PATH(path))?;
// metadata.get_size(), metadata.get_mime_type(), etc.
```

---

### 2. Stream buffer: `Chunker<T>`

Splits a file into fixed-size chunks. Each chunk carries the main file’s metadata and the chunk payload.

- **Chunk size:** 32 bytes (configurable via `stream_buffer::domain::constants::CHUNK_SIZE`).
- **Callbacks:** `on_chunk(Chunk)` for each chunk, optional `on_error(String)` on I/O errors.
- **Generic:** Accepts any type that implements `MetadataExtractor` (e.g. `ExtractMetadata`).

**Usage (concept):**

```rust
use file_byte::metadata::file_metadata::ExtractMetadata;
use file_byte::stream_buffer::services::chunker::Chunker;

let chunker = Chunker::new(ExtractMetadata::new());
chunker.chunk_file(
    "/path/to/file.png".to_string(),
    |chunk| {
        // Process chunk: chunk.get_buffer(), chunk.get_size(),
        // chunk.get_main_metadata().get_mime_type(), etc.
    },
    Some(|err| eprintln!("Error: {}", err)),
);
```

---

### 3. Stream buffer: `Reassembler`

Writes chunks to a single output file in append order, so that chunking then reassembly reproduces the original file.

- **Input:** Output path and a reference to each `Chunk` (writes `chunk.get_buffer()`).
- **Output:** File created/opened in append mode; each `write_chunk` appends that chunk’s bytes.

**Usage (concept):**

```rust
use file_byte::stream_buffer::services::reassembler::Reassembler;

let reassembler = Reassembler::new("/path/to/reassembled.png".to_string());
// Inside your on_chunk callback:
reassembler.write_chunk(&chunk)?;
```

---

## Project layout

```
src/
├── lib.rs                 # Library root (metadata, stream_buffer, utils)
├── main.rs                # Example: chunk file → reassemble to new path
├── metadata/
│   ├── domain/            # Metadata, MetadataInput, etc.
│   └── services/          # MetadataExtractor trait, ExtractMetadata
├── stream_buffer/
│   ├── domain/            # Chunk, ChunkFrom, CHUNK_SIZE
│   └── services/          # Chunker, Reassembler
└── utils/
    └── logger.rs          # Logger (Debug, Info, Warn, Error)
```

---

## Requirements

- Rust (edition and toolchain as in `Cargo.toml`).
- Dependencies: `chrono`, `mimetype-detector` (see `Cargo.toml`).

---

## Build and run

```bash
cargo build
cargo run
```

The default `main.rs` chunks a sample image under `tests/resources/` and reassembles it to `tests/resources/reassembled_image.png`.
