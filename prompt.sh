_user_hburger() {
    hburger hash "$USER" -l 1 -c 1 -r 1
}

_cwd_hburgers() {
    if [[ "$PWD" == "$HOME" || "$PWD" == "$HOME/"* ]]; then
        printf "~"
        relpath="${PWD#$HOME}"
    else
        printf " "
        relpath="$PWD"
    fi

    if [ -z "$relpath" ]; then
        printf "/"
        return
    fi

    # Adjust these arguments to your liking
    hburger hash-path "$relpath" \
        --left-bun-length 4 \
        --center-hashpatty-length 2 \
        --right-bun-length 4 \
        --padding-char " " \
        --start-components 2 \
        --end-components 2 \
        --divider ":"
}

# For bash
_bash_host_hburger() {
    hburger hash "$HOSTNAME" -l 1 -c 1 -r 1
}
BASH_USERHOST_COLOR='\[\e[38;5;99m\]'  # Purple
BASH_CWD_COLOR='\[\e[38;5;220m\]'   # Yellow
BASH_RESET_COLOR='\[\e[0m\]'
PS1="$BASH_USERHOST_COLOR"'$(_user_hburger)@$(_bash_host_hburger)'"$BASH_CWD_COLOR"'[$(_cwd_hburgers)]'"$BASH_RESET_COLOR"'\$ '

# For zsh
_zsh_host_hburger() {
    hburger hash "$HOST" -l 1 -c 1 -r 1
}
ZSH_USERHOST_COLOR='%F{099}'  # Purple
ZSH_CWD_COLOR='%F{220}'  # Yellow
ZSH_RESET_COLOR='%f'
PROMPT="$ZSH_USERHOST_COLOR"'$(_user_hburger)@$(_zsh_host_hburger)'"$ZSH_CWD_COLOR"'[$(_cwd_hburgers)]'"$ZSH_RESET_COLOR"'%# '
