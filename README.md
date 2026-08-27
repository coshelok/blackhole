# BlackHole CLI

> **Turns any directory structure into a microscopic 1 KB `.singularity` file.**

`blackhole` is a cross-platform command-line tool designed for folder annihilation. It traverses directory trees deep-first, overwrites files with zero-byte buffers, flushes physical disk write caches, and collapses the target path into a single JSON metadata artifact (`.singularity`).


# Disclaimer
**This tool performs real, unrecoverable file wiping. Files zeroed out and deleted by `blackhole` cannot be restored using standard data recovery utilities. Use at your own risk.**


# Features
- **Deep-First Collapse:** Deletes leaves and subdirectories from the inside out.
- **Hardware Cache Flush:** Calls `fsync` before unlinking to bypass OS memory buffering.
- **Singularity Artifact:** Leaves behind a tiny `.singularity` JSON log containing destroyed mass and file metrics.


# Build from Source
git clone https://github.com/coshelok/blackhole.git
cd blackhole
cargo build --release


> **The compiled executable will be at target/release/blackhole.**


# Run
./target/release/blackhole /path/to/target_directory


# Output Example
{
  "event": "GRAVITATIONAL_COLLAPSE",
  "target": "/tmp/my_folder",
  "timestamp": 1787873038,
  "consumed_mass_bytes": 1489481728,
  "consumed_files": 4210,
  "consumed_dirs": 312,
  "status": "SINGULARITY_REACHED"
}

# License
Distributed under the MIT License.