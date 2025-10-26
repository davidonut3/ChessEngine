cd .\engine\
maturin build --release
pip install target\wheels\engine-0.1.0-cp312-cp312-win_amd64.whl --force-reinstall
cd ..