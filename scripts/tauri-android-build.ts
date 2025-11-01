import { spawn } from "promisify-child-process";

await spawn("bun", ["android:configure"], { stdio: "inherit" });
spawn("bun", ["tauri", "android", "build", "-v", "--target", "aarch64"], {
	stdio: "inherit",
});
