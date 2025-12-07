// ============================================================================
// JADWAL WORKFLOW SERVICE - RAYON MULTIPROCESSING IMPLEMENTATION
// ============================================================================
// NOTE: SeaORM untuk database fetch, Rayon untuk parallel processing
// ============================================================================

use crate::entities::{Showtime, ShowtimesEntity};
use chrono::{DateTime, Local, Datelike, Timelike};
use rayon::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::HashMap;
use std::sync::Arc;

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

#[derive(Debug, Clone)]
pub struct JadwalStatistik {
    pub total_jadwal: usize,
    pub jadwal_hari_ini: usize,
    pub jadwal_minggu_ini: usize,
    pub jadwal_mendesak: usize,
    pub harga_rata_rata: f64,
    pub harga_tertinggi: Option<i64>,
    pub harga_terendah: Option<i64>,
    pub studio_terpopuler: Option<i64>,
    pub movie_terpopuler: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct FilterKriteria {
    pub studio_id: Option<i64>,
    pub movie_id: Option<i64>,
    pub min_harga: Option<i64>,
    pub max_harga: Option<i64>,
    pub hari_ini_saja: bool,
    pub hanya_mendesak: bool,
}

pub struct JadwalWorkflowService {
    db: DatabaseConnection,
}

impl JadwalWorkflowService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // ========================================================================
    // DATABASE LAYER - SeaORM
    // ========================================================================
    
    pub async fn fetch_jadwal_dari_db(&self) -> Result<Vec<Showtime>, sea_orm::DbErr> {
        ShowtimesEntity::find().all(&self.db).await
    }

    // ========================================================================
    // RAYON MULTIPROCESSING - PARALLEL OPERATIONS
    // ========================================================================
    
    /// RAYON: Cari jadwal terdekat dengan parallel processing
    pub fn cari_jadwal_terdekat(jadwal_slice: &[Showtime]) -> HasilAnalisa {
        if jadwal_slice.is_empty() {
            return HasilAnalisa::TidakAdaJadwal;
        }

        let waktu_sekarang = Local::now();

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

    /// 🔥 MULTIPROCESSING: Filter jadwal terdekat untuk film di semua bioskop
        }
    }

    /// 🔥 RAYON: Filter jadwal terdekat per film di semua bioskop (parallel grouping)
        
        // 🔥 STEP 1: Group jadwal by movie_id secara parallel
        let waktu_sekarang = Local::now();
        
        // 🔥 RAYON: Parallel grouping by movie_id
            .fold(
                HashMap::new,
                |mut map: HashMap<i64, Vec<&Showtime>>, (movie_id, jadwal)| {
                    map.entry(movie_id).or_insert_with(Vec::new).push(jadwal);
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, mut v) in map2 {
                        map1.entry(k).or_insert_with(Vec::new).append(&mut v);
                    }
                    map1
                },
            );
        
                },
            );
        
        // 🔥 RAYON: Parallel search jadwal terdekat per film
                // Cari jadwal terdekat untuk film ini di semua bioskop
                let jadwal_terdekat = jadwal_list
                    .par_iter()
                    .filter_map(|jadwal| jadwal.start_time.map(|st| (*jadwal, st)))
                    .filter(|(_, start_time)| *start_time > waktu_sekarang)
                    .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());
                
                jadwal_terdekat.map(|(jadwal, start_time)| {
                    let selisih = start_time - waktu_sekarang;
                    let selisih_menit = selisih.num_minutes();
                    
                    let status = if selisih_menit <= 30 {
                        StatusJadwal::Mendesak {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                            selisih_menit,
                        }
                    } else {
                        StatusJadwal::Aman {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                        }
                    };
                    
                    (*movie_id, status)
                })
            })
            .collect()
    }
    
            .collect()
    }
    
    /// 🔥 RAYON: Filter jadwal terdekat untuk 1 film di semua bioskop (parallel grouping)
    ) -> Vec<(i64, StatusJadwal)> {
        let waktu_sekarang = Local::now();
        
        // 🔥 STEP 1: Filter jadwal untuk movie_id tertentu + group by studio
        let waktu_sekarang = Local::now();
        
        // 🔥 RAYON: Parallel filter & grouping by studio
            .filter_map(|j| j.studio_id.map(|sid| (sid, j)))
            .fold(
                HashMap::new,
                |mut map: HashMap<i64, Vec<&Showtime>>, (studio_id, jadwal)| {
                    map.entry(studio_id).or_insert_with(Vec::new).push(jadwal);
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, mut v) in map2 {
                        map1.entry(k).or_insert_with(Vec::new).append(&mut v);
                    }
                    map1
                },
            );
                },
            );
        
        // 🔥 RAYON: Parallel search jadwal terdekat per bioskop
            .filter_map(|(studio_id, jadwal_list)| {
                // Cari jadwal terdekat untuk film ini di studio/bioskop ini
                let jadwal_terdekat = jadwal_list
                    .par_iter()
                    .filter_map(|jadwal| jadwal.start_time.map(|st| (*jadwal, st)))
                    .filter(|(_, start_time)| *start_time > waktu_sekarang)
                    .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());
                
                jadwal_terdekat.map(|(jadwal, start_time)| {
                    let selisih = start_time - waktu_sekarang;
                    let selisih_menit = selisih.num_minutes();
                    
                    let status = if selisih_menit <= 30 {
                        StatusJadwal::Mendesak {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                            selisih_menit,
                        }
                    } else {
                        StatusJadwal::Aman {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                        }
                    };
                    
                    (*studio_id, status)
                })
            })
            .collect()
    }
            .collect()
    }

    /// 🔥 RAYON: Multi-level filtering dengan parallel execution
        
        // 🚀 RAYON PARALLEL FILTER: Multi-kondisi filter secara concurrent
        let waktu_sekarang = Local::now();
        
        // 🔥 RAYON: Parallel multi-condition filter
        jadwal_slice
            .par_iter()
            .filter(|jadwal| {alse;
                    }
                }

                // Filter movie
                }
