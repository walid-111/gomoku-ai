with import <nixpkgs> {};
stdenv.mkDerivation {
	name = "Pente";
	buildInputs = [
		nodejs_22
		wasm-pack
		rustup
		fish

		valgrind
		kdePackages.kcachegrind
		gnuplot
	];
}
