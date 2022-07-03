# set -x

for f in `find . -path "./.git" -prune -o  -print`; do
    r=`file $f | grep -o executable`
    if [ ${#r} -gt 0 ]; then 
        echo find binary file $f
        rm $f
    fi
done

