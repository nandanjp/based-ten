import "dotenv/config";
import "module-alias/register";

import cors from "cors";
import helmet from "helmet";
import morgan from "morgan";
import express, { Request, Response } from "express";
import { StatusCodes } from "http-status-codes";
import prisma from "../lib/db";

const app = express();

app.use(cors());
app.use(helmet());
app.use(morgan("dev"));

app.use(`/api/health`, (_req: Request, res: Response) => {
  console.log("Server is Healthy");
  res.status(StatusCodes.OK).json({
    status: StatusCodes.OK,
    message: "server is healthy and running!",
  });
});

app.get(`/api/manga`, async (_req: Request, res: Response) => {
  const manga = await prisma.manga.findMany();
  if (!manga) {
    res.status(StatusCodes.BAD_REQUEST).json({
      status: StatusCodes.BAD_REQUEST,
      manga,
      message: "Failed to retrieve all manga currently in the database",
    });
    return;
  }
  res.status(StatusCodes.OK).json({
    status: StatusCodes.OK,
    manga,
    message: "Received all manga currently in the database",
  });
});

app.get(`/api/anime`, async (_req: Request, res: Response) => {
  const anime = await prisma.anime.findMany();
  if (!anime) {
    res.status(StatusCodes.BAD_REQUEST).json({
      status: StatusCodes.BAD_REQUEST,
      anime,
      message: "Failed to retrieve all manga currently in the database",
    });
    return;
  }
  res.status(StatusCodes.OK).json({
    status: StatusCodes.OK,
    anime,
    message: "Received all manga currently in the database",
  });
});

app.use("/api/videogames", (_req: Request, res: Response) => {
  console.log("Requesting all video games");
  const allVideoGames = prisma.videoGame.findMany();
  res.send(allVideoGames);
});

app.listen(process.env.PORT, () => {
  console.log(`Now listening on port: ${process.env.PORT}`);
});
