option("build_mode")
    set_showmenu(true)
    set_default("release")
    set_values("debug", "release")

option("proj_name")
    set_showmenu(true)
    set_default("myrcore")

option("bootloader")
    set_showmenu(true)
    set_default("./bootloader/rustsbi-qemu.bin")

option("kernel_entry_pa")
    set_showmenu(true)
    set_default("0x80200000")



target("kernel_bin")
    set_kind("phony")
    on_build(function (target)
        import("modules.path")
        os.exec("cargo build --$(build_mode)")
        -- local kernel_path = string.format("target/riscv64gc-unknown-none-elf/%s/%s", get_config("build_mode"), get_config("proj_name"))
        local kernel_path = path.get_kernel_path()
        os.exec("rust-objcopy %s --strip-all -O binary %s.bin", kernel_path, kernel_path)
    end)

target("debug")
    set_kind("phony")
    add_deps("kernel_bin")
    on_run(function (target)
        import("modules.path")
        local kernel_path = path.get_kernel_path()
        local kernel_bin_path = string.format("%s.bin", kernel_path)
        local qemu_args = string.format("-machine virt \
			 -nographic \
			 -bios %s \
			 -device loader,file=%s,addr=%s",
             get_config("bootloader"),
             kernel_bin_path,
             get_config("kernel_entry_pa")
        )
        print(kernel_path)
        os.exec(string.format("tmux new-session -d qemu-system-riscv64 %s -s -S", qemu_args))
        os.exec(string.format("tmux split-window -h \"riscv64-unknown-elf-gdb -q -ex 'file %s' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'\"", kernel_path))
        os.exec("tmux -2 attach-session -d")
    end)

