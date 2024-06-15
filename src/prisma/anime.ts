interface Anime {
  anime_id: number
  name: string
  image: string
  studio: string
  created_at: Date
  episodes: number
}

export const SeedAnime: Anime[] = [
  {
    anime_id: 1,
    name: 'Naruto',
    studio: 'Pierrot',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/94/NarutoCoverTankobon1.jpg',
    created_at: new Date('2002-10-03'),
    episodes: 220
  },
  {
    anime_id: 2,
    name: 'Attack on Titan',
    studio: 'Wit Studio',
    image: 'https://upload.wikimedia.org/wikipedia/en/6/68/Attack_on_Titan_cover.jpg',
    created_at: new Date('2013-04-07'),
    episodes: 87
  },
  {
    anime_id: 3,
    name: 'One Piece',
    studio: 'Toei Animation',
    image: 'https://upload.wikimedia.org/wikipedia/en/4/4b/OnePieceVolume61Cover.jpg',
    created_at: new Date('1999-10-20'),
    episodes: 1061
  },
  {
    anime_id: 4,
    name: 'Death Note',
    studio: 'Madhouse',
    image: 'https://upload.wikimedia.org/wikipedia/en/6/6f/Death_Note_Vol_1.jpg',
    created_at: new Date('2006-10-03'),
    episodes: 37
  },
  {
    anime_id: 5,
    name: 'Fullmetal Alchemist',
    studio: 'Bones',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/99/Fullmetal_Alchemist_-_Volume_1.jpg',
    created_at: new Date('2003-10-04'),
    episodes: 51
  },
  {
    anime_id: 6,
    name: 'Dragon Ball Z',
    studio: 'Toei Animation',
    image: 'https://upload.wikimedia.org/wikipedia/en/7/7d/Dragon_Ball_Z_vol._1_DVD_cover.jpg',
    created_at: new Date('1989-04-26'),
    episodes: 291
  },
  {
    anime_id: 7,
    name: 'My Hero Academia',
    studio: 'Bones',
    image: 'https://upload.wikimedia.org/wikipedia/en/a/af/My_Hero_Academia,_Volume_1.jpg',
    created_at: new Date('2016-04-03'),
    episodes: 88
  },
  {
    anime_id: 8,
    name: 'Demon Slayer',
    studio: 'Ufotable',
    image: 'https://upload.wikimedia.org/wikipedia/en/2/22/Demon_Slayer_-_Kimetsu_no_Yaiba%2C_volume_1.jpg',
    created_at: new Date('2019-04-06'),
    episodes: 44
  },
  {
    anime_id: 9,
    name: 'Sword Art Online',
    studio: 'A-1 Pictures',
    image: 'https://upload.wikimedia.org/wikipedia/en/b/b2/Sword_Art_Online_light_novel_volume_1_cover.jpg',
    created_at: new Date('2012-07-08'),
    episodes: 96
  },
  {
    anime_id: 10,
    name: 'One Punch Man',
    studio: 'Madhouse',
    image: 'https://upload.wikimedia.org/wikipedia/en/7/72/OnePunchMan_manga_cover.png',
    created_at: new Date('2015-10-04'),
    episodes: 24
  }
]
