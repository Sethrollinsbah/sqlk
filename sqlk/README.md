# SQLK - Interactive SQL Query Tool

A terminal-based SQL query execution and visualization tool with vim-like
navigation and Matrix-style loading animations.

## Features

- **Interactive SQL Query Execution**: Execute SQL queries from files within
cursor-based selection
- **Database Support**: Currently supports PostgreSQL (MySQL and SQLite support planned)
- **Table Visualization**: Browse query results in an interactive table viewer
- **Foreign Key Navigation**: Automatic foreign key detection and lookup
- **Vim-like Navigation**: Familiar keybindings for file and table navigation
- **Matrix Animation**: Optional loading animation while connecting to database
- **Clipboard Integration**: Copy rows and cells to system clipboard
- **Search Functionality**: Search within query results
- **Query Block Parsing**: Automatically detect and execute SQL query blocks in files

## Installation

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL client libraries
- A terminal with Unicode support

### Build from Source

```bash
git clone https://github.com/SethRollinsBah/SQLk.git
cd sqlk
cargo build --release
```

The binary will be available at `target/release/sqlk`.

## Usage

### Basic Usage

```bash
# Execute with a SQL file
sqlk --file queries.sql

# Use a custom .env file
sqlk --env .env.development --file queries.sql

# Execute a specific query
sqlk --query "SELECT * FROM users LIMIT 10"

# Combine file and environment
sqlk --env .env.production --file analytics.sql
```

### Environment Setup

Create a `.env` file in your project directory:

```bash
DATABASE_URL="postgresql://username:password@localhost:5432/database_name"
```

### Configuration

SQLK uses a configuration file located at:

- **Linux/macOS**: `~/.config/sqlk/config.toml`
- **Windows**: `%APPDATA%\sqlk\config.toml`

Example configuration:

```toml
env_file = ".env"

[matrix]
enabled = true
duration_ms = 6000
chars = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ01"
width = 80
height = 24

[foreign_keys]
enabled = true

[foreign_keys.manual_mapping]
user_id = "users.id"
product_id = "products.id"

[database]
url = "postgresql://localhost:5432/mydb"
db_type = "PostgreSQL" # optional, SQLk finds database from string
```

## Keybindings

### File View Mode

| Key | Action |
|-----|--------|
| `j/k` or `↓/↑` | Navigate lines |
| `PageUp/PageDown` | Page navigation |
| `Home/End` | Jump to start/end |
| `e` | Execute query at cursor |
| `?` | Show help |
| `q` or `Esc` | Quit |

### Table Viewer Mode

| Key | Action |
|-----|--------|
| `h/j/k/l` or Arrow keys | Navigate cells |
| `yy` | Copy entire row |
| `yiw` | Copy current cell |
| `/` | Search in results |
| `F` | Foreign key lookup |
| `K` | Show cell info |
| `?` | Show help |
| `q` or `Esc` | Back to file view |

### Search Mode

| Key | Action |
|-----|--------|
| `Enter` | Execute search |
| `Esc` | Cancel search |
| `Backspace` | Delete character |

## SQL File Format

SQLK can parse and execute SQL query blocks from files. Queries can be separated
by:

- Empty lines
- SQL comments (`--` or `/* */`)
- Semicolons

Example file:

```sql
-- User analytics query
SELECT 
    u.id,
    u.email,
    COUNT(o.id) as order_count
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
GROUP BY u.id, u.email
ORDER BY order_count DESC
LIMIT 20;

-- Product performance
SELECT 
    p.name,
    SUM(oi.quantity) as total_sold,
    SUM(oi.price * oi.quantity) as revenue
FROM products p
JOIN order_items oi ON p.id = oi.product_id
WHERE oi.created_at > NOW() - INTERVAL '30 days'
GROUP BY p.id, p.name
ORDER BY revenue DESC;
```

## Foreign Key Navigation

SQLK automatically detects foreign key relationships in PostgreSQL databases.
When viewing query results:

1. Navigate to a cell containing a foreign key value
2. Press `F` to lookup related records
3. Use `K` to view detailed cell information including foreign key relationships

### Manual Foreign Key Mapping

You can define custom foreign key relationships in your config:

```toml
[foreign_keys.manual_mapping]
user_id = "users.id"
customer_id = "customers.id"
order_id = "orders.id"
```

## Database Support

### PostgreSQL

- Full support for all PostgreSQL data types
- Automatic foreign key detection
- Connection pooling
- SSL support

### Planned Database Support

- MySQL
- SQLite
- Microsoft SQL Server

## Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── app/                 # Main application logic
│   ├── mod.rs
│   ├── state.rs         # Application state management
│   ├── events.rs        # Event loop and updates
│   ├── modes.rs         # Mode-specific handlers
│   └── input.rs         # Input processing
├── database/            # Database abstraction layer
│   ├── mod.rs
│   ├── manager.rs       # Database manager trait
│   └── postgres/        # PostgreSQL implementation
├── config/              # Configuration management
│   ├── mod.rs
│   ├── types.rs         # Config structures
│   ├── loader.rs        # File loading
│   └── parser.rs        # URL parsing
├── ui/                  # User interface
├── table_viewer/        # Table display logic
└── matrix/              # Matrix animation
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- --file test.sql
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Dependencies

- `sqlx` - Async SQL toolkit
- `tokio` - Async runtime
- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation
- `anyhow` - Error handling
- `serde` - Serialization
- `toml` - Configuration file format
- `clap` - Command line argument parsing

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Connection Issues

#### Error: "No DATABASE_URL found"

- Ensure your `.env` file contains a valid `DATABASE_URL`
- Check that the `.env` file is in the correct location
- Verify the database URL format

#### Error: "relative URL without a base"

- Check your DATABASE_URL format - it should start with `postgresql://`
- Remove any extra quotes around the URL in your `.env` file

### Performance Issues

#### Slow query execution

- Use `LIMIT` clauses for large result sets
- Check database indexes for query optimization
- Consider connection pooling settings

### Display Issues

#### Matrix animation not working

- Ensure your terminal supports Unicode characters
- Try disabling the matrix animation in config: `matrix.enabled = false`

#### Table display problems

- Verify terminal size and font settings
- Try a smaller result set
- Check terminal color support

## Support

For issues, questions, or contributions:

- Open an issue on GitHub
- Check existing documentation
- Review the troubleshooting section above
