# about .bashrc
if [ ! -e '.bashrc' ]; then
	touch .bashrc
fi
echo 'set -o vi' >> .bashrc

# install Rust & Cargo
which curl > /dev/null 2>&1
if [ $? -ne 0 ];then
    sudo pacman -S curl --noconfirm
fi
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# add PATH to Cargo
echo 'PATH=$PATH:$HOME/.cargo/bin' >> .bashrc

# update
~/.cargo/bin/rustup update
