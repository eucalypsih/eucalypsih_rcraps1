```bash
termux-reload-settings
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
git checkout -b main # Switched to a new branch 'main'
```
```bash
git -C $HOME/AndroidCICD add .
````
```bash
git add .
```
```bash
git status
```
```bash
git add .github/workflows/AndroidBuild.yml
```
```bash
git reset
```
```bash
git diff --cached
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

rename url
```bash
git remote set-url origin git@github.com:eucalypsih/eucalypsih_rcraps1.git
```
```bash
git remote remove origin
```

partial clone
> --filter=blob:none tidak download isi file (blob)<br>
> --no-checkout tidak membuat working directory<br>
> Hanya metadata commit & tree yang diambil<br>
`git clone --filter=blob:none --no-checkout https://github.com/ohmyzsh/ohmyzsh.git` # Clone repositori tanpa checkout
```bash
git clone --filter=blob:none --no-checkout https://github.com/ohmyzsh/ohmyzsh.git # Clone repositori tanpa checkout
```

git 2.25
```bash
cd ohmyzsh
git sparse-checkout init --cone # Inisialisasi sparse-checkout
git sparse-checkout set plugins/hitokoto   # Tentukan folder atau file tertentu yang ingin Anda checkout
git checkout
```

```bash
git config core.sparseCheckout true # Inisialisasi sparse-checkout
echo "plugins/hitokoto/*" >> .git/info/sparse-checkout   # Tentukan folder atau file tertentu yang ingin Anda checkout
git checkout master
```

`git checkout -b master` # membuat baru tanpa mendownload
`git checkout master`    # sekaligus mendownload
`git switch -c master`    # buat + pindah
`git switch master`        # pindah saja

`git fetch --tags`          # Ambil semua tag
`git checkout tags/v7.0.0`   # Checkout versi tertentu (misalnya, v7.0.0)
`git describe --tags`    # Verifikasi Versi

<span>Ukuran</span>
```bash
du -sh ohmyzsh
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
