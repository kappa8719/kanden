use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    os::unix::process::CommandExt,
    path::Path,
    process::{exit, Command},
};

use clap::Parser;
use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks, Repository};
use log::{error, info};

#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    #[clap(long, default_value_t = false)]
    copy_only: bool,
}

const EXTRACTOR_REPOSITORY_URL: &str = "https://github.com/kappa8719/kanden-extractor.git";
const COPIES: &[(&str, &str)] = &[
    // kanden-entity
    ("entities.json", "crates/kanden_entity/extracted"),
    ("misc.json", "crates/kanden_entity/extracted"),
    // kanden-generated
    ("misc.json", "crates/kanden_generated/extracted"),
    ("attributes.json", "crates/kanden_generated/extracted"),
    ("blocks.json", "crates/kanden_generated/extracted"),
    ("effects.json", "crates/kanden_generated/extracted"),
    ("items.json", "crates/kanden_generated/extracted"),
    ("packets.json", "crates/kanden_generated/extracted"),
    ("sounds.json", "crates/kanden_generated/extracted"),
    ("data_components.json", "crates/kanden_generated/extracted"),
    // kanden-lang
    ("translation_keys.json", "crates/kanden_lang/extracted"),
    // kanden-registry
    ("registry_codec.json", "crates/kanden_registry/extracted"),
    ("tags.json", "crates/kanden_registry/extracted"),
    // packet-inspector
    ("packets.json", "tools/packet_inspector/extracted"),
];

fn update_extractor(dst: &Path, branch: &str) -> Result<git2::Repository, git2::Error> {
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(RemoteCallbacks::new());
    fetch_options.depth(1);

    if dst.exists() {
        if dst.join(".git").exists() {
            info!("updating extractor");

            let repo = Repository::open(dst)?;
            {
                let mut remote = if let Ok(remote) = repo.find_remote("origin") {
                    remote
                } else {
                    let remotes = repo.remotes()?;
                    let Some(name) = remotes.get(0) else {
                        error!("the repository does not have any remote");
                        exit(-1);
                    };

                    repo.find_remote(name)?
                };

                remote.fetch(&[branch], Some(&mut fetch_options), None)?;
            }
            return Ok(repo);
        } else {
            error!("the extractor is corrupted. delete '.extractor' to fetch the extractor again");
            exit(-1);
        }
    }

    info!("cloning extractor");
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    return builder.clone(EXTRACTOR_REPOSITORY_URL, dst);
}

fn copy_files(target: &Path) {
    for (src, dst) in COPIES {
        let src_path = target.join("run/_data").join(src);
        let dst_path = Path::new(dst).join(src);
        std::fs::copy(src_path.as_path(), dst_path.as_path()).unwrap_or_else(|e| {
            panic!("failed to copy from '{src_path:?}' to '{dst_path:?}': {e}")
        });
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let args = Args::parse();

    let target = Path::new("extractor");

    if args.copy_only {
        info!("copy only: copying files");
        copy_files(target);
        info!("complete");
        return;
    }

    update_extractor(target, "main").unwrap_or_else(|e| {
        panic!("failed to clone extractor from {EXTRACTOR_REPOSITORY_URL}: {e}")
    });

    let eula = target.join("run/eula.txt");
    if !eula.exists() {
        println!("type 'agree' to agree on EULA and proceed");
        print!("by typing 'agree' you are indicating your agreement to Minecraft EULA (https://aka.ms/MinecraftEULA): ");
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if buf == "agree" || buf == "agree\n" {
            std::fs::create_dir_all(target.join("run"))
                .expect("failed to create directory 'extractor/run'");
            let mut eula = File::create(eula).expect("failed to open eula.txt");
            eula.write("eula=true".as_bytes())
                .expect("failed to write to eula.txt");
        } else {
            error!("aborting: user disagreed on eula");
            return;
        }
    }

    info!("running extractor");
    let status = Command::new("sh")
        .arg("./gradlew")
        .arg("runServer")
        .current_dir("./extractor")
        .status()
        .expect("failed to execute runServer task");

    if status.success() {
        info!("extraction complete");
        info!("copying files");
        copy_files(target);
        info!("copied files");
    } else {
        error!("extraction failed with status {status}");
    }
}
