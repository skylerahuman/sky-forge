# SkyBolt shell setup. Legacy FORGE_* variables are still honored by the Rust code.

if command -v sky >/dev/null 2>&1; then
  source <(sky zsh plugin)
fi