false;
                    }
                }

                // Filter harga minimum
                if let Some(min_harga) = kriteria.min_harga {
                }
e;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter harga maksimum
                if let Some(max_harga) = kriteria.max_harga {
                    }
                }

                    }
                }

                // Filter hari ini saja
                if kriteria.hari_ini_saja {
                    if let Some(start_time) = jadwal.start_time {
                    }
                }

                        return false;
                    }
                }

                // Filter hanya jadwal mendesak (dalam 30 menit)
                if kriteria.hanya_mendesak {
                    if let Some(start_time) = jadwal.start_time {
                    }
                }

                    } else {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// 🔥 MULTIPROCESSING: Agregasi statistik dengan parallel reduce
    /// Kalkulasi statistik kompleks menggunakan parallel aggregation
    pub fn hitung_statistik(jadwal_slice: &[Showtime]) -> JadwalStatistik {
        let waktu_sekarang = Local::now();
            .collect()
    }

    /// 🔥 RAYON: Agregasi statistik dengan parallel reduce
            total_jadwal,
            jadwal_hari_ini,
            jadwal_minggu_ini,
        let hari_ini = waktu_sekarang.date_naive();
        let minggu_ini_start = waktu_sekarang.date_naive() - chrono::Duration::days(7);

        // 🔥 RAYON: Parallel fold + reduce untuk aggregation
            .par_iter() // 🔥 PARALLEL: Process semua jadwal concurrent
            .fold(
                || (0usize, 0usize, 0usize, 0usize, 0i64, 0usize),
                |(total, hari_ini_count, minggu_ini_count, mendesak, harga_sum, harga_count), jadwal| {
                    let mut new_total = total + 1;
                    let mut new_hari_ini = hari_ini_count;
                    let mut new_minggu_ini = minggu_ini_count;
        ) = jadwal_slice
            .par_iter()
                    let mut new_harga_count = harga_count;

                    // Count hari ini
                    if let Some(start_time) = jadwal.start_time {
                        if start_time.date_naive() == hari_ini {
                            new_hari_ini += 1;
                        }
                    let mut new_harga_sum = harga_sum;
                    let mut new_harga_count = harga_count;

                        // Count mendesak
                        let selisih = (start_time - waktu_sekarang).num_minutes();
                        if selisih > 0 && selisih <= 30 {
                            new_mendesak += 1;
                        }
                        if start_time.date_naive() >= minggu_ini_start {
                            new_minggu_ini += 1;
                        }
price;
                        new_harga_count += 1;
                    }

                    (new_total, new_hari_ini, new_minggu_ini, new_mendesak, new_harga_sum, new_harga_count)
                        }
                    }
