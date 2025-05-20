import { spawn } from "promisify-child-process";
import { configure, env } from "./tauri-configure";

await configure();
spawn("bun", ["run", "build"], { env, stdio: "inherit" });