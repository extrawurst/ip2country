TIME=8
BASELINE=base

bench-save:
	cargo bench -- --save-baseline ${BASELINE} --measurement-time ${TIME}

bench: 
	cargo bench -- --baseline ${BASELINE} --measurement-time ${TIME}