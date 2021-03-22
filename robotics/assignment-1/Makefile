ASSETS := $(wildcard assets/*.png)

.PHONY: build
build: src/Assignment1.cpp CMakeLists.txt
	mkdir -p build
	cd build && cmake .. && make
	rm -rf compile_commands.json && ln -s build/compile_commands.json .

.PHONY: run
run: build ${ASSETS}
	build/Assignment1 assets/reference.png ${ASSETS}
