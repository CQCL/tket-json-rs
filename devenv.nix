{ pkgs, lib, config, inputs, ... }:

{
  languages.rust = {
    enable = true;
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  languages.python = {
    enable = true;
    uv = {
      enable = true;
    };
  };
}
