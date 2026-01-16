import * as THREE from 'three';

interface ViewDataInterface {
	camera: THREE.OrthographicCamera | THREE.PerspectiveCamera;
	renderer: THREE.WebGLRenderer;
	scene: THREE.Scene;
	container: HTMLElement;
	renderVisualization: (() => void) | null;
	loopAnimateIntervalId: ReturnType<typeof setInterval>;
}

const View = {
	data: {} as ViewDataInterface,
	initialize: (container: HTMLElement) => {
		View.data.container = container;
		View.data.container.style.width = '100%';
		View.data.container.style.height = '100%';
		View.data.camera = new THREE.OrthographicCamera(
			window.innerWidth / -2,
			window.innerWidth / 2,
			window.innerHeight / 2,
			window.innerHeight / -2,
			1,
			1000
		);
		View.data.camera.position.y = 150;
		View.data.camera.position.z = 500;
		View.data.scene = new THREE.Scene();

		View.resetRenderer();

		View.loopAnimate();
	},
	resetRenderer: () => {
		// Plane
		View.data.renderer = new THREE.WebGLRenderer({
			alpha: true,
			preserveDrawingBuffer: true
		});
		View.data.renderer.setPixelRatio(window.devicePixelRatio);
		View.data.renderer.setSize(window.innerWidth, window.innerHeight);
		View.data.container.appendChild(View.data.renderer.domElement);
	},
	usePerspectiveCamera: function () {
		View.data.camera = new THREE.PerspectiveCamera(
			70,
			window.innerWidth / window.innerHeight,
			0.01,
			2000
		);
		View.data.camera.position.y = 150;
		View.data.camera.position.z = 500;
	},
	useOrthographicCamera: function () {
		View.data.camera = new THREE.OrthographicCamera(
			window.innerWidth / -2,
			window.innerWidth / 2,
			window.innerHeight / 2,
			window.innerHeight / -2,
			1,
			1000
		);
		View.data.camera.position.y = 150;
		View.data.camera.position.z = 500;
	},
	onWindowResize: () => {
		if (View.data.camera instanceof THREE.PerspectiveCamera)
			View.data.camera.aspect = window.innerWidth / window.innerHeight;
		View.data.camera.updateProjectionMatrix();
		View.data.renderer.setSize(window.innerWidth, window.innerHeight);
	},
	loopAnimate: () => {
		View.animate();
		View.data.loopAnimateIntervalId = setInterval(View.animate, 1000 / 60);
	},
	animate: () => {
		if (View.data.renderVisualization) View.data.renderVisualization();
		View.data.renderer.render(View.data.scene, View.data.camera);
	},
	destroy: () => {
		clearInterval(View.data.loopAnimateIntervalId);
		View.data.renderer.dispose();
		View.data.renderer.forceContextLoss();
	},
	clear: () => {
		View.data.scene = new THREE.Scene();
		View.data.renderer.clear();
	}
};

export default View;
