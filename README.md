# FerrumDB 🦀

<div align="center">
  <img src="assets/logo.png" alt="Rusty Bot Logo" width="200">
</div>

A modern terminal-based database client written in Rust. FerrumDB provides an intuitive TUI (Text User Interface) for interacting with your databases.

## Features

### Currently Implemented
- 🖥️ Modern terminal user interface using [ratatui](https://github.com/ratatui-org/ratatui)
- ⌨️ Vim-like modal editing (Normal and Insert modes)
- 📝 Interactive SQL query editor with cursor support
- 🎨 Multi-panel layout:
  - Query input panel
  - Results display in table format
  - Database navigation sidebar
  - Status bar showing current connection
- ⚙️ Configuration file support (config.toml)
- 🔍 Query execution and results display
- 📊 Table view for query results

### Coming Soon
- [ ] Database schema navigation
- [ ] Query history
- [ ] Syntax highlighting
- [ ] Auto-completion
- [ ] Multiple database backend support
- [ ] Keyboard shortcuts customization
- [ ] Results pagination
- [ ] Column sorting

## Installation

Currently, the project is under development. To try it out:

```bash
# Clone the repository
git clone https://github.com/louire/ferrum_db.git
cd ferrum_db

# Create a config.toml file
cat > config.toml << EOL
[database]
host = "localhost"
port = 5432
username = "your_username"
password = "your_password"
database = "your_database"
EOL

# Build the project
cargo build

# Run in debug mode
cargo run
```

## Usage

### Configuration
Create a `config.toml` file in the project root with your database connection details:

```toml
[database]
host = "localhost"
port = 5432
username = "postgres"
password = "your_password"
database = "your_database"
```

If no configuration file is found, a default one will be created.

### Keyboard Shortcuts

#### Normal Mode
- `i` - Enter Insert mode
- `q` - Quit application
- `Ctrl+C` - Quit application
- `r` - Clear results
- `d` - Toggle database list (coming soon)

#### Insert Mode
- `Esc` - Return to Normal mode
- `Enter` - Execute query and return to Normal mode
- `Ctrl+Enter/Alt+Enter` - Execute query and stay in Insert mode
- `←/→` - Move cursor
- `Backspace` - Delete character before cursor
- `Delete` - Delete character under cursor
- `Home/Ctrl+a` - Move to start of line
- `End/Ctrl+e` - Move to end of line

## Project Structure

```
ferrum_db/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app/              # Application logic
│   │   ├── mod.rs        # App struct and event handling
│   │   ├── state.rs      # Application state management
│   │   ├── input.rs      # Input handling and modes
│   │   └── query.rs      # Query result handling
│   ├── config/           # Configuration handling
│   │   └── mod.rs        # Configuration loading/saving
│   ├── database/         # Database interaction
│   │   ├── mod.rs        # Database connection and queries
│   │   └── error.rs      # Custom error types
│   └── ui/               # User interface
│       ├── mod.rs        # UI layout and rendering
│       ├── input.rs      # Query input panel
│       ├── results.rs    # Query results panel
│       ├── sidebar.rs    # Navigation sidebar
│       ├── status.rs     # Status bar
│       └── table.rs      # Table rendering component
```

## Development Status

The project is in active development. Current focus is on:
1. Implementing database schema navigation
2. Adding query history
3. Adding syntax highlighting
4. Implementing auto-completion

## Contributing

The project is in early development stages. Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](/LICENSE.md) file for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - For the terminal user interface framework
- [sqlx](https://github.com/launchbadge/sqlx) - For the SQL toolkit
- [crossterm](https://github.com/crossterm-rs/crossterm) - For terminal manipulation
- [config-rs](https://github.com/mehcode/config-rs) - For configuration management

---
*Built with 🦀 Rust and ❤️*