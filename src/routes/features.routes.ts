import { Router } from "express";
import { getFeatures } from "../services/features/features.handlers";


const router = Router();

router.get("/", getFeatures); //get all features

export default router;