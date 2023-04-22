FROM archlinux:latest 
# user for build package
RUN useradd -m builder
# add some packages
RUN pacman -Syu --noconfirm && pacman -S base-devel git sudo --noconfirm
# add user `builder` right to use sudo
RUN echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

USER builder

COPY ./init.sh /home/builder/
