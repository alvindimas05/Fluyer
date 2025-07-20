import { IconType, IconThemeType } from "$lib/icon/types";
import type { Component } from "svelte";
import iconRegistryPhospor from "$lib/icon/registry/icon-registry-phospor";
import iconRegistryLucide from "$lib/icon/registry/icon-registry-lucide";
import iconRegistryMaterial from "$lib/icon/registry/icon-registry-material";

export const iconRegistry: Record<
	IconThemeType,
	Partial<Record<IconType, Component>>
> = {
	[IconThemeType.Phosphor]: iconRegistryPhospor,
	[IconThemeType.Lucide]: iconRegistryLucide,
	[IconThemeType.Material]: iconRegistryMaterial,
};
