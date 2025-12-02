import { platform } from "@tauri-apps/plugin-os";

export function isAndroid(): boolean {
	return platform() == "android";
}

export function isIos(): boolean {
	return platform() == "ios";
}

export function isMobile(): boolean {
	return isAndroid() || isIos();
}

export function isDesktop(): boolean {
	return !isMobile();
}

export function isMacos(): boolean {
	return platform() == "macos";
}

export function isWindows(): boolean {
	return platform() == "windows";
}

export function isLinux(): boolean {
	return platform() == "linux";
}