import * as THREE from "three";
import View from "$lib/features/visualizers/vissonance/View";
import AudioAnalyser from "$lib/features/visualizers/vissonance/AudioAnalyser";
import Spectrum from "$lib/features/visualizers/vissonance/Spectrum";
import Visualizer from "$lib/features/visualizers/vissonance/visualizers/Visualizer";

class Silk extends Visualizer {
	name = "Silk";
	// @ts-ignore
	group2: THREE.Object3D;
	// @ts-ignore
	group3: THREE.Object3D;
	// @ts-ignore
	group4: THREE.Object3D;
	// @ts-ignore
	bgPlane: THREE.Mesh;

	// @ts-ignore
	geometry: THREE.CircleGeometry;
	// @ts-ignore
	uniforms: { [uniform: string]: IUniform<any> };

	barGap = 0.12;
	vertexShader = [
		"void main() {",
		"gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
		"}",
	].join("\n");
	fragmentShader = [
		"uniform vec3 col;",
		"void main() {",
		"gl_FragColor = vec4( col.r, col.g, col.b, 1.0 );",
		"}",
	].join("\n");
	bgfragmentShader = [
		"void main() {",
		"gl_FragColor = vec4( 0.0, 0.0, 0.0, 1.0 );",
		"}",
	].join("\n");

