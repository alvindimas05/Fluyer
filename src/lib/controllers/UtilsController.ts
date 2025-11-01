const UtilsController = {
	withBase64: (value: string) => {
		return `data:image/png;base64,${value}`;
	},
};

export default UtilsController;
