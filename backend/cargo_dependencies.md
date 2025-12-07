# Cargo.toml Dependencies untuk Workflow SeaORM & Rayon

Dokumentasi ini berisi semua dependencies yang diperlukan untuk implementasi workflow dengan SeaORM, Rayon, dan Chrono.

## 📦 Dependencies yang Perlu Ditambahkan

Tambahkan dependencies berikut ke file `Cargo.toml`:

```toml
[dependencies]
# === EXISTING DEPENDENCIES (Tetap dipertahankan) ===
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenvy = "0.15"
tower-http = { version = "0.5", features = ["cors"] }
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
bcrypt = "0.15"
jsonwebtoken = "9.2"

# === NEW DEPENDENCIES UNTUK WORKFLOW ===

# SeaORM - ORM untuk Rust dengan support async
sea-orm = { version = "0.12", features = [
    "sqlx-postgres",           # PostgreSQL driver
    "runtime-tokio-native-tls", # Tokio runtime
    "macros",                   # Derive macros
    "with-chrono"               # Chrono integration
] }

# Rayon - Parallel processing library
rayon = "1.8"

# Chrono - Date and time library
chrono = { version = "0.4", features = ["serde"] }

# === OPTIONAL: Development Tools ===
[dev-dependencies]
# Sea-orm migration tool (optional, untuk database migrations)
sea-orm-migration = "0.12"
```

## 🔧 Installation Commands

### 1. Install Dependencies
```bash
cd backend
cargo build
```

### 2. Install SeaORM CLI (Optional, untuk generate entities)
```bash
cargo install sea-orm-cli
```

### 3. Verify Installation
```bash
cargo check
```

## 📚 Dependency Explanations

### SeaORM (0.12+)
**Purpose:** ORM (Object-Relational Mapping) untuk database operations

**Features:**
- `sqlx-postgres`: PostgreSQL database driver
- `runtime-tokio-native-tls`: Async runtime dengan TLS support
- `macros`: Derive macros untuk entity definitions
- `with-chrono`: Integration dengan Chrono untuk DateTime fields

**Alternatives:**
- `sqlx-mysql`: Untuk MySQL database
- `sqlx-sqlite`: Untuk SQLite database

**Usage Example:**
```rust
use sea_orm::{Database, DatabaseConnection, EntityTrait};

let db: DatabaseConnection = Database::connect("postgresql://...").await?;
let jadwal = JadwalEntity::find().all(&db).await?;
```

### Rayon (1.8+)
**Purpose:** Data parallelism library untuk pemrosesan paralel

**Key Features:**
- `par_iter()`: Parallel iterator
- `par_iter_mut()`: Mutable parallel iterator
- Thread pool management automatic
- Work-stealing scheduler

**Usage Example:**
```rust
use rayon::prelude::*;

let result = data_slice
    .par_iter()
    .filter(|x| x.is_valid())
    .min_by_key(|x| x.priority);
```

### Chrono (0.4+)
**Purpose:** Date and time handling

**Features:**
- `serde`: Serialization/deserialization support
- `DateTime<Utc>`: UTC timestamps
- `Duration`: Time duration calculations
- Timezone handling

**Usage Example:**
```rust
use chrono::{DateTime, Utc, Duration};

let now = Utc::now();
let later = now + Duration::hours(2);
let diff = later - now;
```

## 🔄 Migration from Existing Setup

### Current Stack:
- SQLx for database (raw SQL)
- No parallel processing
- Manual date handling

### New Stack (Workflow):
- SeaORM for type-safe queries
- Rayon for parallel processing
- Chrono for date operations

### Compatibility:
- ✅ Existing SQLx code tetap berfungsi
- ✅ Dapat digunakan bersamaan (coexistence)
- ✅ Gradual migration path available

## 🚀 Generate Entities from Database

Setelah install sea-orm-cli:

```bash
# Generate entities dari database yang sudah ada
sea-orm-cli generate entity \
    -u postgresql://user:password@localhost:5432/tioskop_db \
    -o src/entities
```

Atau manual config dengan `sea-orm-cli.toml`:

