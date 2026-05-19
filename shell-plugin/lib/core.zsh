# Minimal embedded SkyBolt shell integration.

function _forge_osc133_emit() {
  print -n "\e]133;$1\a"
}

function _forge_reset() {
  true
}

function _forge_apply_keybindings() {
  true
}

typeset -ga zvm_after_init_commands
zvm_after_init_commands+=('_forge_apply_keybindings')

function _forge_zle_wrapper() {
  local user_action="$1"
    _forge_osc133_emit "B"
    _forge_osc133_emit "C"
    case "$user_action" in
    *) ;;
  esac
    local action_status=$?
    _forge_osc133_emit "D;$action_status"
    _forge_osc133_emit "A"
    _forge_reset
  return $action_status
}
