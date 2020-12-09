.PHONY: all
all: run-part-1-1 run-part-1-2 run-part-2-1

.PHONY: run-part-1-1
run-part-1-1: practica.pdb contacto.pdb
	gprolog --consult-file $< --entry-goal principal --entry-goal halt

.PHONY: run-part-1-2
run-part-1-2: practica.pdb contacto.pdb
	gprolog --consult-file $< --entry-goal principal2 --entry-goal halt

.PHONY: run-part-2-1
run-part-2-1:
	cd part-2 && cargo run -- ../contacto.pdb vidal luis 1
	cd part-2 && cargo run -- ../contacto-con-distancia.pdb vidal luis 1
	cd part-2 && cargo run -- ../contacto-con-distancia.pdb vidal luis 0
