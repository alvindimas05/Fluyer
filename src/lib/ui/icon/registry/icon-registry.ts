import { IconType, IconThemeType } from '$lib/ui/icon/types';
import type { Component } from 'svelte';
import iconRegistryPhospor from '$lib/ui/icon/registry/icon-registry-phospor';
import iconRegistryLucide from '$lib/ui/icon/registry/icon-registry-lucide';
import iconRegistryMaterial from '$lib/ui/icon/registry/icon-registry-material';

export const iconRegistry: Record<IconThemeType, Partial<Record<IconType, Component>>> = {
	[IconThemeType.Phosphor]: iconRegistryPhospor,
	[IconThemeType.Lucide]: iconRegistryLucide,
	[IconThemeType.Material]: iconRegistryMaterial
};
