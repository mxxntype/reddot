{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs
  }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = (import nixpkgs) {
        inherit system;
      };
      naersk' = pkgs.callPackage naersk {};
    in {
      # For `nix build` & `nix run`:
      packages.default = naersk'.buildPackage {
        src = builtins.path {
          name = "reddot";
          path = ./.;
        };
      };

      # For `nix develop`:
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    }
  );
}
