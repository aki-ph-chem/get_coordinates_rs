
src="coordinates_picker"

if [ ! -e "$src" ];then
    cp -r "../${src}" .
fi

cd ${src}
cargo deb
