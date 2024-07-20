"use strict";
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
app.listen(process.env.PORT, () => {
    console.log(`Now listening on port: ${process.env.PORT}`);
});
