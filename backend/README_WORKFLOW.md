# 📋 Daftar File Instruksi Workflow - Quick Reference

## File-File yang Dibuat

Semua file berikut dibuat TANPA mengubah code existing. File-file ini berisi instruksi pengerjaan lengkap.

### 1. Entities (SeaORM)
**File:** `backend/src/entities/mod.rs`
- ✅ Instruksi setup SeaORM entities
- ✅ Cara generate entities dari database
- ✅ Contoh struktur entity Model
- ✅ Export patterns

**Action Required:**
```bash
# Generate entities
sea-orm-cli generate entity -u postgresql://... -o src/entities
```

---

### 2. Service Layer (Business Logic)
**File:** `backend/src/services/mod.rs`
- ✅ Penjelasan lengkap design principles
- ✅ Vec Boundary Policy
- ✅ Immutability rules
- ✅ Enum-First Design
- ✅ Rayon integration patterns

**File:** `backend/src/services/workflow_service.rs`
- ✅ Template implementasi lengkap (commented)
- ✅ Enum definitions (StatusJadwal, HasilAnalisa)
- ✅ Service struct dengan DatabaseConnection
- ✅ Database query methods
- ✅ Processing methods (immutable, Rayon)
- ✅ Workflow orchestration
- ✅ Testing guidelines

**Action Required:**
```rust
// Uncomment semua code di workflow_service.rs
// Sesuaikan entity name dengan hasil generate
```

---

### 3. Routes (API Endpoints)
**File:** `backend/src/routes/workflow_routes.rs`
- ✅ Response types (serializable enums)
- ✅ Handler functions dengan dependency injection
- ✅ Axum router configuration
- ✅ JSON response examples
- ✅ Integration notes

**Action Required:**
```rust
// Uncomment code di workflow_routes.rs
// Implement handlers sesuai template
```

---

### 4. Main.rs Integration
**File:** `backend/src/main_baru.rs`
- ✅ Instruksi lengkap update main.rs
- ✅ Module declarations
- ✅ SeaORM connection setup
- ✅ Multiple state pattern (backward compatible)
- ✅ Router merge strategy
- ✅ Complete code template (commented)
- ✅ Migration path strategy

**Action Required:**
```rust
// JANGAN replace main.rs
// Baca main_baru.rs sebagai panduan
// Copy relevant parts ke main.rs
```

---

### 5. Dependencies Documentation
**File:** `backend/cargo_dependencies.md`
- ✅ Daftar lengkap dependencies baru
- ✅ Feature flags explanation
- ✅ Installation commands
- ✅ Alternative options
- ✅ Performance tuning
- ✅ Testing dependencies
- ✅ Common issues & solutions

**Action Required:**
```bash
# Update Cargo.toml dengan dependencies
cargo build
```

---

### 6. Workflow Guide
**File:** `backend/WORKFLOW_GUIDE.md`
- ✅ Overview lengkap
- ✅ Design principles
- ✅ Alur kerja (flowchart)
- ✅ Step-by-step implementation
- ✅ Testing guide
- ✅ Troubleshooting
- ✅ Design pattern explanations

**Action Required:**
```
# Baca sebagai panduan utama
# Follow step-by-step
```

---

## 🗂️ Struktur Folder Lengkap

```
backend/
├── src/
│   ├── main.rs                          # EXISTING - tidak diubah
│   ├── main_baru.rs                     # ✨ INSTRUKSI integrasi
│   │
│   ├── entities/                        # ✨ NEW
│   │   └── mod.rs                       # Instruksi & exports
│   │
│   ├── services/                        # ✨ NEW
│   │   ├── mod.rs                       # Instruksi design
│   │   └── workflow_service.rs          # Template implementasi
│   │
│   ├── routes/
│   │   ├── workflow_routes.rs           # ✨ NEW: API endpoints
│   │   └── (existing routes)            # EXISTING - tidak diubah
│   │
│   └── (existing structure)             # EXISTING - tidak diubah
│
├── cargo_dependencies.md                # ✨ Dokumentasi dependencies
├── WORKFLOW_GUIDE.md                    # ✨ Panduan lengkap
├── README_WORKFLOW.md                   # ✨ Quick reference (ini)
└── Cargo.toml                           # UPDATE dengan dependencies baru
```

---

## 🚀 Quick Start Guide

### Step 1: Install Dependencies (5 menit)
```bash
cd backend

# Update Cargo.toml
# Lihat: cargo_dependencies.md

cargo build
```

### Step 2: Generate Entities (5 menit)
```bash
# Install CLI
cargo install sea-orm-cli

# Generate
sea-orm-cli generate entity -u postgresql://user:pass@host/db -o src/entities

# Atau ikuti instruksi di: src/entities/mod.rs
```

### Step 3: Implement Service (15 menit)
```bash
# Edit: src/services/workflow_service.rs
# Uncomment semua code
# Sesuaikan entity name

# Lihat instruksi lengkap di file
```

### Step 4: Implement Routes (10 menit)
```bash
# Edit: src/routes/workflow_routes.rs
# Uncomment code
# Implement handlers

# Lihat instruksi di file
```

### Step 5: Update Main.rs (10 menit)
```bash
# Baca: src/main_baru.rs
# Copy relevant parts ke main.rs
# Setup SeaORM connection
# Merge routes

# JANGAN replace main.rs!
```

### Step 6: Test (5 menit)
```bash
# Check
cargo check

# Run
cargo run

# Test
curl http://localhost:3000/api/workflow/jadwal/terdekat
```

**Total Time: ~50 menit**

---

