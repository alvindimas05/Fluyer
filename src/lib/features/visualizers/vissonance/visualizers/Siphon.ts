import * as THREE from 'three';
import View from '$lib/features/visualizers/vissonance/View';
import AudioAnalyser from '$lib/features/visualizers/vissonance/AudioAnalyser';
import Spectrum from '$lib/features/visualizers/vissonance/Spectrum';
import Visualizer from '$lib/features/visualizers/vissonance/visualizers/Visualizer';

class Siphon extends Visualizer {
	name = 'Siphon';
	cylRadius = 100;
	vertexShader = [
		'varying vec4 pos;',
		'void main() {',
		'pos = modelViewMatrix * vec4( position, 1.0 );',
		'gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );',
		'}'
	].join('\n');
	fragmentShader = [
		'uniform vec3 col;',
		'varying vec4 pos;',
		'void main() {',
		'gl_FragColor = vec4( col.r, col.g, col.b, 1.0 );',
		'}'
	].join('\n');

	async make() {
		await super.make();

		this.barGap = 8;

		View.usePerspectiveCamera();
		View.data.camera.position.y = 0;
		View.data.camera.position.z = 0;

		View.data.renderer.autoClearColor = true;
		View.data.renderer.setClearColor(new THREE.Color('hsl( 0, 0%, 100%)'), 1);

		var posX = 0;

		for (var i = 0; i < this.numBands; i++) {
			const geometry = new THREE.CylinderGeometry(
				this.cylRadius,
				this.cylRadius,
				this.barLen,
				this.numBars * 2 - 1,
				1,
				true
			);
			var uniforms = {
				col: { type: 'c', value: new THREE.Color('hsl(240, 100%, 50%)') }
			};
			var material = new THREE.ShaderMaterial({
				uniforms: uniforms,
				vertexShader: this.vertexShader,
				fragmentShader: this.fragmentShader,
				side: THREE.BackSide
			});
			let cylinder = new THREE.Mesh(geometry, material);
			cylinder.rotation.z = Math.PI / 2;
			cylinder.rotation.y = Math.PI / 2;
			cylinder.position.z = posX;

			posX -= this.barGap;

			this.group.add(cylinder);
		}
		this.group.rotation.z = -Math.PI / 2;
		View.data.scene.add(this.group);

		await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.group);
		// View.data.renderer.autoClearColor = false;
	}
	render() {
		AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
		const visualArray = Spectrum.getVisualBins(this.dataArray, this.numBars, 4, 1300);
		let loudness = this.getLoudness(this.dataArray);
		//smooth loudness
		loudness = (loudness + this.lastLoudness) / 2;
		if (!this.group) return;

		this.group.scale.x = 1 - Math.min(loudness / 255, 0.8);
		this.group.scale.y = 1 - Math.min(loudness / 255, 0.8);

		let currentBand = 0;
		for (var c = 0; c < this.group.children.length; c++) {
			this.group.children[c].position.z +=
				loudness <= 1 ? 0 : (Math.pow(loudness / 8192 + 1, 2) - 1) * loudness * 4;
			//Put plane back when out of sight
			if (this.group.children[c].position.z > 10) {
				this.group.children[c].position.z -= this.numBands * this.barGap;
				currentBand = c;
			}
		}
		for (var i = 0; i < visualArray.length; i++) {
			this.scaleGroupVectorLength(
				currentBand,
				i * 3 + this.numBars * 6,
				-visualArray[i] / 3.5 - loudness / 7
			);
			this.scaleGroupVectorLength(
				currentBand,
				(this.numBars * 3 * 2 * 2) / 2 -
					3 -
					(this.numBars * 3 + i * 3) +
					this.numBars * 3 +
					this.numBars * 6,
				-visualArray[i] / 3.5 - loudness / 7
			);
			this.group.children[currentBand].geometry.attributes.position.needsUpdate = true;
		}
		this.setUniformColor(currentBand, loudness);
		this.lastLoudness = loudness;
	}
	scaleGroupVectorLength(groupC: number, groupI: number, length: number) {
		var v3 = new THREE.Vector3(
			this.group.children[groupC].geometry.attributes.position.array[groupI + 0],
			this.group.children[groupC].geometry.attributes.position.array[groupI + 1],
			this.group.children[groupC].geometry.attributes.position.array[groupI + 2]
		);
		var scalar = (length + this.cylRadius) / v3.distanceTo(new THREE.Vector3(0, 0, 0));

		this.group.children[groupC].geometry.attributes.position.array[groupI + 0] *= scalar;
		this.group.children[groupC].geometry.attributes.position.array[groupI + 1] *= scalar;
		this.group.children[groupC].geometry.attributes.position.array[groupI + 2] *= scalar;
	}
	setUniformColor(groupI: number, loudness: number) {
		var h = this.modn(250 - loudness * 7, 360);
		this.group.children[groupI].material.uniforms.col.value = new THREE.Color(
			'hsl(' + h + ', 100%, 50%)'
		);
		View.data.renderer.setClearColor(
			new THREE.Color('hsl(' + ((h + 180) % 360) + ', 100%, 97%)'),
			1
		);
	}
}

export default Siphon;
