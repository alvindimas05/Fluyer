import * as THREE from 'three';
import View from '$lib/features/visualizers/vissonance/View';
import AudioAnalyser from '$lib/features/visualizers/vissonance/AudioAnalyser';
import Spectrum from '$lib/features/visualizers/vissonance/Spectrum';
import Visualizer from '$lib/features/visualizers/vissonance/visualizers/Visualizer';

class Tricentric extends Visualizer {
	name = 'Tricentric';
	vertexShader = [
		'void main() {',
		'gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );',
		'}'
	].join('\n');
	fragmentShader = [
		'uniform vec3 col;',
		'uniform float alpha;',
		'void main() {',
		'gl_FragColor = vec4( col.r, col.g, col.b, alpha );',
		'}'
	].join('\n');

	async make() {
		await super.make();

		this.numBars = 32;

		View.usePerspectiveCamera();
		View.data.camera.position.y = 0;

		var positionZ = 498;

		for (var i = 0; i < this.numBars; i++) {
			const geometry = new THREE.CylinderGeometry(20, 20, 2, 3, 1, true);
			var uniforms = {
				col: { type: 'c', value: new THREE.Color('hsl(250, 100%, 70%)') },
				alpha: { type: 'f', value: 1 }
			};
			var material = new THREE.ShaderMaterial({
				uniforms: uniforms,
				vertexShader: this.vertexShader,
				fragmentShader: this.fragmentShader,
				side: THREE.DoubleSide
			});
			let cylinder = new THREE.Mesh(geometry, material);
			cylinder.position.z = positionZ;
			cylinder.rotation.x = Math.PI / 2;

			positionZ -= 5;

			this.group.add(cylinder);
		}
		View.data.scene.add(this.group);

		await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.group);
		View.data.camera.rotation.z = 0;
	}
	render() {
		AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
		const visualArray = Spectrum.getVisualBins(this.dataArray, 32, 0, 1300);
		var avg = this.arrayAverage(visualArray);
		View.data.camera.rotation.z += avg <= 1 ? 0 : Math.pow(avg / 8192 + 1, 2) - 1;

		if (!this.group) return;
		for (var i = 0; i < visualArray.length; i++) {
			this.setTricentricUniformColor(
				i,
				308 - visualArray[i],
				parseInt((avg / 255) * 40) + 60,
				parseInt((visualArray[i] / 255) * 25) + 45,
				visualArray[i]
			);
			this.group.children[i].scale.x = (visualArray[i] / 255) * (avg / 255) + 0.25;
			this.group.children[i].scale.y = (visualArray[i] / 255) * (avg / 255) + 0.25;
			this.group.children[i].scale.z = (visualArray[i] / 255) * (avg / 255) + 0.25;
		}
	}

	setTricentricUniformColor(groupI: number, h: number, s: number, l: number, factor: number) {
		View.data.renderer.setClearColor(new THREE.Color('hsl( 0, 0%, 0%)'), 1);
		//l = parseInt( (factor / 255) * 60 + 1 );
		this.group.children[groupI].material.uniforms.col.value = new THREE.Color(
			'hsl(' + h + ', ' + s + '%, ' + l + '%)'
		);
		// this.group.children[groupI].material.uniforms.alpha.value = s / 100;
	}

	arrayAverage(arr: number[]) {
		var sum = 0;
		for (var i = 0; i < arr.length; i++) {
			sum += arr[i];
		}
		return sum / arr.length;
	}
}

export default Tricentric;