	async make() {
		await super.make();

		this.numBars = 512;

		View.usePerspectiveCamera();
		View.data.camera.position.x = 0;
		View.data.camera.position.y = 0;
		View.data.camera.position.z = 0;

		while (View.data.container.firstChild) {
			View.data.container.removeChild(View.data.container.firstChild);
		}

		View.data.renderer = new THREE.WebGLRenderer({
			alpha: true,
			preserveDrawingBuffer: true,
		});
		View.data.renderer.setPixelRatio(window.devicePixelRatio);
		View.data.renderer.setSize(window.innerWidth, window.innerHeight);
		View.data.container.appendChild(View.data.renderer.domElement);

		// View.data.renderer.setClearColor( new THREE.Color( 0xfdfdfd ), 1);
		// View.data.renderer.clear();
		View.data.renderer.autoClearColor = false;

		let posX = 3;
		//var spriteMap = new THREE.TextureLoader().load( "resources/glow.png" );
		for (let i = 0; i < this.numBars; i++) {
			this.geometry = new THREE.CircleGeometry(10, 6);
			this.uniforms = {
				col: { type: "c", value: new THREE.Color("hsl(240, 100%, 50%)") },
			};
			const material = new THREE.ShaderMaterial({
				uniforms: this.uniforms,
				vertexShader: this.vertexShader,
				fragmentShader: this.fragmentShader,
				side: THREE.DoubleSide,
			});

			//material = new THREE.MeshBasicMaterial( {wireframe: true} );
			/*
            var spriteMaterial = new THREE.SpriteMaterial( { map: spriteMap, color: 0xffffff } );
            var sprite = new THREE.Sprite( spriteMaterial );
            sprite.position.x = posX;
            sprite.position.y = 0;
            sprite.position.z = -50;

            */
			let circle = new THREE.Mesh(this.geometry, material);
			circle.position.x = posX;
			circle.position.y = 0;
			circle.position.z = -50;
			//circle.rotation.z = Math.PI/6;

			posX += this.barGap;

			const pivot = new THREE.Object3D();
			pivot.add(circle);

			this.group.add(pivot);
		}

		View.data.scene.add(this.group);

		//group2 //top left
		this.group2 = this.group.clone();
		//group2.position.x -= numBars * barGap;
		this.group2.rotation.z = Math.PI;
		View.data.scene.add(this.group2);

		//group3 //bottom left
		this.group3 = this.group.clone();
		//group3.position.x -= numBars * barGap;
		this.group3.rotation.z = Math.PI;
		View.data.scene.add(this.group3);

		//group4 //bottom right
		this.group4 = this.group.clone();
		View.data.scene.add(this.group4);

		//bgPlane
		const geometry = new THREE.PlaneGeometry(2000, 2000, 1, 1);
		const material = new THREE.ShaderMaterial({
			uniforms: this.uniforms,
			vertexShader: this.vertexShader,
			fragmentShader: this.bgfragmentShader,
			transparent: true,
			depthWrite: false,
		});

		//material = new THREE.MeshBasicMaterial( {wireframe: true} );
		/*
        var spriteMaterial = new THREE.SpriteMaterial( { map: spriteMap, color: 0xffffff } );
        var sprite = new THREE.Sprite( spriteMaterial );
        sprite.position.x = posX;
        sprite.position.y = 0;
        sprite.position.z = -50;

        */
		this.bgPlane = new THREE.Mesh(geometry, material);
		this.bgPlane.position.x = 0;
		this.bgPlane.position.y = 0;
		this.bgPlane.position.z = -60;
		View.data.scene.add(this.bgPlane);

		await this.waitForLoad();
	}
	destroy() {
		View.data.scene.remove(this.group);
		View.data.scene.remove(this.group2);
		View.data.scene.remove(this.group3);
		View.data.scene.remove(this.group4);

		View.data.scene.remove(this.bgPlane);

		while (View.data.container.firstChild) {
			View.data.container.removeChild(View.data.container.firstChild);
		}

		View.data.renderer = new THREE.WebGLRenderer({ alpha: true });
		View.data.renderer.setPixelRatio(window.devicePixelRatio);
		View.data.renderer.setSize(window.innerWidth, window.innerHeight);
		View.data.container.appendChild(View.data.renderer.domElement);

		View.data.renderer.autoClearColor = true;
		// View.data.renderer.setClearColor( new THREE.Color( 0xffffff ), 0);
		// View.data.renderer.clear();
	}
	executeOnNewSong() {
		if (!this.group) return;
		for (let i = 0; i < this.group.children.length; i++) {
			this.group.children[i].position.y = 0;
			this.group2.children[i].position.y = 0;
			this.group3.children[i].position.y = 0;
			this.group4.children[i].position.y = 0;
		}
	}
	render() {
		AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
		let visualArray = Spectrum.getVisualBins(
			this.dataArray,
			this.numBars,
			6,
			1300,
		);
		//visualArray.reverse();
		let loudness = this.getLoudness(this.dataArray);
		//smooth loudness
		loudness = (loudness + this.lastLoudness) / 2;
		if (!this.group) return;
		for (let i = 0; i < visualArray.length; i++) {
			this.group.children[i].children[0].scale.x =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);
			this.group.children[i].children[0].scale.y =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group2.children[i].children[0].scale.x =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);
			this.group2.children[i].children[0].scale.y =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group3.children[i].children[0].scale.x =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);
			this.group3.children[i].children[0].scale.y =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group4.children[i].children[0].scale.x =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);
			this.group4.children[i].children[0].scale.y =
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);
			//

			this.group.children[i].position.y +=
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group2.children[i].position.y +=
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group3.children[i].position.y -=
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			this.group4.children[i].position.y -=
				visualArray[i] <= 1
					? 0.0001
					: 4 * Math.log10(1 + visualArray[i] / 255 / 7);

			//
			if (this.group.children[i].position.y > 30 || loudness <= 1) {
				this.group.children[i].position.y = 0;
			}

			if (this.group2.children[i].position.y > 30 || loudness <= 1) {
				this.group2.children[i].position.y = 0;
			}

			if (this.group3.children[i].position.y < -30 || loudness <= 1) {
				this.group3.children[i].position.y = 0;
			}

			if (this.group4.children[i].position.y < -30 || loudness <= 1) {
				this.group4.children[i].position.y = 0;
			}

			this.setUniformColor(i, visualArray[i]);
		}
		this.lastLoudness = loudness;
	}
	setUniformColor(groupI: number, loudness: number) {
		const h = this.modn(250 - loudness * 0.9, 360);
		//Just once since they share materials
		this.group.children[groupI].children[0].material.uniforms.col.value =
			new THREE.Color(
				"hsl(" + h + ", 90%, " + (100 - Math.min(40, loudness)) + "%)",
			);
	}
}

export default Silk;
