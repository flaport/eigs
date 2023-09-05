rs:
	cd ../eigs-pyo3 && cargo build --release && cp target/release/libeigs_pyo3.so ../eigs-py/eigs_pyo3.so

dist: (rs)
	python -m build --sdist --bdist_wheel

clean:
	rm -rf *.egg-info
	find . -name "dist" | xargs rm -rf
	find . -name "build" | xargs rm -rf
	find . -name "builds" | xargs rm -rf
	find . -name "__pycache__" | xargs rm -rf
	find . -name "*.so" | xargs rm -rf
	find . -name ".ipynb_checkpoints" | xargs rm -rf
	find . -name ".pytest_cache" | xargs rm -rf
	find . -name ".mypy_cache" | xargs rm -rf
