function get_kernel_path()
    local kernel_path = string.format("%s/target/riscv64gc-unknown-none-elf/%s/%s", os.projectdir(), get_config("build_mode"), get_config("proj_name"))
    return kernel_path
end