0, 0),
                |(t1, h1, m1, md1, hs1, hc1), (t2, h2, m2, md2, hs2, hc2)| {
                    (t1 + t2, h1 + h2, m1 + m2, md1 + md2, hs1 + hs2, hc1 + hc2)
                },
            );

        // 🚀 PARALLEL: Cari harga tertinggi & terendah concurrent
        let harga_tertinggi = jadwal_slice
            .par_iter()
            .filter_map(|j| j.price)
            .max();

        let harga_terendah = jadwal_slice
            .par_iter()
                },
            );

        // 🔥 RAYON: Parallel min/max search
        let studio_terpopuler = Self::studio_terpopuler_parallel(jadwal_slice);
        let movie_terpopuler = Self::movie_terpopuler_parallel(jadwal_slice);

        let harga_rata_rata = if count_harga > 0 {
            total_harga as f64 / count_harga as f64
        } else {
            .par_iter()
            .filter_map(|j| j.price)
            .min();

            jadwal_hari_ini,
            jadwal_minggu_ini,
            jadwal_mendesak,
            harga_rata_rata,
            harga_tertinggi,
            harga_terendah,
            studio_terpopuler,
            movie_terpopuler,
        }
    }

    /// 🔥 MULTIPROCESSING: Cari studio terpopuler dengan parallel grouping
    fn studio_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        use std::collections::HashMap;
        
        // 🚀 PARALLEL: Group by studio_id dan count concurrent
        let studio_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.studio_id)
            .fold(
        }
    }

    /// 🔥 RAYON: Cari studio terpopuler dengan parallel grouping
    fn studio_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        use std::collections::HashMap;
        
        // 🔥 RAYON: Parallel grouping & counting
                |mut map1, map2| {
                    for (k, v) in map2 {
                        *map1.entry(k).or_insert(0) += v;
                    }
                    map1
                },
            );

        studio_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(studio_id, _)| studio_id)
    }

    /// 🔥 MULTIPROCESSING: Cari movie terpopuler dengan parallel grouping
    fn movie_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        use std::collections::HashMap;
        
        // 🚀 PARALLEL: Group by movie_id dan count concurrent
        let movie_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.movie_id)
            .fold(
            .map(|(studio_id, _)| studio_id)
    }

    /// 🔥 RAYON: Cari movie terpopuler dengan parallel grouping
    fn movie_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        use std::collections::HashMap;
        
        // 🔥 RAYON: Parallel grouping & counting
                |mut map1, map2| {
                    for (k, v) in map2 {
                        *map1.entry(k).or_insert(0) += v;
                    }
                    map1
                },
            );

        movie_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(movie_id, _)| movie_id)
    }

    /// 🔥 MULTIPROCESSING: Filter by studio (parallel)
    pub fn filter_by_studio(jadwal_slice: &[Showtime], studio_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter() // 🔥 PARALLEL
            .filter(|jadwal| jadwal.studio_id == Some(studio_id))
            .collect()
    }

    /// 🔥 MULTIPROCESSING: Filter by movie (parallel)
            .map(|(movie_id, _)| movie_id)
    }

    /// 🔥 RAYON: Filter by studiovie_id))
    pub fn filter_by_studio(jadwal_slice: &[Showtime], studio_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
    /// Hitung jumlah jadwal yang tersedia
            .collect()
    }

    /// 🔥 RAYON: Filter by movie
    pub fn filter_by_movie(jadwal_slice: &[Showtime], movie_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .par_iter() // 🔥 PARALLEL
            .filter(|jadwal| jadwal.price.is_some())
            .max_by_key(|jadwal| jadwal.price.unwrap())
    }

    // ========================================================================
        jadwal_slice.len()
    }

    /// 🔥 RAYON: Cari jadwal dengan harga tertinggi
    pub fn jadwal_harga_tertinggi(jadwal_slice: &[Showtime]) -> Option<&Showtime> {
        jadwal_slice
            .par_iter()f) -> Result<HasilAnalisa, String> {
        // Step 1: Fetch dari DB menggunakan SeaORM (tidak diubah)
            .max_by_key(|jadwal| jadwal.price.unwrap())
    }

    // ========================================================================
    // WORKFLOW ORCHESTRATION
    // ========================================================================lice);

        Ok(hasil)
    }

    pub async fn execute_workflow(&self) -> Result<HasilAnalisa, String> { -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let jadwal_slice = jadwal_vec.as_slice();d_jadwal.as_slice());

        Ok(hasil)
    }

    /// Workflow dengan filter movie (parallel)
        Ok(hasil)
    }

    /// Workflow dengan filter studio
            .map_err(|e| format!("Database error: {}", e))?;

        // 🔥 PARALLEL FILTER
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_studio(jadwal_vec.as_slice(), studio_id);
    }

    /// 🔥 MULTIPROCESSING: Workflow dengan filter kompleks (parallel multi-condition)
    pub async fn execute_workflow_kompleks(&self, kriteria: FilterKriteria) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
        Ok(hasil)
    }

    /// Workflow dengan filter movie
        // 🔥 PARALLEL COMPLEX FILTER
        let filtered = Self::filter_kompleks(jadwal_vec.as_slice(), &kriteria);
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_movie(jadwal_vec.as_slice(), movie_id);
    /// 🔥 MULTIPROCESSING: Workflow untuk mendapatkan jadwal terdekat per film di semua bioskop
    pub async fn execute_workflow_jadwal_film_semua_bioskop(
        &self,
        movie_id: i64,
        Ok(hasil)
    }

    /// Workflow dengan filter kompleks
            .map_err(|e| format!("Database error: {}", e))?;

        // 🔥 PARALLEL: Cari jadwal terdekat untuk film ini di setiap bioskop
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_kompleks(jadwal_vec.as_slice(), &kriteria);
    /// 🔥 MULTIPROCESSING: Workflow untuk mendapatkan jadwal terdekat semua film
    pub async fn execute_workflow_jadwal_semua_film(&self) -> Result<Vec<(i64, StatusJadwal)>, String> {
        let jadwal_vec = self
        Ok(hasil)
    }

    /// Workflow jadwal terdekat per film di semua bioskop
        // 🔥 PARALLEL: Cari jadwal terdekat untuk setiap film
        let hasil = Self::filter_jadwal_terdekat_per_film(jadwal_vec.as_slice());

        Ok(hasil)
    }

            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_film_semua_bioskop(jadwal_vec.as_slice(), movie_id);
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(hasil)
    }

    /// Workflow jadwal terdekat untuk semua film
            .map(|chunk| Self::cari_jadwal_terdekat(chunk))
            .collect();

            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_per_film(jadwal_vec.as_slice());ing> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // 🔥 PARALLEL STATISTICS CALCULATION
        let statistik = Self::hitung_statistik(jadwal_vec.as_slice());

        Ok(statistik)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil_batch = jadwal_vec
        Ok(hasil_batch)
    }

            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let statistik = Self::hitung_statistik(jadwal_vec.as_slice());
        Ok(statistik)
    }