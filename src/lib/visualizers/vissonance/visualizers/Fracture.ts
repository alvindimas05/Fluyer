import Visualizer from "$lib/visualizers/vissonance/visualizers/Visualizer";
import * as THREE from "three";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import View from "$lib/visualizers/vissonance/View";
import Spectrum from "$lib/visualizers/vissonance/Spectrum";

class Fracture extends Visualizer {
	// @ts-ignore
	group2: THREE.Object3D;
	vertexShader = [
		"varying vec4 pos;",
		"void main() {",
		"pos = modelViewMatrix * vec4( position, 1.0 );",
		"gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
		"}",
	].join("\n");
	fragmentShader = [
		"uniform vec3 col;",
		"varying vec4 pos;",
		"void main() {",
		"gl_FragColor = vec4( -pos.z/500.0 * col.r, -pos.z/500.0 * col.g, -pos.z/500.0 * col.b, 1 );",
		"}",
	].join("\n");

	constructor() {
		super();
		AudioAnalyser.data.analyser.fftSize = 4096;
	}

	async make() {
		View.data.renderer.autoClearColor = true;
		View.data.renderer.setClearColor(new THREE.Color("hsl( 0, 0%, 100%)"), 1);

		View.usePerspectiveCamera();
		View.data.camera.position.y = 0;
		View.data.camera.position.z = 0;

		let positionZ = 10;

		for (let i = 0; i < this.numBands; i++) {
			const geometry = new THREE.PlaneGeometry(
				this.barLen * 2,
				10,
				this.numBars * 2 - 1,
			);
			let uniforms = {
				col: { type: "c", value: new THREE.Color("hsl(240, 100%, 50%)") },
			};
			let material = new THREE.ShaderMaterial({
				uniforms: uniforms,
				vertexShader: this.vertexShader,
				fragmentShader: this.fragmentShader,
			});
			//material = new THREE.MeshBasicMaterial({color:"red",wireframe:true});
			let plane = new THREE.Mesh(geometry, material);
			plane.rotation.x = -Math.PI / 2;
			plane.position.y = -10;
			plane.position.z = positionZ;

			positionZ -= this.barGap;

			this.group.add(plane);
		}

		View.data.scene.add(this.group);

		// @ts-ignore
		this.group2 = this.group.clone();
		this.group2.rotation.z = Math.PI;
		View.data.scene.add(this.group2);

		await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.group);
		View.data.scene.remove(this.group2);
		View.data.renderer.autoClearColor = false;
	}
	render() {
		AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
		const visualArray = Spectrum.getVisualBins(
			this.dataArray,
			this.numBars,
			-200,
			1300,
		);
		let loudness = this.getLoudness(this.dataArray);
		const v = this.numBars * 3 * 2 * 2;
		//smooth loudness
		loudness = (loudness + this.lastLoudness) / 2;
		//view.renderer.setClearColor( new THREE.Color( 'hsl( ' + modn( 250 - (loudness*2.2), 360 ) + ', 100%, 15%)' ), 1);
		View.data.camera.rotation.z -=
			loudness <= 1 ? 0 : (Math.pow(loudness / 8192 + 1, 2) - 1) / 2;
		if (!this.group) return;
		this.group.position.y =
			loudness <= 1 ? -10 : -10 + Math.min((loudness / 255) * 20, 9.8);
		this.group2.position.y =
			loudness <= 1 ? 10 : 10 - Math.min((loudness / 255) * 20, 9.8);
		for (var c = 0; c < this.group.children.length; c++) {
			this.group.children[c].position.z +=
				loudness <= 1
					? 0
					: (Math.pow(loudness / 8192 + 1, 2) - 1) * loudness * 1.7;
			this.group2.children[c].position.z +=
				loudness <= 1
					? 0
					: (Math.pow(loudness / 8192 + 1, 2) - 1) * loudness * 1.7;
			//Put plane back when out of sight
			if (this.group.children[c].position.z > 20) {
				this.group.children[c].position.z -= this.numBands * this.barGap;
				this.group2.children[c].position.z -= this.numBands * this.barGap;
			}
			for (let i = 0; i < visualArray.length; i++) {
				this.setUniformColor(c, loudness);
				if (c % 2 != 0) {
					this.group.children[c].geometry.attributes.position.array[i * 3 + 2] =
						visualArray[i] / 10;
					//group.children[c].geometry.attributes.position.array[i*3 + 2 + (numBars*6)] = visualArray[i] / 10;
					this.group.children[c].geometry.attributes.position.needsUpdate =
						true;
				} else {
					this.group.children[c].geometry.attributes.position.array[
						v / 2 - 3 - (this.numBars * 3 + i * 3) + 2 + this.numBars * 3
					] = visualArray[i] / 10;
					//group.children[c].geometry.attributes.position.array[( v - ( (i+numBars) * 3 ) ) + 2] = visualArray[i] / 20;
					this.group.children[c].geometry.attributes.position.needsUpdate =
						true;
				}
			}
		}
		this.lastLoudness = loudness;
	}
}

export default Fracture;
