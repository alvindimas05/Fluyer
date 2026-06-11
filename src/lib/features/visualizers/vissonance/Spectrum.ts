const spectrumMaxExponent = 5;
const spectrumMinExponent = 3;
const spectrumHeight = 255;

const Spectrum = {
	getVisualBins: (
		dataArray: Uint8Array,
		numElements: number,
		SpectrumStart: number,
		SpectrumEnd: number
	) => {
		const SpectrumBarCount = numElements;
		const SamplePoints = [];
		const NewArray = [];
		let LastSpot = 0;
		for (let i = 0; i < SpectrumBarCount; i++) {
			let Bin = Math.round(
				Spectrum.ease(i / SpectrumBarCount) * (SpectrumEnd - SpectrumStart) + SpectrumStart
			);
			if (Bin <= LastSpot) {
				Bin = LastSpot + 1;
			}
			LastSpot = Bin;
			SamplePoints[i] = Bin;
		}

		const MaxSamplePoints = [];
		for (let i = 0; i < SpectrumBarCount; i++) {
			const CurSpot = SamplePoints[i];
			let NextSpot = SamplePoints[i + 1];
			if (NextSpot == null) {
				NextSpot = SpectrumEnd;
			}

			let CurMax = dataArray[CurSpot];
			let MaxSpot = CurSpot;
			const Dif = NextSpot - CurSpot;
			for (let j = 1; j < Dif; j++) {
				const NewSpot = CurSpot + j;
				if (dataArray[NewSpot] > CurMax) {
					CurMax = dataArray[NewSpot];
					MaxSpot = NewSpot;
				}
			}
			MaxSamplePoints[i] = MaxSpot;
		}

		for (let i = 0; i < SpectrumBarCount; i++) {
			const NextMaxSpot = MaxSamplePoints[i];
			let LastMaxSpot = MaxSamplePoints[i - 1];
			if (LastMaxSpot == null) {
				LastMaxSpot = SpectrumStart;
			}
			const LastMax = dataArray[LastMaxSpot];
			const NextMax = dataArray[NextMaxSpot];

			NewArray[i] = (LastMax + NextMax) / 2;
			if (isNaN(NewArray[i])) {
				NewArray[i] = 0;
			}
		}
		return Spectrum.exponentialTransform(NewArray);
	},
	ease: (value: number) => Math.pow(value, 2.55),
	exponentialTransform: (array: number[]) => {
		const newArr = [];
		for (let i = 0; i < array.length; i++) {
			const exp =
				spectrumMaxExponent + (spectrumMinExponent - spectrumMaxExponent) * (i / array.length);
			newArr[i] = Math.max(Math.pow(array[i] / spectrumHeight, exp) * spectrumHeight, 1);
		}
		return newArr;
	}
};

export default Spectrum;
