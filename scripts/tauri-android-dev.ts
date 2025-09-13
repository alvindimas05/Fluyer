import {spawn} from "promisify-child-process";

await spawn("bun", ["android:configure"], { stdio: "inherit" });
spawn("bun", ["tauri", "android", "dev"], { stdio: "inherit" });