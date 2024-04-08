with import ./nix/pkgs.nix {};
let merged-openssl = symlinkJoin { name = "merged-openssl"; paths = [ openssl.out openssl.dev ]; };
in stdenv.mkDerivation rec {
  name = "barren_plateau";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    openssl
    cmake
    wasm-pack
    wasm-bindgen-cli
  ];
  shellHook = ''
  export OPENSSL_DIR="${merged-openssl}"
  '';
}
