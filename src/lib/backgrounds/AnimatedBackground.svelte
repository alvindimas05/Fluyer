<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import { backgroundIsLight, observerCounts } from "$lib/stores/background";
import { musicCurrent } from "$lib/stores/music";
import { prominent } from "color.js";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import { platform } from "@tauri-apps/plugin-os";
import { isMobile } from "$lib/platform";
import BackgroundController from "$lib/controllers/BackgroundController";

const SIZE = 10;

const GRID_COLS = Array.apply(null, Array(SIZE))
	.map(() => "auto")
	.join(" ");

let animatedClasses = $state("hidden");
let lastImage = "";
let position: string[][] = $state([]);
let secondPosition: string[][] = $state([]);

function hexToRgb(hex: string): { r: number; g: number; b: number } {
	const bigint = parseInt(hex.slice(1), 16);
	const r = (bigint >> 16) & 255;
	const g = (bigint >> 8) & 255;
	const b = bigint & 255;
	return { r, g, b };
}

function getLuminance(r: number, g: number, b: number): number {
	const a = [r, g, b].map((v) => {
		v /= 255;
		return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
	});
	return 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];
}

function isMajorityLight(colors: string[]): boolean {
	let lightCount = 0;
	let darkCount = 0;

	for (const hex of colors) {
		const { r, g, b } = hexToRgb(hex);
		const luminance = getLuminance(r, g, b);
		if (luminance > 0.5) {
			lightCount++;
		} else {
			darkCount++;
		}
	}

	return lightCount > darkCount;
}

async function getColors() {
	if (lastImage === MusicController.currentMusicAlbumImage()) return;
	$observerCounts = 0;
	position = [];

	let image = new Image();
	image.src = lastImage = MusicController.currentMusicAlbumImage();

	// @ts-ignore
	let colors: Hex[] = await prominent(image, {
		amount: 10,
		format: "hex",
	});
	BackgroundController.setIsLight(isMajorityLight(colors));

	position = Array.from({ length: SIZE }, () =>
		Array.from(
			{ length: SIZE },
			() => colors[Math.floor(Math.random() * colors.length)],
		),
	);
	secondPosition = Array.from({ length: SIZE }, () =>
		Array.from(
			{ length: SIZE },
			() => colors[Math.floor(Math.random() * colors.length)],
		),
	);

	// FIXME: Visible Animated Colored Squares on Linux
	// Note: Probably won't be fixed soon since it's WebView issue
	if (platform() == "linux") {
		LoadingController.setLoadingBackground(true);
		animatedClasses = "";
	} else {
		animatedClasses = "animate__animated animate__fadeIn";
	}
}
musicCurrent.subscribe(() => !isMobile() && getColors());
</script>

{#if isMobile()}
	<div
		class="fixed z-[-10] w-full h-full animate__animated animate__fadeIn"
		onanimationend={() => LoadingController.setLoadingBackground(true)}
	>
		<div>
			<img
				class="bg-blur-colors object-cover scale-x-[-1]"
				src="/images/mobile-background.png"
				alt="Background"
			/>
		</div>
		<div class="bg-blur-heart">
			<img
				class="bg-blur-colors object-cover"
				src="/images/mobile-background.png"
				alt="Background"
			/>
		</div>
	</div>
{:else}
	<div
		class={`fixed ${animatedClasses}`}
		onanimationend={() => LoadingController.setLoadingBackground(true)}
	>
		<div>
			<div
				class="bg-blur-colors"
				style={`grid-template-columns: ${GRID_COLS}`}
			>
				{#each position as row}
					{#each row as col}
						<div
							class="bg-blur-pixel"
							style={`background: ${col}`}
						></div>
					{/each}
				{/each}
			</div>
		</div>
		<div>
			<div
				class="bg-blur-colors bg-blur-heart"
				style={`grid-template-columns: ${GRID_COLS}`}
			>
				{#each secondPosition as row}
					{#each row as col}
						<div
							class="bg-blur-pixel"
							style={`background: ${col}`}
						></div>
					{/each}
				{/each}
			</div>
		</div>
		<div class="bg-blur"></div>
	</div>
{/if}
{#if $backgroundIsLight}
	<div class="bg-blur-dark"></div>
{/if}
