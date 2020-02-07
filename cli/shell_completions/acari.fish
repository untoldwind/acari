function __fish_is_arg_n --argument-names n
    test $n -eq (count (string match -v -- '-*' (commandline -poc)))
end

function __fish_arg_n --argument-names n
    set -l args (string match -v -- '-*' (commandline -poc))
    echo -n $args[(math "$n + 1")]
end

# options
complete -c acari -s o -l output -a "flat json pretty" -d "set output format"
complete -c acari -s h -l help -d "show help"
complete -c acari -l no-cache -d "disable caching"

# subcommands
complete -f -c acari -n "__fish_use_subcommand" -a add -d "add time entry"
complete -f -c acari -n "__fish_use_subcommand" -a init -d "initialize connection"
complete -f -c acari -n "__fish_use_subcommand" -a check -d "check connection"
complete -f -c acari -n "__fish_use_subcommand" -a clear-cache -d "Clear local cache"
complete -f -c acari -n "__fish_use_subcommand" -a customers -d "list customers"
complete -f -c acari -n "__fish_use_subcommand" -a entries -d "list time entries"
complete -f -c acari -n "__fish_use_subcommand" -a projects -d "list projects"
complete -f -c acari -n "__fish_use_subcommand" -a services -d "list services"
complete -f -c acari -n "__fish_use_subcommand" -a set -d "set time entry"
complete -f -c acari -n "__fish_use_subcommand" -a start -d "start time tracking"
complete -f -c acari -n "__fish_use_subcommand" -a stop -d "stop time tracking"
complete -f -c acari -n "__fish_use_subcommand" -a tracking -d "show current time tracking"

# add
complete -f -c acari -n "__fish_seen_subcommand_from add"
complete -f -c acari -n "__fish_seen_subcommand_from add; and __fish_is_arg_n 2" -a "(acari -oflat customers)" 
complete -f -c acari -n "__fish_seen_subcommand_from add; and __fish_is_arg_n 3" -a "(acari -oflat projects (__fish_arg_n 2))" 
complete -f -c acari -n "__fish_seen_subcommand_from add; and __fish_is_arg_n 4" -a "(acari -oflat services)" 

# check
complete -f -c acari -n "__fish_seen_subcommand_from check"

# check
complete -f -c acari -n "__fish_seen_subcommand_from clear-cache"

# customers
complete -f -c acari -n "__fish_seen_subcommand_from customers"

# entries
complete -f -c acari -n "__fish_seen_subcommand_from entries"
complete -f -c acari -n "__fish_seen_subcommand_from entries; and __fish_is_arg_n 2" -a "today yesterday this-week last-week this-month last-month (date +%Y-%m-%d)" 

# projects
complete -f -c acari -n "__fish_seen_subcommand_from projects"
complete -f -c acari -n "__fish_seen_subcommand_from projects; and __fish_is_arg_n 2" -a "(acari -oflat customers)" 

# services
complete -f -c acari -n "__fish_seen_subcommand_from services"

# set
complete -f -c acari -n "__fish_seen_subcommand_from set"
complete -f -c acari -n "__fish_seen_subcommand_from set; and __fish_is_arg_n 2" -a "(acari -oflat customers)" 
complete -f -c acari -n "__fish_seen_subcommand_from set; and __fish_is_arg_n 3" -a "(acari -oflat projects (__fish_arg_n 2))" 
complete -f -c acari -n "__fish_seen_subcommand_from set; and __fish_is_arg_n 4" -a "(acari -oflat services)" 

# start
complete -f -c acari -n "__fish_seen_subcommand_from start"
complete -f -c acari -n "__fish_seen_subcommand_from start; and __fish_is_arg_n 2" -a "(acari -oflat customers)" 
complete -f -c acari -n "__fish_seen_subcommand_from start; and __fish_is_arg_n 3" -a "(acari -oflat projects (__fish_arg_n 2))" 
complete -f -c acari -n "__fish_seen_subcommand_from start; and __fish_is_arg_n 4" -a "(acari -oflat services)" 

# stop
complete -f -c acari -n "__fish_seen_subcommand_from stop"

# tracking
complete -f -c acari -n "__fish_seen_subcommand_from tracking"
