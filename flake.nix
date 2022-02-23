{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        # `nix build`
        packages.haskell-profile-hot-path = naersk-lib.buildPackage {
          pname = "haskell-profile-hot-path";
          root = ./.;
        };
        defaultPackage = packages.haskell-profile-hot-path;

        # `nix run`
        apps.haskell-profile-hot-path = flake-utils.lib.mkApp {
          drv = packages.haskell-profile-hot-path;
        };
        defaultApp = apps.haskell-profile-hot-path;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
        };
      }
    );
}
