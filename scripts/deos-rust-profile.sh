# source cargo profile if it exists and isn't already there
if [[ -f "$HOME/.cargo/env" ]] &&  [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]
then
  . ~/.cargo/env
fi

# export desk to path
if [[ ":$PATH:" != *":/desk/bin:"* ]]
then
  PATH="/desk/bin:$PATH"
fi
