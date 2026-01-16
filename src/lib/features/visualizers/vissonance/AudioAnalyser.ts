import musicStore from '$lib/stores/music.svelte';

interface AnalyserInterface {
	audioContext: AudioContext;
	analyser: AnalyserNode;
	gainNode: GainNode;
	source: AudioBufferSourceNode;
	dataArray: Uint8Array;
}

const AudioAnalyser = {
	data: {} as AnalyserInterface,
	initialize: () => {
		// @ts-ignore
		const audioContext = new (window.AudioContext || window.webkitAudioContext)();
		AudioAnalyser.data.audioContext = audioContext;
		AudioAnalyser.data.analyser = audioContext.createAnalyser();
		AudioAnalyser.data.gainNode = audioContext.createGain();
		AudioAnalyser.data.gainNode.gain.value = 0;
	},
	makeAudio: async function (data: ArrayBuffer) {
		if (AudioAnalyser.data.source) {
			AudioAnalyser.data.source.stop(0);
		}

		AudioAnalyser.data.source = AudioAnalyser.data.audioContext.createBufferSource();
		if (AudioAnalyser.data.audioContext.decodeAudioData) {
			await AudioAnalyser.data.audioContext.decodeAudioData(data, function (buffer) {
				AudioAnalyser.data.source.buffer = buffer;
			});
			await AudioAnalyser.playAudio();
		} else {
			// @ts-ignore
			AudioAnalyser.data.source.buffer = AudioAnalyser.data.audioContext.createBuffer(data, false);
			await AudioAnalyser.playAudio();
		}
	},
	playAudio: async () => {
		AudioAnalyser.data.source.connect(AudioAnalyser.data.analyser);
		AudioAnalyser.data.source.connect(AudioAnalyser.data.gainNode);
		AudioAnalyser.data.gainNode.connect(AudioAnalyser.data.audioContext.destination);

		let duration = musicStore.progressDuration;
		if (duration === null) return;
		duration = duration / 1000;

		AudioAnalyser.data.source.start(0, duration);
	},
	destroy: () => {
		AudioAnalyser.data.source.disconnect(AudioAnalyser.data.analyser);
		AudioAnalyser.data.source.disconnect(AudioAnalyser.data.gainNode);
		AudioAnalyser.data.gainNode.disconnect(AudioAnalyser.data.audioContext.destination);
	}
};

export default AudioAnalyser;
