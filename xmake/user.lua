target("user_bin")
    set_kind("phony")
    on_build(function (target)
        import("modules.path")
        os.cd(string.format("%s/user", os.projectdir()))
        os.exec("python3 build.py")
        os.exec("cargo build --%s", get_config("build_mode"))
        local elfs_dir = path.get_elf_dir("user")
        for _, filepath in ipairs(os.files(elfs_dir.."/*")) do
            if string.find(filepath, ".", 1, true)==nil then
                os.exec("rust-objcopy %s --strip-all -O binary %s.bin", filepath, filepath)
            end
        end
    end)