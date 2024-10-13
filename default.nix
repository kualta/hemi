with import <nixpkgs> {};
mkShell {
  buildInputs = [
    libiconv
    openssl
    pkg-config
    dioxus-cli
  ];
}
