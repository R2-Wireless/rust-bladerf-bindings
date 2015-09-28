lib:
	rm -rf bladeRF || echo Ok && \
	mkdir bladeRF && \
	(cd bladeRf && \
		git init && \
		git remote add origin https://github.com/Nuand/bladeRF.git && \
		git fetch --depth 1 origin 9fc57e6c7feabde04feebd62cc5b8bb83223728c && \
		git checkout FETCH_HEAD \
	) && \ 
	(cd bladeRf/host && \
		mkdir -p build && \
		cd build && \
		cmake ../ && \
		make && \
		sudo make install && \
		sudo ldconfig \
	)
install:
	cargo build
pre-build-ubuntu:
	apt install llvm-dev libclang-dev clang cmake libusb-dev
pre-build-mac:
	brew install llvm cmake libusb
start:
	cargo run
