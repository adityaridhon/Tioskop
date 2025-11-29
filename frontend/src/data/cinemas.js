// src/data/cinemas.js
// Pastikan poster film ada di: src/assets/

export const cinemas = [
  // ================= E-WALK XXI =================
  {
    slug: 'ewalk-xxi',
    name: 'E-WALK XXI',
    address: 'E-Walk Balikpapan Superblock',
    city: 'Balikpapan',
    image: '/images/cinemas/ewalk-xxi.jpg',
    schedules: [
      {
        dayLabel: 'Hari ini',
        date: 'Hari Ini',
        movies: [
          {
            id: 1,
            title: 'NOW YOU SEE ME: NOW YOU DONT',
            poster: new URL('../assets/film-6.webp', import.meta.url).href,
            genre: 'Crime',
            duration: '1 jam 52 menit',
            director: 'Ruben Fleischer',
            rating: 'R13+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '12:00' },
              { time: '14:10' },
              { time: '16:20' },
              { time: '18:30' },
              { time: '20:40' }
            ]
          },
          {
            id: 2,
            title: 'AGAK LAEN: MENYALA PANTIKU!',
            poster: new URL('../assets/film-2.webp', import.meta.url).href,
            genre: 'Drama, Komedi',
            duration: '1 jam 59 menit',
            director: 'Muhadkly Acho',
            rating: 'R13+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '12:05' },
              { time: '13:05' },
              { time: '14:20' },
              { time: '15:20' },
              { time: '16:35' },
              { time: '17:35' },
              { time: '18:50' },
              { time: '20:05' }
            ]
          },
          {
            id: 3,
            title: 'ZOOTOPIA 2',
            poster: new URL('../assets/film-1.webp', import.meta.url).href,
            genre: 'Animation, Adventure',
            duration: '1 jam 48 menit',
            director: 'Byron Howard',
            rating: 'SU',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '11:00' },
              { time: '13:15' },
              { time: '15:30' },
              { time: '17:45' },
              { time: '20:00' }
            ]
          }
        ]
      }
    ]
  },

  // ================= CGV PLAZA =================
  {
    slug: 'cgv-plaza-balikpapan',
    name: 'CGV Plaza Balikpapan',
    address: 'Plaza Balikpapan, Jl. Jenderal Sudirman',
    city: 'Balikpapan',
    image: '/images/cinemas/cgv-plaza-balikpapan.jpg',
    schedules: [
      {
        dayLabel: 'Hari ini',
        date: 'Hari Ini',
        movies: [
          {
            id: 4,
            title: 'PANGKU',
            poster: new URL('../assets/film-2.webp', import.meta.url).href,
            genre: 'Horror, Mystery',
            duration: '1 jam 45 menit',
            director: 'Rikky Satria',
            rating: 'D17+',
            format: '2D',
            price: 'Rp40.000',
            showtimes: [
              { time: '11:30' },
              { time: '13:40' },
              { time: '15:50' },
              { time: '18:00' },
              { time: '20:10' }
            ]
          },
          {
            id: 5,
            title: 'WICKED: FOR GOOD',
            poster: new URL('../assets/film-5.webp', import.meta.url).href,
            genre: 'Musical, Fantasy',
            duration: '2 jam 10 menit',
            director: 'Jon M. Chu',
            rating: 'R13+',
            format: '2D',
            price: 'Rp55.000',
            showtimes: [
              { time: '12:15' },
              { time: '15:20' },
              { time: '18:25' },
              { time: '21:30' }
            ]
          },
          {
            id: 6,
            title: 'KEEPER',
            poster: new URL('../assets/film-8.webp', import.meta.url).href,
            genre: 'Drama, Sports',
            duration: '1 jam 50 menit',
            director: 'Aditya Rahman',
            rating: 'R13+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '13:00' },
              { time: '15:30' },
              { time: '18:00' },
              { time: '20:30' }
            ]
          }
        ]
      }
    ]
  },

  // ================= CINEPOLIS =================
  {
    slug: 'cinepolis-living-plaza-balikpapan',
    name: 'Cinepolis Living Plaza Balikpapan',
    address: 'Living Plaza Balikpapan, Jl. MT Haryono',
    city: 'Balikpapan',
    image: '/images/cinemas/cinepolis-living-plaza.jpg',
    schedules: [
      {
        dayLabel: 'Hari ini',
        date: 'Hari Ini',
        movies: [
          {
            id: 7,
            title: 'DANYANG WINGIT JUMAT KLIWON',
            poster: new URL('../assets/film-4.webp', import.meta.url).href,
            genre: 'Horror, Mystery',
            duration: '1 jam 42 menit',
            director: 'Lala Pradipta',
            rating: 'D17+',
            format: '2D',
            price: 'Rp40.000',
            showtimes: [
              { time: '12:00' },
              { time: '14:10' },
              { time: '16:20' },
              { time: '18:30' },
              { time: '20:40' }
            ]
          },
          {
            id: 8,
            title: 'DOPAMIN',
            poster: new URL('../assets/film-3.webp', import.meta.url).href,
            genre: 'Thriller, Mystery',
            duration: '2 jam 05 menit',
            director: 'Naya Hartono',
            rating: 'R13+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '13:00' },
              { time: '15:45' },
              { time: '18:30' },
              { time: '21:15' }
            ]
          }
        ]
      }
    ]
  },

  // ================= PENTACITY XXI =================
  {
    slug: 'pentacity-xxi',
    name: 'PENTACITY XXI',
    address: 'Pentacity Mall Balikpapan',
    city: 'Balikpapan',
    image: '/images/cinemas/pentacity-xxi.jpg',
    schedules: [
      {
        dayLabel: 'Hari ini',
        date: 'Hari Ini',
        movies: [
          {
            id: 9,
            title: 'THE RUNNING MAN',
            poster: new URL('../assets/film-7.webp', import.meta.url).href,
            genre: 'Action, Sci-Fi',
            duration: '2 jam 00 menit',
            director: 'Jacob Lin',
            rating: 'D17+',
            format: '2D',
            price: 'Rp50.000',
            showtimes: [
              { time: '13:00' },
              { time: '15:30' },
              { time: '18:00' },
              { time: '20:30' }
            ]
          },
          {
            id: 10,
            title: 'PANGKU',
            poster: new URL('../assets/film-2.webp', import.meta.url).href,
            genre: 'Horror',
            duration: '1 jam 45 menit',
            director: 'Rikky Satria',
            rating: 'D17+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '12:10' },
              { time: '14:20' },
              { time: '16:30' },
              { time: '18:40' },
              { time: '20:50' }
            ]
          }
        ]
      }
    ]
  },

  // ================= STUDIO XXI =================
  {
    slug: 'studio-xxi',
    name: 'STUDIO XXI',
    address: 'Jl. Jenderal Sudirman',
    city: 'Balikpapan',
    image: '/images/cinemas/studio-xxi.jpg',
    schedules: [
      {
        dayLabel: 'Hari ini',
        date: 'Hari Ini',
        movies: [
          {
            id: 11,
            title: 'ZOOTOPIA 2',
            poster: new URL('../assets/film-1.webp', import.meta.url).href,
            genre: 'Animation, Adventure',
            duration: '1 jam 48 menit',
            director: 'Byron Howard',
            rating: 'SU',
            format: '2D',
            price: 'Rp40.000',
            showtimes: [
              { time: '10:45' },
              { time: '13:00' },
              { time: '15:15' },
              { time: '17:30' },
              { time: '19:45' }
            ]
          },
          {
            id: 12,
            title: 'DOPAMIN',
            poster: new URL('../assets/film-3.webp', import.meta.url).href,
            genre: 'Thriller, Mystery',
            duration: '2 jam 05 menit',
            director: 'Naya Hartono',
            rating: 'R13+',
            format: '2D',
            price: 'Rp45.000',
            showtimes: [
              { time: '12:20' },
              { time: '15:00' },
              { time: '17:40' },
              { time: '20:20' }
            ]
          }
        ]
      }
    ]
  }
]

export function getCinemaBySlug(slug) {
  return cinemas.find((c) => c.slug === slug)
}
