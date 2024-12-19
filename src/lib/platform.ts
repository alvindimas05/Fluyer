import { platform } from "@tauri-apps/plugin-os";

export function isMobile(): boolean {
	return ["android", "ios"].includes(platform());
}

export function isTablet(){
	return window.innerWidth > 768 && isMobile();
}

export function tabletStyle(tabletClass: string, desktopClass: string) {
	return isMobile() ? `md:${tabletClass}` : `lg:${desktopClass}`;
}