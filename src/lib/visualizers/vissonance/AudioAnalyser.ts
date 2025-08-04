interface AnalyserInterface {
    audioContext: AudioContext,
    analyser: AnalyserNode,
    gainNode: GainNode,
    source: AudioBufferSourceNode
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
    makeAudio: function (data: ArrayBuffer){
        if(AudioAnalyser.data.source){
            AudioAnalyser.data.source.stop(0);
        }

        AudioAnalyser.data.source = AudioAnalyser.data.audioContext.createBufferSource();
        if(AudioAnalyser.data.audioContext.decodeAudioData){
            AudioAnalyser.data.audioContext.decodeAudioData(data, function (buffer) {
                AudioAnalyser.data.source.buffer = buffer;
            });
            AudioAnalyser.playAudio();
        } else {
            // @ts-ignore
            AudioAnalyser.data.source.buffer = AudioAnalyser.data.audioContext.createBuffer(data, false);
            AudioAnalyser.playAudio();
        }
    },
    playAudio: () => {
        AudioAnalyser.data.source.connect(AudioAnalyser.data.analyser);
        AudioAnalyser.data.source.connect(AudioAnalyser.data.gainNode);
        AudioAnalyser.data.gainNode.connect(AudioAnalyser.data.audioContext.destination);
        AudioAnalyser.data.source.start(0);
    }
};

export default AudioAnalyser;