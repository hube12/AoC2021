cmake_minimum_required(VERSION 3.17)
project(AOC-zig)

# Custom target will always cause its dependencies to be evaluated and is
# run by default
add_custom_target(ZIG_RUN ALL
        DEPENDS zig_build_run
)

# custom_output will always be rebuilt because it depends on always_rebuild
add_custom_command(
        OUTPUT zig_build_run
        COMMAND zig build run
        DEPENDS always_rebuild
)

# Dummy output which is never actually produced. Anything that depends on
# this will always be rebuilt.
add_custom_command(
        OUTPUT always_rebuild
        COMMAND cmake -E echo
)