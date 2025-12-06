// ============================================================================
// JADWAL WORKFLOW SERVICE IMPLEMENTATION
// ============================================================================

use crate::entities::{Showtime, ShowtimesEntity};
use chrono::{DateTime, Local};
use rayon::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};

// ============================================================================
// ENUM DEFINITIONS
// ============================================================================

#[derive(Debug, Clone)]
pub enum StatusJadwal {
    Mendesak {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
        selisih_menit: i64,
    },
    Aman {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
    },
    Selesai {
        jadwal_id: i64,
        waktu_selesai: DateTime<Local>,
    },
}

#[derive(Debug)]
pub enum HasilAnalisa {
    JadwalTerdekat(StatusJadwal),
    TidakAdaJadwal,
    Error(String),
}

// ============================================================================
// SERVICE STRUCT
// ============================================================================

pub struct JadwalWorkflowService {
    db: DatabaseConnection,
}

impl JadwalWorkflowService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // ========================================================================
    // DATABASE LAYER (Vec Boundary)
    // ========================================================================

    /// Fetch jadwal dari database
    /// Vec HANYA digunakan sebagai return dari query database
    pub async fn fetch_jadwal_dari_db(&self) -> Result<Vec<Showtime>, sea_orm::DbErr> {
        ShowtimesEntity::find().all(&self.db).await
    }

    // ========================================================================
    // PROCESSING LAYER (Immutable, Slice-based)
    // ========================================================================

    /// Cari jadwal terdekat dengan Rayon parallel processing
    /// Parameter: &[Showtime] - Slice (immutable)
    /// No mut keyword, no Vec mutation
    pub fn cari_jadwal_terdekat(jadwal_slice: &[Showtime]) -> HasilAnalisa {
        if jadwal_slice.is_empty() {
            return HasilAnalisa::TidakAdaJadwal;
        }

        let waktu_sekarang = Local::now();

        // Rayon parallel iterator
        let jadwal_terdekat = jadwal_slice
            .par_iter()
            .filter_map(|jadwal| jadwal.start_time.map(|st| (jadwal, st)))
            .filter(|(_, start_time)| *start_time > waktu_sekarang)
            .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());

        match jadwal_terdekat {
            Some((jadwal, start_time)) => {
                let selisih = start_time - waktu_sekarang;
                let selisih_menit = selisih.num_minutes();

                if selisih_menit <= 30 {
                    HasilAnalisa::JadwalTerdekat(StatusJadwal::Mendesak {
                        jadwal_id: jadwal.id,
                        waktu_mulai: start_time,
                        selisih_menit,
                    })
                } else {
                    HasilAnalisa::JadwalTerdekat(StatusJadwal::Aman {
                        jadwal_id: jadwal.id,
                        waktu_mulai: start_time,
                    })
                }
            }
            None => HasilAnalisa::TidakAdaJadwal,
        }
    }

    /// Filter jadwal by studio (parallel)
    pub fn filter_by_studio(jadwal_slice: &[Showtime], studio_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.studio_id == Some(studio_id))
            .collect()
    }

    /// Filter jadwal by movie (parallel)
    pub fn filter_by_movie(jadwal_slice: &[Showtime], movie_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.movie_id == Some(movie_id))
            .collect()
    }

    /// Hitung jumlah jadwal yang tersedia
    pub fn count_jadwal(jadwal_slice: &[Showtime]) -> usize {
        jadwal_slice.len()
    }

    /// Cari jadwal dengan harga tertinggi
    pub fn jadwal_harga_tertinggi(jadwal_slice: &[Showtime]) -> Option<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.price.is_some())
            .max_by_key(|jadwal| jadwal.price.unwrap())
    }

    // ========================================================================
    // WORKFLOW ORCHESTRATION
    // ========================================================================

    /// Main workflow: Fetch → Slice → Process
    pub async fn execute_workflow(&self) -> Result<HasilAnalisa, String> {
        // Step 1: Fetch dari DB (Vec boundary)
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Step 2: Convert ke slice SEGERA
        let jadwal_slice = jadwal_vec.as_slice();

        // Step 3: Process (immutable)
        let hasil = Self::cari_jadwal_terdekat(jadwal_slice);

        Ok(hasil)
    }

    /// Workflow dengan filter studio
    pub async fn execute_workflow_by_studio(&self, studio_id: i64) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_studio(jadwal_vec.as_slice(), studio_id);

        // Convert Vec<&Showtime> ke Vec<Showtime> untuk slice
        let owned_jadwal: Vec<Showtime> = filtered.into_iter().cloned().collect();
        let hasil = Self::cari_jadwal_terdekat(owned_jadwal.as_slice());

        Ok(hasil)
    }

    /// Workflow dengan filter movie
    pub async fn execute_workflow_by_movie(&self, movie_id: i64) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_movie(jadwal_vec.as_slice(), movie_id);
        let owned_jadwal: Vec<Showtime> = filtered.into_iter().cloned().collect();
        let hasil = Self::cari_jadwal_terdekat(owned_jadwal.as_slice());

        Ok(hasil)
    }
}
