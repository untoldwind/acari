function __fish_is_arg_n --argument-names n
    test $n -eq (count (string match -v -- '-*' (commandline -poc)))
end

# options
complete -c acari -s o -l output -a "flat json pretty" -d "set output format"
complete -c acari -s h -l help -d "show help"
complete -c acari -l no-cache -d "disable caching"

# subcommands
complete -f -c acari -n "__fish_use_subcommand" -a init -d "initialize connection"
complete -f -c acari -n "__fish_use_subcommand" -a check -d "check connection"
complete -f -c acari -n "__fish_use_subcommand" -a clear-cache -d "Clear local cache"
complete -f -c acari -n "__fish_use_subcommand" -a customers -d "list customers"
complete -f -c acari -n "__fish_use_subcommand" -a projects -d "list projects"
complete -f -c acari -n "__fish_use_subcommand" -a services -d "list services"

# check
complete -f -c acari -n "__fish_seen_subcommand_from check"

# check
complete -f -c acari -n "__fish_seen_subcommand_from clear-cache"

# customers
complete -f -c acari -n "__fish_seen_subcommand_from customers"

# projects
complete -f -c acari -n "__fish_seen_subcommand_from projects"
complete -f -c acari -n "__fish_seen_subcommand_from projects; and __fish_is_arg_n 2" -a "(acari -oflat customers)" 

# services
complete -f -c acari -n "__fish_seen_subcommand_from services"
