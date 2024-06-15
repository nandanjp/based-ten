interface Movie {
  id: number
  name: string
  image: string
  created_at: Date
}

export const SeedMovies: Movie[] = [
  {
    id: 1,
    name: 'Oppenheimer',
    image:
      'https://m.media-amazon.com/images/M/MV5BMDBmYTZjNjUtN2M1MS00MTQ2LTk2ODgtNzc2M2QyZGE5NTVjXkEyXkFqcGdeQXVyNzAwMjU2MTY@._V1_.jpg',
    created_at: new Date('2023-07-21')
  },
  {
    id: 2,
    name: 'Barbie',
    image:
      'https://m.media-amazon.com/images/M/MV5BNjU3N2QxNzYtMjk1NC00MTc4LTk1NTQtMmUxNTljM2I0NDA5XkEyXkFqcGdeQXVyODE5NzE3OTE@._V1_.jpg',
    created_at: new Date('2023-07-21')
  },
  {
    id: 3,
    name: 'Top Gun: Maverick',
    image:
      'https://m.media-amazon.com/images/M/MV5BZWYzOGEwNTgtNWU3NS00ZTQ0LWJkODUtMmVhMjIwMjA1ZmQwXkEyXkFqcGdeQXVyMjkwOTAyMDU@._V1_.jpg',
    created_at: new Date('2022-05-27')
  },
  {
    id: 4,
    name: 'Everything Everywhere All at Once',
    image:
      'https://m.media-amazon.com/images/M/MV5BYTdiOTIyZTQtNmQ1OS00NjZlLWIyMTgtYzk5Y2M3ZDVmMDk1XkEyXkFqcGdeQXVyMTAzMDg4NzU0._V1_FMjpg_UX1000_.jpg',
    created_at: new Date('2022-03-11')
  },
  {
    id: 5,
    name: 'Toy Story',
    image:
      'https://m.media-amazon.com/images/M/MV5BMDU2ZWJlMjktMTRhMy00ZTA5LWEzNDgtYmNmZTEwZTViZWJkXkEyXkFqcGdeQXVyNDQ2OTk4MzI@._V1_.jpg',
    created_at: new Date('1995-11-22')
  },
  {
    id: 6,
    name: "Five Nights at Freddy's",
    image:
      'https://upload.wikimedia.org/wikipedia/en/thumb/d/d6/Five_Nights_At_Freddy%27s_poster.jpeg/220px-Five_Nights_At_Freddy%27s_poster.jpeg',
    created_at: new Date('2023-10-27')
  },
  {
    id: 7,
    name: 'Spider-Man: Across the Spider-Verse',
    image: 'https://upload.wikimedia.org/wikipedia/en/b/b4/Spider-Man-_Across_the_Spider-Verse_poster.jpg',
    created_at: new Date('2023-06-02')
  },
  {
    id: 8,
    name: 'Avengers: Endgame',
    image: 'https://upload.wikimedia.org/wikipedia/en/0/0d/Avengers_Endgame_poster.jpg',
    created_at: new Date('2019-04-26')
  },
  {
    id: 9,
    name: 'Forrest Gump',
    image:
      'https://upload.wikimedia.org/wikipedia/en/thumb/6/67/Forrest_Gump_poster.jpg/220px-Forrest_Gump_poster.jpg',
    created_at: new Date('1994-07-06')
  },
  {
    id: 10,
    name: 'Green Book',
    image: 'https://upload.wikimedia.org/wikipedia/en/5/5b/Green_Book_%282018_poster%29.png',
    created_at: new Date('2018-09-11')
  }
]
