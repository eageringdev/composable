{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      rustSrc = pkgs.lib.cleanSourceWith {
        filter = pkgs.lib.cleanSourceFilter;
        src = pkgs.lib.cleanSourceWith {
          filter = let
            isProto = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
            isJSON = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
            isREADME = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix "README.md" name;
            isDir = name: type: type == "directory";
            isCargo = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
              || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
            isRust = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".rs" name;
            customFilter = name: type:
              builtins.any (fun: fun name type) [
                isCargo
                isRust
                isDir
                isREADME
                isJSON
                isProto
              ];
          in pkgs.nix-gitignore.gitignoreFilterPure customFilter
          [ ../.gitignore ] ./.;
          src = ./.;
        };
      };
      # Build a wasm runtime, unoptimized
      mkRuntime = name: features: cargoArtifacts:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          pname = "${name}-runtime";
          src = rustSrc;
          inherit cargoArtifacts;
          cargoBuildCommand =
            "cargo build --release --package ${name}-runtime-wasm --target wasm32-unknown-unknown"
            + pkgs.lib.strings.optionalString (features != "")
            (" --features=${features}");
          # From parity/wasm-builder
          RUSTFLAGS =
            "-C target-cpu=mvp -C target-feature=-sign-ext -C link-arg=--export-table -Clink-arg=--export=__heap_base -C link-arg=--import-memory";
        });

      # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
      mkOptimizedRuntime = { name, features ? ""
        , common-deps ? self'.packages.common-deps-nightly }:
        let runtime = mkRuntime name features common-deps;
        in pkgs.stdenv.mkDerivation {
          name = "${runtime.name}-optimized";
          phases = [ "installPhase" ];
          nativeBuildInputs = [ pkgs.binaryen ];
          installPhase = ''
            mkdir --parents $out/lib
            # https://github.com/paritytech/substrate/blob/30cb4d10b3118d1b3aa5b2ae7fa8429b2c4f28de/utils/wasm-builder/src/wasm_project.rs#L694
            wasm-opt ${runtime}/lib/${name}_runtime.wasm -o $out/lib/runtime.optimized.wasm -Os --strip-dwarf --debuginfo --mvp-features            
            ${self'.packages.subwasm}/bin/subwasm compress $out/lib/runtime.optimized.wasm $out/lib/runtime.optimized.wasm
          '';
        };

    in {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        picasso-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "";
        };
        picasso-testfast-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "testnet,fastnet";
        };
        composable-testfast-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "testnet,fastnet";
        };
        composable-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "";
        };
        picasso-bench-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "runtime-benchmarks";
          common-deps = self'.packages.common-wasm-bench-deps;
        };
        composable-bench-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "runtime-benchmarks";
          common-deps = self'.packages.common-wasm-bench-deps;
        };
      };

    };
}
