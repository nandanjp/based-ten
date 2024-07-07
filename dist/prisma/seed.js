"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const db_1 = __importDefault(require("../lib/db"));
const fs_1 = require("fs");
const path_1 = require("path");
const client_1 = require("@prisma/client");
const csv_parse_1 = require("csv-parse");
function readAnimeCSV(filePath) {
    return __awaiter(this, void 0, void 0, function* () {
        const anime = [];
        return yield new Promise((resolve, reject) => {
            (0, fs_1.createReadStream)(filePath)
                .pipe((0, csv_parse_1.parse)({ delimiter: ",", from_line: 2 }))
                .on("data", ([id, name, score, genres, english_name, japanese_name, synopsis, type, episodes, aired, premiered, producers, licensors, studios, source, duration, rating, ranked, popularity,]) => {
                anime.push({
                    id: !Number.isNaN(parseInt(id)) ? parseInt(id) : -1,
                    name,
                    score: !Number.isNaN(parseFloat(score)) ? parseFloat(score) : -1,
                    genres: genres
                        .split(",")
                        .filter((genre) => genre.toLowerCase() !== "ecchi")
                        .map((s) => s.toLowerCase().split(" ").join("_"))
                        .map((s) => client_1.Genre[s])
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
            })
                .on("error", (err) => {
                console.error(`Failed to parse the CSV: ${err}`);
                reject();
            })
                .on("finish", () => {
                console.log("Finished parsing the CSV file!");
                resolve(anime.slice(0, 100));
            });
        });
    });
}
function readMangaCSV(filePath) {
    return __awaiter(this, void 0, void 0, function* () {
        const manga = [];
        return yield new Promise((resolve, reject) => {
            (0, fs_1.createReadStream)(filePath)
                .pipe((0, csv_parse_1.parse)({ delimiter: ",", from_line: 2 }))
                .on("data", ([name, date_released, depth, download_timeout, download_slot, download_latency, link, genres, status, rating, img_link,]) => {
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
                        .map((s) => client_1.Genre[s])
                        .filter((genre) => genre),
                    status,
                    rating: !Number.isNaN(parseFloat(rating)) ? parseFloat(rating) : -1,
                    img_link,
                });
            })
                .on("error", (err) => {
                console.error(`failed to parse the CSV file: ${err}`);
                reject();
            })
                .on("end", function () {
                console.log("finished parsing the CSV file");
                resolve(manga.slice(0, 100));
            });
        });
    });
}
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const anime = yield readAnimeCSV((0, path_1.join)(__dirname, "..", "datasets", "anime-filtered.csv"));
        const manga = yield readMangaCSV((0, path_1.join)(__dirname, "..", "datasets", "manga.csv"));
        yield db_1.default.anime.createMany({
            data: anime.map(({ id, name, score, genres, english_name, japanese_name, synopsis, type, episodes, aired, premiered, producers, licensors, studios, source, duration, rating, ranked, popularity, }) => ({
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
            })),
            skipDuplicates: true,
        });
        yield db_1.default.manga.createMany({
            data: manga.map(({ name, date_released, depth, download_timeout, download_slot, download_latency, link, genres, status, rating, img_link, }) => ({
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
            })),
            skipDuplicates: true,
        });
    });
}
main()
    .then(() => __awaiter(void 0, void 0, void 0, function* () {
    yield db_1.default.$disconnect();
}))
    .catch((e) => __awaiter(void 0, void 0, void 0, function* () {
    console.error(e);
    yield db_1.default.$disconnect();
    process.exit(1);
}));
