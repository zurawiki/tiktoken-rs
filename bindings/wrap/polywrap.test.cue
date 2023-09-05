package e2e

getContextSize: {
	davinci: {
		$0: {
			data: 2049,
			error?: _|_,
		}
	},
	gpt4: {
		$0: {
			data: 8192,
			error?: _|_,
		}
	},
}

p50kBase: {
	$0: {
		data: [...uint8],
		error?: _|_,
	}
	encode: {
		$0: {
			data: [...uint],
			error?: _|_,
		}
	},
	decode: {
		$0: {
			data: "This is a test         with a lot of spaces",
			error?: _|_,
		}
	},
}
