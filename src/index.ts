import "dotenv/config";
import "module-alias/register";

import cors from "cors";
import helmet from "helmet";
import morgan from "morgan";
import express, { Request, Response } from "express";
import { StatusCodes } from "http-status-codes";

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

app.get(`/api/games`, (_req: Request, res: Response) =>
{
    console.log("games");
    res.send('games');
});

app.post(`/api/usergames`, (req: Request, res: Response) =>
{
    console.log(req.body);
    res.send('User games received');
});

app.listen(process.env.PORT, () =>
{
    console.log(`Now listening on port: ${process.env.PORT}`);
});
