# Minimal SkyBolt prompt theme placeholder.

function _forge_prompt_precmd() {
  true
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd _forge_prompt_precmd
