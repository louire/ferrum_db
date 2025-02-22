# FerrumDB 🦀

A modern terminal-based database client written in Rust. FerrumDB provides an intuitive TUI (Text User Interface) for interacting with your databases.

## Features

### Currently Implemented
- 🖥️ Modern terminal user interface using [ratatui](https://github.com/ratatui-org/ratatui)
- ⌨️ Vim-like modal editing (Normal and Insert modes)
- 📝 Interactive SQL query editor with cursor support
- 🎨 Multi-panel layout:
  - Query input panel
  - Results display
  - Database navigation sidebar
  - Status bar showing current connection

### Coming Soon
- [ ] Query execution and results display
- [ ] Table view for query results
- [ ] Database schema navigation
- [ ] Query history
- [ ] Syntax highlighting
- [ ] Auto-completion
- [ ] Multiple database backend support
- [ ] Configuration file support
- [ ] Keyboard shortcuts customization

## Installation

Currently, the project is under development. To try it out:

```bash
# Clone the repository
git clone https://github.com/louire/ferrum_db.git
cd ferrum_db

# Build the project
cargo build

# Run in debug mode
cargo run
```

## Usage

### Keyboard Shortcuts

#### Normal Mode
- `i` - Enter Insert mode
- `q` - Quit application
- `Ctrl+C` - Quit application

#### Insert Mode
- `Esc` - Return to Normal mode
- `Enter` - Execute query
- `←/→` - Move cursor
- `Backspace` - Delete character

## Project Structure

```
ferrum_db/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app/              # Application logic
│   │   ├── mod.rs        # App struct and event handling
│   │   ├── state.rs      # Application state management
│   │   └── input.rs      # Input handling and modes
│   ├── database/         # Database interaction
│   │   ├── mod.rs        # Database connection and queries
│   │   └── error.rs      # Custom error types
│   └── ui/              # User interface
│       ├── mod.rs       # UI layout and rendering
│       ├── input.rs     # Query input panel
│       ├── results.rs   # Query results panel
│       ├── sidebar.rs   # Navigation sidebar
│       └── status.rs    # Status bar
```

## Development Status

The project is in active development. Current focus is on:
1. Implementing database query execution
2. Displaying query results in a table format
3. Adding database schema navigation
4. Implementing query history

## Contributing

The project is in early development stages. Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - For the terminal user interface framework
- [sqlx](https://github.com/launchbadge/sqlx) - For the SQL toolkit
- [crossterm](https://github.com/crossterm-rs/crossterm) - For terminal manipulation

---
*Built with 🦀 Rust and ❤️*