# ubuntu
# cat main.rs | xsel --clipboard --input

# WSL
cat $1 | iconv -t sjis | clip.exe
