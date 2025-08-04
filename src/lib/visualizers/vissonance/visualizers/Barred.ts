import * as THREE from "three";
import View from "$lib/visualizers/vissonance/View";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import Spectrum from "$lib/visualizers/vissonance/Spectrum";
import Visualizer from "$lib/visualizers/vissonance/visualizers/Visualizer";

class Barred extends Visualizer {
    vertexShader = [
        "void main() {",
        "gl_Position = gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
        "}"
    ].join("\n");
    fragmentShader = [
        "void main() {",
        "gl_FragColor = vec4( gl_FragCoord.y/500.0, 0, gl_FragCoord.y/1000.0, 1.0 );",
        "}"
    ].join("\n");

    async make() {
        View.useOrthographicCamera();

        let positionX = -20 * ( this.numBars / 2 );

        for( let i = 0; i < this.numBars; i++ ) {

            let geometry = new THREE.PlaneGeometry( 18, 5, 1 );
            let uniforms = {};
            let material = new THREE.ShaderMaterial( {
                uniforms: uniforms,
                vertexShader: this.vertexShader,
                fragmentShader: this.fragmentShader
            });
            let plane = new THREE.Mesh( geometry, material );
            plane.position.x = positionX;
            positionX += 20;

            this.group.add( plane );
        }

        View.data.scene.add( this.group );

        await this.waitForLoad();
    }
    destroy() {
        View.data.scene.remove(this.group);
    }
    render() {
        AudioAnalyser.data.analyser.getByteFrequencyData(this.dataArray);
        const visualArray = Spectrum.getVisualBins(
            this.dataArray,
            this.numBars,
            4,
            1300,
        );
        if (!this.group) return;
        for(var i = 0; i < visualArray.length; i++) {
            this.group.children[i].geometry.attributes.position.array[1] = visualArray[i];
            this.group.children[i].geometry.attributes.position.array[4] = visualArray[i];
            this.group.children[i].geometry.attributes.position.needsUpdate = true;
        }
    }
}

export default Barred;
