```bash
rm -rf $PREFIX/etc/apt/sources.list.d && echo -n "deb https://packages-cf.termux.dev/apt/termux-main-21 stable main" > $PREFIX/etc/apt/sources.list && apt remove game-repo science-repo
```
```bash
pkg up
```
```bash
pkg install openssh -y && ssh-keygen -t rsa -b 4096 -C "eucalypsih@gmail.com"
```
```bash
cat $HOME/.ssh/id_rsa.pub
```
```bash
ssh-agent $PREFIX/bin/bash
```
```bash
eval "$(ssh-agent -s)"
````
```bash
ssh-add $HOME/.ssh/id_rsa
```
Are you sure you want to continue connecting (yes/no/[fingerprint])?
```bash
ssh -T git@github.com
```
```bash
pkg install fish
```
```bash
rm -rf $PREFIX/etc/bash.bashrc && echo -ne "if [ -x /data/data/com.termux/files/usr/libexec/termux/command-not-found ];then\x0a\x09command_not_found_handle(){\x0a\x09\x09/data/data/com.termux/files/usr/libexec/termux/command-not-found \x22\x241\x22;\x0a\x09}\x0afi\x0aPS1=\x27\x5c[\x5ce[0;32m\x5c]\x5cW\x5c[\x5ce[0m\x5c] \x5c[\x5ce[0;97m\x5c]\x5c\x24\x5c[\x5ce[0m\] \x27\x0aclear\x0afish" > $PREFIX/etc/bash.bashrc
```
```bash
pkg install git
```
```bash
rm -rf eucalypsih_rcraps1 && mkdir eucalypsih_rcraps1 && cd eucalypsih_rcraps1 && git init && git remote add origin git@github.com:eucalypsih/eucalypsih_rcraps1.git && git config --global user.name "eucalypsih" && git config --global user.email "eucalypsih@gmail.com" && git pull origin main --allow-unrelated-histories && git branch -M main
```
```bash
rm -rf .git
```
```bash
git init && git remote add origin git@github.com:eucalypsih/eucalypsih_rcraps1.git && git config --global user.name "eucalypsih" && git config --global user.email "eucalypsih@gmail.com"
```
```bash
git checkout -b main
```
```bash
git add .
```
```bash
git commit -m "init"
```
```bash
git push origin main --force
```
```bash
git add . && git commit -m "up" --amend && git push -u origin main --force-with-lease
```
```text
curl https://api.github.com/repos/nodejs/node 2> /dev/null
```
```bash
rm -rf eucalypsih_rcraps1 && git clone -q git@github.com:eucalypsih/eucalypsih_rcraps1.git && cd eucalypsih_rcraps1 && git config --global user.name "eucalypsih" && git config --global user.email "eucalypsih@gmail.com" && git branch -M main
```
```bash
git clone -q git@github.com:eucalypsih/eucalypsih_rcraps1.git && cd eucalypsih_rcraps1 && git config user.name "eucalypsih" && git config user.email "eucalypsih@gmail.com" && git pull origin main --allow-unrelated-histories
```
```bash
base64 /dev/urandom
```
```bash
```
```bash
echo -ne "include \x22/data/data/com.termux/files/usr/share/nano/*nanorc\x22\x0aset nohelp" > $PREFIX/etc/nanorc
```
CTRL + i	tab<br>
CTRL + s	save<br>
CTRL + x	exit<br>
CTRL + w	search<br>
ALT + w		next<br>
ALT + q		forward<br>
CTRL + q	search[forward]<br>
CTRL+6 CTRL+^ CTRL+a	set mark(select)<br>
CTRL + 7	ctrl+6 + ctrl+7 go line<br>
CTRL + k	cut<br>
CTRL + u	paste<br>
CTRL + j	justified<br>
ALT + n		line numbering<br>
ALT + <br>
