function get_elf_dir(target)
    return string.format("%s/%s/target/riscv64gc-unknown-none-elf/%s", os.projectdir(), target, get_config("build_mode"))
end

function get_kernel_path()
    return string.format("%s/%s", get_elf_dir("os"), get_config("proj_name"))
end

