use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

struct BlackHoleStats {
    total_bytes: u64,
    total_files: u64,
    total_dirs: u64,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: blackhole <target_directory>");
        std::process::exit(1);
    }

    let target_path = PathBuf::from(&args[1]);

    if !target_path.exists() {
        eprintln!("Error: Path '{:?}' does not exist!", target_path);
        std::process::exit(1);
    }

    println!("Initializing event horizon for: {:?}", target_path);

    let mut entries = Vec::new();
    let mut stats = BlackHoleStats {
        total_bytes: 0,
        total_files: 0,
        total_dirs: 0,
    };

    for entry in WalkDir::new(&target_path).contents_first(true) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            stats.total_bytes += metadata.len();
            stats.total_files += 1;
        } else if metadata.is_dir() {
            stats.total_dirs += 1;
        }
        entries.push(entry.path().to_path_buf());
    }

    println!(
        "Target mass detected: {} files, {} directories, {:.2} MB.",
        stats.total_files,
        stats.total_dirs,
        stats.total_bytes as f64 / 1024.0 / 1024.0
    );

    print!("Are you sure you want to pull this target into singularity? (y/N): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    if confirm.trim().to_lowercase() != "y" {
        println!("Abort. Event horizon closed.");
        return Ok(());
    }

    println!("\nGravitational collapse initiated...\n");

    for path in entries {
        if path.is_file() {
            print!("[WIPING] {:?} ... ", path);
            io::stdout().flush()?;

            if let Err(e) = wipe_and_remove_file(&path) {
                println!("ERROR: {}", e);
            } else {
                println!("ZEROED & UNLINKED");
            }
        } else if path.is_dir() && path != target_path {
            let _ = fs::remove_dir(&path);
        }
    }

    if target_path.is_dir() {
        fs::remove_dir(&target_path)?;
    }

    let singularity_filename = format!(
        "{}.singularity",
        target_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    let singularity_path = target_path.with_file_name(singularity_filename);

    create_singularity(&singularity_path, &target_path, &stats)?;

    println!("\n✨ Collapse completed!");
    println!(
        "Singularity coordinates: {:?} (Size: ~1 KB)",
        singularity_path
    );

    Ok(())
}

fn wipe_and_remove_file(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let len = metadata.len();

    if len > 0 {
        let mut file = OpenOptions::new().write(true).open(path)?;

        let buffer_size = 65536;
        let zeros = vec![0u8; buffer_size];
        let mut written = 0;

        while written < len {
            let to_write = std::cmp::min(buffer_size as u64, len - written) as usize;
            file.write_all(&zeros[..to_write])?;
            written += to_write as u64;
        }

        file.sync_all()?;
    }

    let _ = File::create(path);
    fs::remove_file(path)?;
    Ok(())
}

fn create_singularity(
    output_path: &Path,
    original_path: &Path,
    stats: &BlackHoleStats,
) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let json_payload = format!(
        "{{\n  \"event\": \"GRAVITATIONAL_COLLAPSE\",\n  \"target\": \"{}\",\n  \"timestamp\": {},\n  \"consumed_mass_bytes\": {},\n  \"consumed_files\": {},\n  \"consumed_dirs\": {},\n  \"status\": \"SINGULARITY_REACHED\"\n}}\n",
        original_path.to_string_lossy().replace('\\', "/"),
        timestamp,
        stats.total_bytes,
        stats.total_files,
        stats.total_dirs
    );

    file.write_all(json_payload.as_bytes())?;
    file.sync_all()?;
    Ok(())
}