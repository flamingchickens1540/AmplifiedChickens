#!/bin/bash
cd frontend
bun install
bun run build
bunx tailwindcss -i ./src/input.css -o ./dist/output.css
cd ..
cargo run
