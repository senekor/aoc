#!/usr/bin/env nu

if (git rev-parse --abbrev-ref HEAD) == main {
    source check_difficulty.nu
}
