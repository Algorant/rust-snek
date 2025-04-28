# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Run game: `cargo run`
- Lint code: `cargo clippy`
- Format code: `cargo fmt`
- Check compilation: `cargo check`
- Run tests: `cargo test` (none currently implemented)
- Run single test: `cargo test test_name` (replace test_name with actual test)

## Code Style Guidelines
- **Imports**: External crates first, then internal modules
- **Formatting**: 4-space indentation, follow `rustfmt` conventions
- **Types**: Use explicit type annotations for clarity
- **Constants**: Use `SCREAMING_SNAKE_CASE` with type annotations
- **Structs/Enums**: Use `PascalCase` for definitions
- **Methods/Variables**: Use `snake_case` for names
- **Error Handling**: Currently using `unwrap()`, improve with proper `Result` handling
- **Documentation**: Add doc comments (`///`) for public functions and types
- **Organization**: Keep modular structure with separate concerns (game, draw, snake)
- **Naming**: Descriptive names reflecting purpose and behavior