import * as THREE from "three";
import type {Visualizer} from "$lib/visualizers/vissonance/visualizers/types";
import View from "$lib/visualizers/vissonance/View";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import Spectrum from "$lib/visualizers/vissonance/Spectrum";

let dataArray: Uint8Array;
const numBars = 128;
const vertexShader = [
    "varying vec4 pos;",
    "void main() {",
    "pos = modelViewMatrix * vec4( position, 1.0 );",
    "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );",
    "}"
].join('\n');
const fragmentShader = [
    "uniform vec3 col;",
    "varying vec4 pos;",
    "void main() {",
    "gl_FragColor = vec4( -pos.z/180.0 * col.r, -pos.z/180.0 * col.g, -pos.z/180.0 * col.b, 1.0 );",
    "}"
].join('\n');

let group: THREE.Object3D;

const Iris: Visualizer = {
    initialize: () => {},
    make: () => {
        group = new THREE.Object3D();
        const bufferLength = AudioAnalyser.data.analyser.frequencyBinCount;
        dataArray = new Uint8Array(bufferLength);

        View.usePerspectiveCamera();
        View.data.camera.position.y = 0;
        View.data.camera.position.z = 250;

        for(let i = 0; i < numBars / 2; i++) {
            const uniforms = {
                col: { type: 'c', value: new THREE.Color( 'hsl(240, 100%, 50%)' ) },
            };
            const material = new THREE.ShaderMaterial({
                uniforms: uniforms,
                vertexShader: vertexShader,
                fragmentShader: fragmentShader
            });

            let geometry = new THREE.PlaneGeometry(3, 500, 1);
            geometry.rotateX(Math.PI / 1.8);
            geometry.applyMatrix4(new THREE.Matrix4().makeTranslation( 0, 60, 0));
            let plane = new THREE.Mesh(geometry, material);

            plane.rotation.z = i * (Math.PI * 2 / numBars) + (Math.PI / numBars);

            group.add(plane);

            geometry = new THREE.PlaneGeometry(3, 500, 1);
            geometry.rotateX(Math.PI / 1.8);
            geometry.applyMatrix4(new THREE.Matrix4().makeTranslation(0, 60, 0));
            plane = new THREE.Mesh(geometry, material);

            plane.rotation.z = -i * ( Math.PI * 2 / numBars ) - ( Math.PI / numBars );

            group.add(plane);
        }
        View.data.scene.add(group);
    },
    destroy: () => View.data.scene.remove(group),
    render: () => {
        AudioAnalyser.data.analyser.getByteFrequencyData(dataArray);
        var loudness = getLoudness(dataArray);
        const visualArray = Spectrum.getVisualBins(dataArray, numBars, 4, 1300);
        if(!group) return;
        for(let i = 0; i < visualArray.length / 2; i++) {
            // Left and right share the same material hence why we don't need i * 2 + 1
            setUniformColor(i * 2, loudness);

            group.children[i * 2].geometry.attributes.position.array[7] = visualArray[i] / 2 + ( 65 + (loudness/1.5) );
            group.children[i * 2].geometry.attributes.position.array[10] = visualArray[i] / 2 + ( 65 + (loudness/1.5) );
            group.children[i * 2].geometry.attributes.position.needsUpdate = true;

            group.children[i * 2 + 1].geometry.attributes.position.array[7] = visualArray[i] / 2 + ( 65 + (loudness/1.5) );
            group.children[i * 2 + 1].geometry.attributes.position.array[10] = visualArray[i] / 2 + ( 65 + (loudness/1.5) );
            group.children[i * 2 + 1].geometry.attributes.position.needsUpdate = true;
        }
    },
}

function setUniformColor(groupI: number, loudness: number) {
    let h = modn(250 - (loudness * 2.2), 360);
    group.children[groupI].material.uniforms.col.value = new THREE.Color(`hsl(${h}, 100%, 50%)`);
}


function getLoudness(arr: Uint8Array) {
    let sum = 0;
    for(let i = 0; i < arr.length; i++) {
        sum += arr[i];
    }
    return sum / arr.length;
}

function modn(n: number, m: number) {
    return ((n % m) + m) % m;
}

export default Iris;