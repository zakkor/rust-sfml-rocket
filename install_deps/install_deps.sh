#!/bin/sh

### Download, compile, and install SFML-2.3.2.
wget http://www.sfml-dev.org/files/SFML-2.3.2-sources.zip --no-check-certificate
unzip -q SFML-2.3.2-sources.zip

### make -jX : replace X with number of cores you want to use for compilation
### Default: 4
pushd SFML-2.3.2 && mkdir build && cd build && cmake .. && make -j4

### Install SFML to /usr/local/...
sudo make install
popd

### Download, compile, and install CSFML-2.3.
wget http://www.sfml-dev.org/files/CSFML-2.3-sources.zip --no-check-certificate
unzip -q CSFML-2.3-sources.zip
pushd CSFML-2.3
mkdir cmake/Modules

cp /usr/local/share/SFML/cmake/Modules/FindSFML.cmake cmake/Modules

### make -jX : replace X with number of cores you want to use for compilation
### Default: 4

### Replace this path
mkdir build && cd build && cmake -DCMAKE_MODULE_PATH=/absolute/path/to/install_deps/CSFML-2.3/cmake/Modules -DSFML_ROOT=/usr/local .. && make -j4

### Install CSFML to /usr/local/...
sudo make install
popd

echo "All done!"
