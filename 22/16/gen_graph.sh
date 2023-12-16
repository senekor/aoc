#!/bin/bash

cargo run --bin print_graph | dot -Tpng > graph.png
