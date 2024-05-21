rule("markdown2html")
    set_extensions(".md", ".markdown")
    on_buildcmd_file(function (target, batchcmds, sourcefile, opt)

        -- make sure build directory exists
        local targetdir = os.projectdir().."/doc"
        batchcmds:mkdir(targetdir)

        -- replace .md with .html
        local targetfile = path.join(targetdir, path.basename(sourcefile) .. ".html")

        -- call pandoc to make a standalone html file from a markdown file
        batchcmds:vrunv('pandoc', {"-s", "-f", "markdown", "-t", "html", "-o", targetfile, sourcefile})
        batchcmds:show_progress(opt.progress, "${color.build.object}markdown %s", sourcefile)

        -- only rebuild the file if its changed since last run
        batchcmds:add_depfiles(sourcefile)
    end)

target("doc")
    set_kind("object")
    add_rules("markdown2html")
    add_files(os.projectdir().."/*.md")

