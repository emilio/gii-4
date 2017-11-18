% Map representation as a graph.
%
% `conectado` indicates a bi-directional edge from one position to another.
conectado(S,c1,10).

conectado(c1,s1,5).
conectado(c1,s5,5).
conectado(c1,c2,10).

conectado(c2,s2,5).
conectado(c2,s6,5).
conectado(c2,c3,10).

conectado(c3,s3,5).
conectado(c3,s7,5).
conectado(c3,c4,10).

conectado(c4,s4,5).
conectado(c4,s8,5).
conectado(c4,T,10).

ubicacion(patatas,s1,200).
ubicacion(melones,s1,100).
ubicacion(boligrafos,s2,500).
ubicacion(boligrafos,s3,400).
ubicacion(melocotones,s4,200).
ubicacion(berzas,s4,100).
ubicacion(papeles,s5,500).
ubicacion(boligrafos,s6,400).
ubicacion(patatas,s1,200).
ubicacion(melones,s1,100).
ubicacion(plumas,s7,500).
ubicacion(plumas,s8,400).
ubicacion(colonias,s3,150).
ubicacion(ratones,s4,210).
