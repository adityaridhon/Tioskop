
use crate::entities::{Showtime, ShowtimesEntity};
use chrono::{DateTime, Local};
use rayon::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::HashMap;

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
    pub harga_tertinggi: Option<Decimal>,
    pub harga_terendah: Option<Decimal>,
    pub studio_terpopuler: Option<i64>,
    pub movie_terpopuler: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct FilterKriteria {
    pub studio_id: Option<i64>,
    pub movie_id: Option<i64>,
    pub min_harga: Option<Decimal>,
    pub max_harga: Option<Decimal>,
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

    pub async fn fetch_jadwal_dari_db(&self) -> Result<Vec<Showtime>, sea_orm::DbErr> {
        ShowtimesEntity::find().all(&self.db).await
    }

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

    pub fn filter_jadwal_terdekat_per_film(jadwal_slice: &[Showtime]) -> Vec<(i64, StatusJadwal)> {
        let waktu_sekarang = Local::now();
        
        let jadwal_by_movie: HashMap<i64, Vec<&Showtime>> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.movie_id.map(|mid| (mid, j)))
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
        
        jadwal_by_movie
            .par_iter()
            .filter_map(|(movie_id, jadwal_list)| {
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
    
    pub fn filter_jadwal_terdekat_film_semua_bioskop(
        jadwal_slice: &[Showtime],
        movie_id: i64,
    ) -> Vec<(i64, StatusJadwal)> {
        let waktu_sekarang = Local::now();
        
        let jadwal_by_studio: HashMap<i64, Vec<&Showtime>> = jadwal_slice
            .par_iter()
            .filter(|j| j.movie_id == Some(movie_id))
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
        
        jadwal_by_studio
            .par_iter()
            .filter_map(|(studio_id, jadwal_list)| {
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

    pub fn filter_kompleks(jadwal_slice: &[Showtime], kriteria: &FilterKriteria) -> Vec<Showtime> {
        let waktu_sekarang = Local::now();
        
        jadwal_slice
            .par_iter()
            .filter(|jadwal| {
                if let Some(studio_id) = kriteria.studio_id {
                    if jadwal.studio_id != Some(studio_id) {
                        return false;
                    }
                }

                if let Some(movie_id) = kriteria.movie_id {
                    if jadwal.movie_id != Some(movie_id) {
                        return false;
                    }
                }

                if let Some(min_harga) = kriteria.min_harga {
                    if let Some(price) = jadwal.price {
                        if price < min_harga {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if let Some(max_harga) = kriteria.max_harga {
                    if let Some(price) = jadwal.price {
                        if price > max_harga {
                            return false;
                        }
                    }
                }

                if kriteria.hari_ini_saja {
                    if let Some(start_time) = jadwal.start_time {
                        if start_time.date_naive() != waktu_sekarang.date_naive() {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if kriteria.hanya_mendesak {
                    if let Some(start_time) = jadwal.start_time {
                        let selisih = (start_time - waktu_sekarang).num_minutes();
                        if selisih > 30 || selisih < 0 {
                            return false;
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

    pub fn hitung_statistik(jadwal_slice: &[Showtime]) -> JadwalStatistik {
        let waktu_sekarang = Local::now();
        let hari_ini = waktu_sekarang.date_naive();
        let minggu_ini_start = waktu_sekarang.date_naive() - chrono::Duration::days(7);

        let (
            total_jadwal,
            jadwal_hari_ini,
            jadwal_minggu_ini,
            jadwal_mendesak,
            total_harga,
            count_harga,
        ) = jadwal_slice
            .par_iter()
            .fold(
                || (0usize, 0usize, 0usize, 0usize, Decimal::ZERO, 0usize),
                |(total, hari_ini_count, minggu_ini_count, mendesak, mut harga_sum, harga_count), jadwal| {
                    let new_total = total + 1;
                    let mut new_hari_ini = hari_ini_count;
                    let mut new_minggu_ini = minggu_ini_count;
                    let mut new_mendesak = mendesak;
                    let mut new_harga_count = harga_count;

                    if let Some(start_time) = jadwal.start_time {
                        if start_time.date_naive() == hari_ini {
                            new_hari_ini += 1;
                        }
                        if start_time.date_naive() >= minggu_ini_start {
                            new_minggu_ini += 1;
                        }

                        let selisih = (start_time - waktu_sekarang).num_minutes();
                        if selisih > 0 && selisih <= 30 {
                            new_mendesak += 1;
                        }
                    }

                    if let Some(price) = jadwal.price {
                        harga_sum += price;
                        new_harga_count += 1;
                    }

                    (new_total, new_hari_ini, new_minggu_ini, new_mendesak, harga_sum, new_harga_count)
                },
            )
            .reduce(
                || (0, 0, 0, 0, Decimal::ZERO, 0),
                |(t1, h1, m1, md1, hs1, hc1), (t2, h2, m2, md2, hs2, hc2)| {
                    (t1 + t2, h1 + h2, m1 + m2, md1 + md2, hs1 + hs2, hc1 + hc2)
                },
            );

        let harga_tertinggi = jadwal_slice
            .par_iter()
            .filter_map(|j| j.price)
            .max();

        let harga_terendah = jadwal_slice
            .par_iter()
            .filter_map(|j| j.price)
            .min();

        let studio_terpopuler = Self::studio_terpopuler_parallel(jadwal_slice);
        let movie_terpopuler = Self::movie_terpopuler_parallel(jadwal_slice);

        let harga_rata_rata = if count_harga > 0 {
            (total_harga / Decimal::from(count_harga)).to_f64().unwrap_or(0.0)
        } else {
            0.0
        };

        JadwalStatistik {
            total_jadwal,
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

    fn studio_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        let studio_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.studio_id)
            .fold(
                HashMap::new,
                |mut map, studio_id| {
                    *map.entry(studio_id).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                HashMap::new,
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

    fn movie_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        let movie_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.movie_id)
            .fold(
                HashMap::new,
                |mut map, movie_id| {
                    *map.entry(movie_id).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                HashMap::new,
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

    pub fn filter_by_studio(jadwal_slice: &[Showtime], studio_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.studio_id == Some(studio_id))
            .collect()
    }

    pub fn filter_by_movie(jadwal_slice: &[Showtime], movie_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.movie_id == Some(movie_id))
            .collect()
    }

    pub fn count_jadwal(jadwal_slice: &[Showtime]) -> usize {
        jadwal_slice.len()
    }

    pub fn jadwal_harga_tertinggi(jadwal_slice: &[Showtime]) -> Option<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.price.is_some())
            .max_by_key(|jadwal| jadwal.price.unwrap())
    }

    pub async fn execute_workflow(&self) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let jadwal_slice = jadwal_vec.as_slice();
        let hasil = Self::cari_jadwal_terdekat(jadwal_slice);

        Ok(hasil)
    }

    pub async fn execute_workflow_by_studio(&self, studio_id: i64) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_studio(jadwal_vec.as_slice(), studio_id);
        let owned_jadwal: Vec<Showtime> = filtered.into_iter().cloned().collect();
        let hasil = Self::cari_jadwal_terdekat(owned_jadwal.as_slice());

        Ok(hasil)
    }

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

    pub async fn execute_workflow_kompleks(&self, kriteria: FilterKriteria) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_kompleks(jadwal_vec.as_slice(), &kriteria);
        let hasil = Self::cari_jadwal_terdekat(filtered.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_jadwal_film_semua_bioskop(
        &self,
        movie_id: i64,
    ) -> Result<Vec<(i64, StatusJadwal)>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_film_semua_bioskop(jadwal_vec.as_slice(), movie_id);

        Ok(hasil)
    }

    pub async fn execute_workflow_jadwal_semua_film(&self) -> Result<Vec<(i64, StatusJadwal)>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_per_film(jadwal_vec.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_batch(&self, chunk_size: usize) -> Result<Vec<HasilAnalisa>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil_batch = jadwal_vec
            .as_slice()
            .par_chunks(chunk_size)
            .map(|chunk| Self::cari_jadwal_terdekat(chunk))
            .collect();

        Ok(hasil_batch)
    }

    pub async fn execute_workflow_statistik(&self) -> Result<JadwalStatistik, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let statistik = Self::hitung_statistik(jadwal_vec.as_slice());

        Ok(statistik)
    }
}