## 📚 Urutan Baca Dokumentasi

### 1. Pertama: Quick Reference
- ✅ `README_WORKFLOW.md` (ini) - Overview

### 2. Kedua: Dependencies
- ✅ `cargo_dependencies.md` - Install dulu

### 3. Ketiga: Workflow Guide
- ✅ `WORKFLOW_GUIDE.md` - Panduan lengkap

### 4. Keempat: Implementation Files
- ✅ `entities/mod.rs` - Entity setup
- ✅ `services/mod.rs` - Design principles
- ✅ `services/workflow_service.rs` - Implementation
- ✅ `routes/workflow_routes.rs` - API endpoints
- ✅ `main_baru.rs` - Integration

---

## ✅ Checklist Implementasi

### Phase 1: Setup
- [ ] Baca semua dokumentasi
- [ ] Update Cargo.toml
- [ ] Run `cargo build`
- [ ] Install sea-orm-cli

### Phase 2: Entities
- [ ] Setup DATABASE_URL di .env
- [ ] Generate entities
- [ ] Verify Model struct
- [ ] Update entities/mod.rs exports

### Phase 3: Service Layer
- [ ] Uncomment workflow_service.rs
- [ ] Sesuaikan entity name
- [ ] Implement enum definitions
- [ ] Implement database methods
- [ ] Implement processing methods
- [ ] Test dengan `cargo test`

### Phase 4: Routes
- [ ] Uncomment workflow_routes.rs
- [ ] Implement response types
- [ ] Implement handlers
- [ ] Setup router

### Phase 5: Integration
- [ ] Backup main.rs
- [ ] Add module declarations
- [ ] Setup SeaORM connection
- [ ] Create workflow state
- [ ] Merge routes
- [ ] Test compilation

### Phase 6: Testing
- [ ] Run server
- [ ] Test all endpoints
- [ ] Verify responses
- [ ] Check performance
- [ ] Review code quality

---

## 🎯 Key Principles Reminder

### 1. Vec Boundary Policy
```rust
// ✅ Vec HANYA dari database
let vec = db.query().await?;

// ✅ LANGSUNG convert ke slice
let slice = vec.as_slice();

// ✅ Processing pakai slice
process(slice);  // Not Vec!
```

### 2. Enum-First Design
```rust
// ✅ Return enum, bukan tuple/raw struct
enum HasilAnalisa {
    JadwalTerdekat(StatusJadwal),
    TidakAdaJadwal,
    Error(String),
}
```

### 3. Strict Immutability
```rust
// ❌ NO
fn process(mut data: Vec<T>) { ... }

// ✅ YES
fn process(data: &[T]) -> Result { ... }
```

### 4. Rayon Integration
```rust
// ✅ Parallel processing
data.par_iter()
    .filter(|x| x.is_valid())
    .min_by_key(|x| x.priority)
```

---

## 🔗 API Endpoints

Setelah implementasi, endpoints yang tersedia:

```
GET  /api/workflow/jadwal/terdekat
     → Jadwal terdekat dengan sekarang

GET  /api/workflow/jadwal/studio/:studio_id
     → Jadwal terdekat untuk studio tertentu

GET  /api/workflow/jadwal/movie/:movie_id
     → Jadwal terdekat untuk movie tertentu

GET  /api/workflow/jadwal/stats
     → Statistik jadwal (demo functional methods)
```

---

## 💡 Tips

### Tip 1: Incremental Implementation
Jangan implement semua sekaligus. Urutan:
1. Entities only → Test
2. Service database method → Test
3. Service processing → Test
4. Routes → Test
5. Integration → Test

### Tip 2: Use Clippy
```bash
cargo clippy -- -W clippy::all
```

### Tip 3: Format Code
```bash
cargo fmt
```

### Tip 4: Check Documentation
```bash
cargo doc --open
```

---

## 🐛 Quick Troubleshooting

| Error | File to Check |
|-------|---------------|
| Cannot find entity | `entities/mod.rs` |
| No par_iter method | Add `use rayon::prelude::*;` |
| DateTime type mismatch | Check sea-orm features |
| Multiple state error | See `main_baru.rs` pattern |
| Database connection fail | Check DATABASE_URL in .env |

---

## 📞 Support

Jika ada masalah:
1. Baca error message carefully
2. Check instruksi di file terkait
3. Lihat WORKFLOW_GUIDE.md troubleshooting section
4. Review cargo_dependencies.md untuk dependency issues

---

## 🎓 Learning Path

Untuk memahami lebih dalam:

1. **SeaORM Basics**
   - Entities & Models
   - Queries & Relations
   - Migrations

2. **Rayon Fundamentals**
   - par_iter() vs iter()
   - Work stealing
   - Performance tuning

3. **Functional Rust**
   - Immutability
   - Iterator patterns
   - Error handling

4. **Axum Web Framework**
   - State management
   - Handlers
   - Middleware

---

## ✨ Summary

**Files Created:** 7 instruksi files
**Files Modified:** 0 (backward compatible)
**Time Required:** ~50 menit
**Complexity:** Medium
**Benefits:** High performance, type safety, maintainability

**Principles:**
- ✅ Vec Boundary
- ✅ Immutability
- ✅ Enum-First
- ✅ Rayon Parallel
- ✅ Type Safety

**Result:**
Fast, safe, maintainable backend workflow dengan modern Rust best practices.

---

**Start from:** `WORKFLOW_GUIDE.md`  
**Reference:** This file (`README_WORKFLOW.md`)  
**Dependencies:** `cargo_dependencies.md`

**Happy Coding! 🦀**
