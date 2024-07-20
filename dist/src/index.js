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
require("dotenv/config");
require("module-alias/register");
const cors_1 = __importDefault(require("cors"));
const helmet_1 = __importDefault(require("helmet"));
const morgan_1 = __importDefault(require("morgan"));
const express_1 = __importDefault(require("express"));
const http_status_codes_1 = require("http-status-codes");
const db_1 = __importDefault(require("../lib/db"));
const app = (0, express_1.default)();
app.use((0, cors_1.default)());
app.use((0, helmet_1.default)());
app.use((0, morgan_1.default)("dev"));
app.use(`/api/health`, (_req, res) => {
    console.log("Server is Healthy");
    res.status(http_status_codes_1.StatusCodes.OK).json({
        status: http_status_codes_1.StatusCodes.OK,
        message: "server is healthy and running!",
    });
});
app.get(`/api/manga`, (_req, res) => __awaiter(void 0, void 0, void 0, function* () {
    const manga = yield db_1.default.manga.findMany();
    if (!manga) {
        res.status(http_status_codes_1.StatusCodes.BAD_REQUEST).json({
            status: http_status_codes_1.StatusCodes.BAD_REQUEST,
            manga,
            message: "Failed to retrieve all manga currently in the database",
        });
        return;
    }
    res.status(http_status_codes_1.StatusCodes.OK).json({
        status: http_status_codes_1.StatusCodes.OK,
        manga,
        message: "Received all manga currently in the database",
    });
}));
app.get(`/api/anime`, (_req, res) => __awaiter(void 0, void 0, void 0, function* () {
    const anime = yield db_1.default.anime.findMany();
    if (!anime) {
        res.status(http_status_codes_1.StatusCodes.BAD_REQUEST).json({
            status: http_status_codes_1.StatusCodes.BAD_REQUEST,
            anime,
            message: "Failed to retrieve all manga currently in the database",
        });
        return;
    }
    res.status(http_status_codes_1.StatusCodes.OK).json({
        status: http_status_codes_1.StatusCodes.OK,
        anime,
        message: "Received all manga currently in the database",
    });
}));
app.use("/api/videogames", (_req, res) => {
    console.log("Requesting all video games");
    const allVideoGames = db_1.default.videoGame.findMany();
    res.send(allVideoGames);
});
app.listen(process.env.PORT, () => {
    console.log(`Now listening on port: ${process.env.PORT}`);
});
