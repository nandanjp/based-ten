import { log } from "console";
import prisma from "../lib/db";
import { readFileSync, createReadStream } from "fs";
import { join } from "path";
import { Genre } from "@prisma/client";
import { parse } from "csv-parse";

interface Anime {
  id: number;
  name: string;
  score: number;
  genres: Genre[];
  english_name: string;
  japanese_name: string;
  synopsis: string;
  type: string;
  episodes: number;
  aired: string;
  premiered: string;
  producers: string;
  licensors: string[];
  studios: string;
  source: string;
  duration: string;
  rating: string;
  ranked: number;
  popularity: number;
}

interface Manga {
  name: string;
  date_released: string;
  depth: number;
  download_timeout: number;
  download_slot: string;
  download_latency: number;
  link: string;
  genres: Genre[];
  status: string;
  rating: number;
  img_link: string;
}

async function readAnimeCSV(filePath: string): Promise<Anime[]> {
  const anime: Anime[] = [];
  return await new Promise((resolve, reject) => {
    createReadStream(filePath)
      .pipe(parse({ delimiter: ",", from_line: 2 }))
      .on(
        "data",
        ([
          id,
          name,
          score,
          genres,
          english_name,
          japanese_name,
          synopsis,
          type,
          episodes,
          aired,
          premiered,
          producers,
          licensors,
          studios,
          source,
          duration,
          rating,
          ranked,
          popularity,
        ]: string[]) => {
          anime.push({
            id: !Number.isNaN(parseInt(id)) ? parseInt(id) : -1,
            name,
            score: !Number.isNaN(parseFloat(score)) ? parseFloat(score) : -1,
            genres: genres
              .split(",")
              .filter((genre) => genre.toLowerCase() !== "ecchi")
              .map((s) => s.toLowerCase().split(" ").join("_"))
              .map((s) => Genre[s as keyof typeof Genre])
              .filter((genre) => genre),
            english_name,
            japanese_name,
            synopsis,
            type,
            episodes: !Number.isNaN(parseInt(episodes))
              ? parseInt(episodes)
              : -1,
            aired,
            premiered,
            producers,
            licensors: licensors.split(","),
            studios,
            source,
            duration,
            rating,
            ranked: !Number.isNaN(parseInt(ranked)) ? parseInt(ranked) : -1,
            popularity: !Number.isNaN(parseInt(popularity))
              ? parseInt(popularity)
              : -1,
          });
        }
      )
      .on("error", (err) => {
        console.error(`Failed to parse the CSV: ${err}`);
        reject();
      })
      .on("finish", () => {
        console.log("Finished parsing the CSV file!");
        resolve(anime.slice(0, 100));
      });
  });
}

async function readMangaCSV(filePath: string): Promise<Manga[]> {
  const manga: Manga[] = [];
  return await new Promise((resolve, reject) => {
    createReadStream(filePath)
      .pipe(parse({ delimiter: ",", from_line: 2 }))
      .on(
        "data",
        ([
          name,
          date_released,
          depth,
          download_timeout,
          download_slot,
          download_latency,
          link,
          genres,
          status,
          rating,
          img_link,
        ]: string[]) => {
          manga.push({
            name,
            date_released,
            depth: !Number.isNaN(parseInt(depth)) ? parseInt(depth) : -1,
            download_timeout: !Number.isNaN(parseFloat(download_timeout))
              ? parseFloat(download_timeout)
              : -1,
            download_slot,
            download_latency: !Number.isNaN(parseFloat(download_latency))
              ? parseFloat(download_latency)
              : -1,
            link,
            genres: genres
              .split(",")
              .filter((genre) => genre.toLowerCase() !== "ecchi")
              .map((s) => s.toLowerCase().split(" ").join("_"))
              .map((s) => Genre[s as keyof typeof Genre])
              .filter((genre) => genre),
            status,
            rating: !Number.isNaN(parseFloat(rating)) ? parseFloat(rating) : -1,
            img_link,
          });
        }
      )
      .on("error", (err) => {
        console.error(`failed to parse the CSV file: ${err}`);
        reject();
      })
      .on("end", function () {
        console.log("finished parsing the CSV file");
        resolve(manga.slice(0, 100));
      });
  });
}

async function main() {
  const anime = await readAnimeCSV(
    join(__dirname, "..", "datasets", "anime-filtered.csv")
  );
  const manga = await readMangaCSV(
    join(__dirname, "..", "datasets", "manga.csv")
  );

  await prisma.anime.createMany({
    data: anime.map(
      ({
        id,
        name,
        score,
        genres,
        english_name,
        japanese_name,
        synopsis,
        type,
        episodes,
        aired,
        premiered,
        producers,
        licensors,
        studios,
        source,
        duration,
        rating,
        ranked,
        popularity,
      }) => ({
        anime_id: id,
        name,
        score,
        genres,
        english_name,
        japanese_name,
        synopsis,
        type,
        aired,
        premiered,
        producers,
        licensors,
        studios,
        source,
        duration,
        rating,
        ranked,
        popularity,
        episodes,
      })
    ),
    skipDuplicates: true,
  });

  await prisma.manga.createMany({
    data: manga.map(
      ({
        name,
        date_released,
        depth,
        download_timeout,
        download_slot,
        download_latency,
        link,
        genres,
        status,
        rating,
        img_link,
      }) => ({
        name,
        date_released,
        depth,
        download_timeout,
        download_slot,
        download_latency,
        link,
        genres,
        status,
        rating,
        img_link,
      })
    ),
    skipDuplicates: true,
  });
}
main()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (e) => {
    console.error(e);
    await prisma.$disconnect();
    process.exit(1);
  });
