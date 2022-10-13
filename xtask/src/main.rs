use std::process::Command;
use clap::Parser;

static TARGET : &str = "riscv64gc-unknown-none-elf";


#[derive(Parser)]
#[command(name = "xtask")]
#[command(bin_name = "xtask")]
enum Xtask {
    Build(BuildArgs),
    Qemu(QemuArgs)
}


#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct BuildArgs {
    #[arg(short, long, default_value_t = String::from("helloworld"))]
    app: String,
}


#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct QemuArgs {
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

fn main() {
    match Xtask::parse() {
        Xtask::Build(_build_args) => {
            build_runo();
        },
        Xtask::Qemu(qemu_args) => {
            let kernel_path =
                "target/riscv64gc-unknown-none-elf/release/unikernel";
            let kernel_bin_path =
                "target/riscv64gc-unknown-none-elf/release/unikernel.bin";
            strip_bin(kernel_path, kernel_bin_path);
            qemu_run(kernel_path, qemu_args.debug);
        }
    };
}


fn build_runo() {
    Command::new(env!("CARGO"))
        .args(["build", "--package", "unikernel", "--release", "--target", TARGET])
        .status()
        .expect("Failed to build unikernel");
}


fn strip_bin(in_file: &str, out_file: &str) {
    Command::new("rust-objcopy")
        .args(["--strip-all", in_file, "-O", "binary", out_file])
        .status()
        .expect("Failed to strip binary file");
}


fn qemu_run(kernel_path: &str, debug: bool) {
    let mut args = vec![
        "-machine", "virt", "-nographic", "-kernel", kernel_path
    ];
    if debug {
        args.push("-s");
        args.push("-S");
    }

    Command::new("qemu-system-riscv64")
        .args(args)
        .status()
        .expect("Failed to build unikernel");
    
}
