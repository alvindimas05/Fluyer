import { spawn } from "promisify-child-process";
import { configure, env } from "./tauri-configure";

await configure();
spawn("bun", ["tauri", "build"], { env, stdio: "inherit" });