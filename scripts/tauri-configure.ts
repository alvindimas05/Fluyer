import path from "path";
import { spawn } from "promisify-child-process";

const newPath = path.resolve("src-tauri", "resources");

export async function configure(){
    await spawn("bun", ["scripts/install-libmpv.ts"], { stdio: "inherit" });
}

export const env = {
    ...process.env,
    PATH: `${process.env.PATH};${newPath}`,
    FLUYER_MPV_SOURCE: newPath,
};