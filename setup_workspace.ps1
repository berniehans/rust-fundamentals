# setup_workspace.ps1
# Setup script for Windows PowerShell/Core

Write-Host "=== Initializing rust-fundamentals Cargo Workspace ===" -ForegroundColor Cyan

# 1. Create the chapters directory
if (!(Test-Path -Path "chapters")) {
    New-Item -ItemType Directory -Path "chapters" | Out-Null
}

# 2. Define the list of chapters
$chapters = @(
    "ch01_getting_started",
    "ch02_guessing_game",
    "ch03_common_concepts",
    "ch04_understanding_ownership",
    "ch05_using_structs",
    "ch06_enums_patterns",
    "ch07_managing_projects",
    "ch08_common_collections",
    "ch09_error_handling",
    "ch10_generics_traits_lifetimes",
    "ch11_writing_tests",
    "ch12_minigrep",
    "ch13_functional_features",
    "ch14_cargo_more",
    "ch15_smart_pointers",
    "ch16_fearless_concurrency",
    "ch17_oop_features",
    "ch18_patterns_matching",
    "ch19_advanced_features",
    "ch20_web_server"
)

# 3. Create a binary crate for each chapter
Write-Host "Creating crates under chapters/..." -ForegroundColor Yellow
foreach ($chapter in $chapters) {
    $path = "chapters/$chapter"
    if (!(Test-Path -Path $path)) {
        Write-Host "Creating ch: $path"
        cargo new --bin --vcs none $path
    } else {
        Write-Host "Chapter $path already exists, skipping." -ForegroundColor DarkGray
    }
}

# 4. Clean up root src directory if it exists
if (Test-Path -Path "src") {
    Write-Host "Cleaning up root src/ directory to leave workspace clean..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "src"
}

Write-Host "=== Workspace Setup Completed Successfully! ===" -ForegroundColor Green
Write-Host "You can run any chapter using: cargo run -p <chapter_name>" -ForegroundColor Green
Write-Host "Example: cargo run -p ch02_guessing_game" -ForegroundColor Green