```toml
# sea-orm-cli.toml
[database]
url = "postgresql://user:password@localhost:5432/tioskop_db"

[output]
path = "src/entities"
lib = true
```

Then run:
```bash
sea-orm-cli generate entity
```

## 📊 Database Schema Requirements

Untuk workflow jadwal, pastikan table `jadwal` (atau `showtimes`) memiliki:

```sql
CREATE TABLE jadwal (
    id SERIAL PRIMARY KEY,
    movie_id INTEGER NOT NULL,
    studio_id INTEGER NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    price INTEGER NOT NULL,
    available_seats INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## 🔐 Environment Variables

Update `.env` file:

```env
# Existing
DATABASE_URL=postgresql://user:password@localhost:5432/tioskop_db
JWT_SECRET=tioskop_dev_secret

# NEW: SeaORM Connection (bisa sama dengan DATABASE_URL)
DATABASE_URL=postgresql://user:password@localhost:5432/tioskop_db

# Optional: Connection pool settings
SEA_ORM_MAX_CONNECTIONS=10
SEA_ORM_MIN_CONNECTIONS=2
```

## ⚡ Performance Considerations

### Rayon Thread Pool
Default: Number of logical CPUs
Configure:
```rust
rayon::ThreadPoolBuilder::new()
    .num_threads(8)
    .build_global()
    .unwrap();
```

### SeaORM Connection Pool
Configure in connection string or code:
```rust
let mut opt = ConnectOptions::new(database_url);
opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8));

let db = Database::connect(opt).await?;
```

## 🧪 Testing Dependencies

Tambahkan untuk testing:

```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"  # Untuk mocking
```

## 📦 Build Profile Optimization

Tambahkan di Cargo.toml untuk performance:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = 1  # Faster compilation, reasonable performance
```

## 🔍 Verify Installation

Setelah cargo build, verify:

```bash
# Check dependencies
cargo tree | grep sea-orm
cargo tree | grep rayon
cargo tree | grep chrono

# Check untuk conflicts
cargo tree --duplicates

# Update dependencies
cargo update
```

## 📖 Documentation Links

- **SeaORM:** https://www.sea-ql.org/SeaORM/
- **Rayon:** https://docs.rs/rayon/latest/rayon/
- **Chrono:** https://docs.rs/chrono/latest/chrono/
- **Axum:** https://docs.rs/axum/latest/axum/

## ⚠️ Common Issues & Solutions

### Issue 1: SeaORM version mismatch
```
Solution: Pastikan sea-orm dan sea-orm-cli versinya sama
cargo install sea-orm-cli --version 0.12
```

### Issue 2: Feature flags missing
```
Error: no method named `all` found for struct `Select`
Solution: Tambahkan feature "macros" di sea-orm dependency
```

### Issue 3: Chrono timezone issues
```
Solution: Gunakan DateTime<Utc> konsisten, avoid naive datetime
```

### Issue 4: Rayon stack overflow
```
Solution: Reduce parallel depth atau increase stack size
```

## ✅ Checklist Installation

- [ ] Backup Cargo.toml yang lama
- [ ] Tambahkan dependencies baru
- [ ] Run `cargo build`
- [ ] Install sea-orm-cli (optional)
- [ ] Generate entities dari database
- [ ] Verify dengan `cargo check`
- [ ] Test compile dengan `cargo test --no-run`
- [ ] Update .env dengan DATABASE_URL
- [ ] Test connection dengan simple query
- [ ] Review cargo.lock untuk version conflicts

## 🎯 Next Steps

Setelah dependencies terinstall:

1. ✅ Generate entities (lihat entities/mod.rs)
2. ✅ Implementasi workflow service (lihat services/workflow_service.rs)
3. ✅ Implementasi routes (lihat routes/workflow_routes.rs)
4. ✅ Update main.rs (lihat main_baru.rs)
5. ✅ Testing endpoints
6. ✅ Performance benchmarking

---

**Last Updated:** 2025-12-07
**Rust Version:** 1.75+
**Edition:** 2021
