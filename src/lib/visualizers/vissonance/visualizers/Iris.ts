import * as THREE from "three";
import View from "$lib/visualizers/vissonance/View";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import Spectrum from "$lib/visualizers/vissonance/Spectrum";
import Visualizer from "$lib/visualizers/vissonance/visualizers/Visualizer";

class Iris extends Visualizer {
	name = 'Iris';
	vertexShader = [
		"varying vec4 pos;",
		"void main() {",
		"pos = modelViewMatrix * vec4( position, 1.0 );",
		"gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
		"}",
	].join("\n");
	fragmentShader = [
		"uniform vec3 col;",
		"varying vec4 pos;",
		"void main() {",
		"gl_FragColor = vec4( -pos.z/180.0 * col.r, -pos.z/180.0 * col.g, -pos.z/180.0 * col.b, 1.0 );",
		"}",
	].join("\n");

	async make() {
		await super.make();

		View.usePerspectiveCamera();
		View.data.camera.position.y = 0;
		View.data.camera.position.z = 250;

		for (let i = 0; i < this.numBars / 2; i++) {
			const uniforms = {
				col: { type: "c", value: new THREE.Color("hsl(240, 100%, 50%)") },
			};
			const material = new THREE.ShaderMaterial({
				uniforms: uniforms,
				vertexShader: this.vertexShader,
				fragmentShader: this.fragmentShader,
			});

			let geometry = new THREE.PlaneGeometry(3, 500, 1);
			geometry.rotateX(Math.PI / 1.8);
			geometry.applyMatrix4(new THREE.Matrix4().makeTranslation(0, 60, 0));
			let plane = new THREE.Mesh(geometry, material);

			plane.rotation.z =
				i * ((Math.PI * 2) / this.numBars) + Math.PI / this.numBars;

			this.group.add(plane);

			geometry = new THREE.PlaneGeometry(3, 500, 1);
			geometry.rotateX(Math.PI / 1.8);
			geometry.applyMatrix4(new THREE.Matrix4().makeTranslation(0, 60, 0));
			plane = new THREE.Mesh(geometry, material);

			plane.rotation.z =
				-i * ((Math.PI * 2) / this.numBars) - Math.PI / this.numBars;

			this.group.add(plane);
		}
		View.data.scene.add(this.group);

		await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.group);
	}
	render() {
		AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
		let loudness = this.getLoudness(this.dataArray);
		const visualArray = Spectrum.getVisualBins(
			this.dataArray,
			this.numBars,
			4,
			1300,
		);
		if (!this.group) return;
		for (let i = 0; i < visualArray.length / 2; i++) {
			// Left and right share the same material hence why we don't need i * 2 + 1
			this.setUniformColor(i * 2, loudness);

			this.group.children[i * 2].geometry.attributes.position.array[7] =
				visualArray[i] / 2 + (65 + loudness / 1.5);
			this.group.children[i * 2].geometry.attributes.position.array[10] =
				visualArray[i] / 2 + (65 + loudness / 1.5);
			this.group.children[i * 2].geometry.attributes.position.needsUpdate =
				true;

			this.group.children[i * 2 + 1].geometry.attributes.position.array[7] =
				visualArray[i] / 2 + (65 + loudness / 1.5);
			this.group.children[i * 2 + 1].geometry.attributes.position.array[10] =
				visualArray[i] / 2 + (65 + loudness / 1.5);
			this.group.children[i * 2 + 1].geometry.attributes.position.needsUpdate =
				true;
		}
	}
}

export default Iris;
