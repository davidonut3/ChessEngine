cd .\utilities\rust\
maturin build --release
cd ..
cd ..
pip install target/wheels/rust_utils-0.1.0-cp312-cp312-win_amd64.whl --force-reinstall