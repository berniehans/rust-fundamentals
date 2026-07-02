#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Initializing rust-fundamentals Cargo Workspace ==="

# 1. Create the chapters directory if it doesn't exist
mkdir -p chapters

# 2. Define the list of chapters matching the Workspace design
chapters=(
    "ch01_getting_started"
    "ch02_guessing_game"
    "ch03_common_concepts"
    "ch04_understanding_ownership"
    "ch05_using_structs"
    "ch06_enums_patterns"
    "ch07_managing_projects"
    "ch08_common_collections"
    "ch09_error_handling"
    "ch10_generics_traits_lifetimes"
    "ch11_writing_tests"
    "ch12_minigrep"
    "ch13_functional_features"
    "ch14_cargo_more"
    "ch15_smart_pointers"
    "ch16_fearless_concurrency"
    "ch17_oop_features"
    "ch18_patterns_matching"
    "ch19_advanced_features"
    "ch20_web_server"
)

# 3. Create a binary crate for each chapter
echo "Creating crates under chapters/..."
for chapter in "${chapters[@]}"; do
    if [ ! -d "chapters/$chapter" ]; then
        echo "Creating ch: chapters/$chapter"
        cargo new --bin --vcs none "chapters/$chapter"
    else
        echo "Chapter chapters/$chapter already exists, skipping."
    fi
done

# 4. Clean up root src directory if it exists
if [ -d "src" ]; then
    echo "Cleaning up root src/ directory to leave workspace clean..."
    rm -rf src
fi

echo "=== Workspace Setup Completed Successfully! ==="
echo "You can run any chapter using: cargo run -p <chapter_name>"
echo "Example: cargo run -p ch02_guessing_game"
