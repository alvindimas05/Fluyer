import * as THREE from 'three';
import AudioAnalyser from '$lib/features/visualizers/vissonance/AudioAnalyser';
import View from '$lib/features/visualizers/vissonance/View';

class Visualizer {
	name = 'Visualizer';
	// @ts-ignore
	group: THREE.Object3D;
	// @ts-ignore
	dataArray: Uint8Array;
	numBars = 128;
	numBands = 64;
	barLen = 150;
	barGap = 10;
	vertexShader = '';
	fragmentShader = '';
	lastLoudness = 0;

	async make() {
		this.group = new THREE.Object3D();
		const bufferLength = AudioAnalyser.data.analyser.frequencyBinCount;
		this.dataArray = new Uint8Array(bufferLength);
		AudioAnalyser.data.analyser.fftSize = 4096;

		View.data.renderer.autoClearColor = true;
		View.data.renderer.setClearColor(new THREE.Color('hsl( 0, 0%, 0%)'), 1);
	}
	destroy() {}
	render() {}
	executeOnNewSong() {}

	waitForLoad() {
		const manager = new THREE.LoadingManager();
		const onLoad = new Promise<void>((resolve) => {
			manager.onLoad = () => {
				resolve();
			};
		});

		const textureLoader = new THREE.TextureLoader(manager);
		try {
			textureLoader.load('/textures/placeholder.jpg');
		} catch (e) {}
		return onLoad;
	}

	setUniformColor(groupI: number, loudness: number) {
		let h = this.modn(250 - loudness * 2.2, 360);
		this.group.children[groupI].material.uniforms.col.value = new THREE.Color(
			`hsl(${h}, 100%, 50%)`
		);
	}

	getLoudness(arr: Uint8Array) {
		let sum = 0;
		for (let i = 0; i < arr.length; i++) {
			sum += arr[i];
		}
		return sum / arr.length;
	}

	modn(n: number, m: number) {
		return ((n % m) + m) % m;
	}
}

export default Visualizer;
