task :perf do
    Dir.chdir("#{ENV['HOME']}/aa") do
        {
            "count all files" => {
                mo: "mo -l -u -U -a",
                rg: "rg --files -uu -a",
                ag: "ag -l -uu -a",
            },
            "search for `test` in .cpp files in subfolder `core` where path contains /auro/" => {
                mo: "mo -C core -e cpp -f /auro/ -w -p test -l",
                rg: "rg -t cpp --files core | rg /auro/ | rg '.cpp$' | tr '\\n' '\\0' | xargs -0 rg -i -w test -l",
                ag: "ag --cpp -l . core | ag /auro/ | ag '.cpp$' | tr '\\n' '\\0' | xargs -0 ag -w test -l",
            },
        }.each do |scn, cmds|
                puts("\nScenario: #{scn}")
                cmds.each do |sym,cmd|
                    term = "sort > #{sym}.txt"
                    term = "wc"
                    cmd = "#{cmd} | #{term}"
                    puts("  Running `#{cmd}`")
                    cmd = "time -f '    Elapsed time: %E' #{cmd}"
                    output = `#{cmd}`
                    puts("    Output: #{output}\n")
                end
            end
    end
end
