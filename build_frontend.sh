#!/bin/bash
cd frontend
bun install
bun run build
bun x tailwindcss -i ./src/input.css -o ./dist/output.css
cd ..
