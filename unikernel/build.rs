use std::{env, fs, path::PathBuf};

fn main() {
    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(ld, LINKER).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}

static LINKER: &str = r#"
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x80200000;


SECTIONS {
    . = BASE_ADDR;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
"#;
