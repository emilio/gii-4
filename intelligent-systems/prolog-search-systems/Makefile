.PHONY: all
all: run-part-1 run-part-2

.PHONY: run-part-1
run-part-1: practica.pdb contacto.pdb
	gprolog --consult-file $< --entry-goal principal --entry-goal halt

.PHONY: run-part-2
run-part-2: practica.pdb contacto.pdb
	gprolog --consult-file $< --entry-goal principal2 --entry-goal halt
