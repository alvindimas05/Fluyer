import * as THREE from "three";
import View from "$lib/visualizers/vissonance/View";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import Spectrum from "$lib/visualizers/vissonance/Spectrum";
import Visualizer from "$lib/visualizers/vissonance/visualizers/Visualizer";

class HillFog extends Visualizer {
	name = "Hill Fog";
	// @ts-ignore
	geometry: THREE.PlaneGeometry;
	// @ts-ignore
	plane: THREE.Mesh;
	vertexShader = [
		"void main() {",
		"gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
		"}",
	].join("\n");
	fragmentShader = [
		"void main() {",
		"gl_FragColor = vec4( gl_FragCoord.y/500.0, 0, gl_FragCoord.y/1000.0, 1.0 );",
		"}",
	].join("\n");

	async make() {
		await super.make();

		View.useOrthographicCamera();

		this.geometry = new THREE.PlaneGeometry(900, 40, 127);
		const material = new THREE.ShaderMaterial({
			vertexShader: this.vertexShader,
			fragmentShader: this.fragmentShader,
		});
		this.plane = new THREE.Mesh(this.geometry, material);
		View.data.scene.add(this.plane);

		// await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.plane);
	}
	render() {
		AudioAnalyser.data.analyser.fftSize = 256;
		const bufferLength = AudioAnalyser.data.analyser.frequencyBinCount;
		const dataArray = new Uint8Array(bufferLength);
		AudioAnalyser.data.analyser.getByteTimeDomainData(dataArray);

		if (!this.geometry) return;
		for (let i = 0; i < bufferLength; i++) {
			this.geometry.attributes.position.array[i * 3 + 1] = dataArray[i] / 3;
		}
		this.geometry.attributes.position.needsUpdate = true;
	}
}

export default HillFog;
