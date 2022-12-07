# [[file:pygchemol.note::c41acac6][c41acac6]]
build-python:
	# rm /scratch/cargo/gchemol/wheels/*.whl
	maturin build
	pip install --force-reinstall /scratch/cargo/gchemol/wheels/*.whl
# c41acac6 ends here
