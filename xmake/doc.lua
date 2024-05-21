rule("markdown2html")
    set_extensions(".md", ".markdown")
    on_build_file(function (target, sourcefile, opt)
        import("core.project.depend")
        import("utils.progress") -- it only for v2.5.9, we need use print to show progress below v2.5.8

        -- make sure build directory exists
        local targetdir = os.projectdir().."/doc"
        os.mkdir(targetdir)

        -- replace .md with .html
        local targetfile = path.join(targetdir, path.basename(sourcefile) .. ".html")

        -- only rebuild the file if its changed since last run
        depend.on_changed(function ()
            -- call pandoc to make a standalone html file from a markdown file
            os.vrunv('pandoc', {"-s", "-f", "markdown", "-t", "html", "-o", targetfile, sourcefile})
            progress.show(opt.progress, "${color.build.object}markdown %s", sourcefile)
        end, {files = sourcefile})
    end)

target("doc")
    set_kind("object")
    add_rules("markdown2html")
    add_files(os.projectdir().."/*.md")

